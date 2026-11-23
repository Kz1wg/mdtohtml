mod highlight;
mod mycss;

use clap::Parser;
use maud::{html, PreEscaped, DOCTYPE};
use pulldown_cmark::{self, html::push_html, CodeBlockKind, CowStr, Event, Tag, TagEnd};
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(author, version, about = "Markdown を見やすい単体 HTML に変換するツール", long_about = None)]
struct Cli {
    /// 変換対象のMarkdownファイルパス（複数指定可）
    #[arg(required = true)]
    filename: Vec<PathBuf>,

    /// ローカル画像を base64 で HTML に埋め込み、HTML 1ファイルで共有できるようにする
    #[arg(short, long)]
    embed_images: bool,

    /// ファイルの変更を監視し、保存のたびに自動で再変換する（Ctrl+C で終了）
    #[arg(short, long)]
    watch: bool,

    /// 本文の最大幅。数値のみなら px として扱う（例: 1000, 1000px, 90%, 60em）
    #[arg(long, default_value = "800px", value_parser = parse_width)]
    width: String,

    /// テーブルが本文幅の何倍まで広がれるか（1.0〜3.0）。超える分はテーブル内で横スクロール
    #[arg(long, default_value_t = 1.2, value_parser = parse_table_scale)]
    table_scale: f64,

    /// Markdown ファイル(.md/.markdown/.mdown)へのリンクを .html に書き換える（外部 URL は対象外、リンク先が無ければ警告）
    #[arg(short = 'l', long)]
    md_links: bool,

    /// リンク先の Markdown も辿って一括変換する（-l を含む）。辿るのは指定ファイルのフォルダ以下のみ
    #[arg(short, long)]
    recursive: bool,
}

/// 変換時のオプション
struct ConvertOptions {
    embed_images: bool,
    width: String,
    table_scale: f64,
    md_links: bool,
}

/// --table-scale の値を検証する
fn parse_table_scale(s: &str) -> Result<f64, String> {
    match s.trim().parse::<f64>() {
        Ok(v) if (1.0..=3.0).contains(&v) => Ok(v),
        _ => Err(format!("1.0〜3.0 の数値を指定してください: {s}")),
    }
}

/// --width の値を検証し、CSS で使える長さに正規化する
fn parse_width(s: &str) -> Result<String, String> {
    let s = s.trim();
    if !s.is_empty() && s.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return Ok(format!("{s}px"));
    }
    let num_end = s
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(s.len());
    let unit = &s[num_end..];
    if num_end > 0 && matches!(unit, "px" | "%" | "em" | "rem" | "vw" | "ch") {
        Ok(s.to_string())
    } else {
        Err(format!(
            "不正な幅指定です: {s}（例: 1000, 1000px, 90%, 60em）"
        ))
    }
}

fn main() -> MyResult<()> {
    let cli = Cli::parse();
    let opts = ConvertOptions {
        embed_images: cli.embed_images,
        width: cli.width,
        table_scale: cli.table_scale,
        // リンク先を一括変換するならリンクの書き換えも必須
        md_links: cli.md_links || cli.recursive,
    };

    let mut conv = Converter::new(&opts, cli.recursive, &cli.filename);
    let count = conv.process(&cli.filename);
    if cli.recursive {
        println!("{count} ファイル変換しました");
    }

    if cli.watch {
        conv.watch();
    }
    Ok(())
}

/// 1ファイルを変換し、文書内のローカル Markdown リンク先（存在するもの）を返す
fn convert_one(target: &Path, opts: &ConvertOptions) -> Vec<PathBuf> {
    if !target.is_file() {
        eprintln!("スキップ: {} はファイルではありません", target.display());
        return Vec::new();
    }
    match target.extension().and_then(|e| e.to_str()) {
        Some("md" | "markdown" | "mdown") => match convert_md(target, opts) {
            Ok(links) => links,
            Err(e) => {
                eprintln!("変換エラー [{}]: {}", target.display(), e);
                Vec::new()
            }
        },
        _ => {
            eprintln!(
                "{} は無効です。md / markdown / mdown の拡張子を持つファイルを指定してください",
                target.display()
            );
            Vec::new()
        }
    }
}

// ── 変換キュー（--recursive）と監視（--watch） ─────────────────────────────────

/// --recursive で辿るファイル数の上限（暴走防止の最終安全弁）
const MAX_RECURSIVE_FILES: usize = 30;

/// 変換対象の管理。暴走防止のため次の3つを守る:
/// 1. 同じファイル（正規化した絶対パスで判定）は1回の処理で1度しか変換しない
/// 2. 辿るのは最初に指定したファイルのフォルダ以下だけ
/// 3. 辿って追加するファイルは全体で MAX_RECURSIVE_FILES 個まで
struct Converter<'a> {
    opts: &'a ConvertOptions,
    recursive: bool,
    /// 辿ってよい範囲（最初に指定したファイルのフォルダ）
    roots: Vec<PathBuf>,
    /// 変換対象として登録済みのファイル（正規化済み）
    known: HashSet<PathBuf>,
    /// 監視対象（登録順）
    files: Vec<PathBuf>,
    /// 範囲外・上限超過の通知を繰り返さないための記録
    notified: HashSet<PathBuf>,
}

impl<'a> Converter<'a> {
    fn new(opts: &'a ConvertOptions, recursive: bool, initial: &[PathBuf]) -> Self {
        let roots = initial
            .iter()
            .filter_map(|p| fs::canonicalize(p).ok())
            .filter_map(|p| p.parent().map(Path::to_path_buf))
            .collect();
        Self {
            opts,
            recursive,
            roots,
            known: HashSet::new(),
            files: Vec::new(),
            notified: HashSet::new(),
        }
    }

    /// starts を変換し、--recursive ならリンク先の未変換ファイルも辿って変換する。
    /// 変換したファイル数を返す
    fn process(&mut self, starts: &[PathBuf]) -> usize {
        let mut queue: VecDeque<PathBuf> = VecDeque::new();
        let mut queued_now: HashSet<PathBuf> = HashSet::new();
        for s in starts {
            let key = fs::canonicalize(s).unwrap_or_else(|_| s.clone());
            if !queued_now.insert(key.clone()) {
                continue; // 同じファイルの重複指定
            }
            if self.known.insert(key) {
                self.files.push(s.clone());
            }
            queue.push_back(s.clone());
        }

        let mut count = 0;
        while let Some(path) = queue.pop_front() {
            let links = convert_one(&path, self.opts);
            count += 1;
            if !self.recursive {
                continue;
            }
            for link in links {
                let Ok(c) = fs::canonicalize(&link) else { continue };
                if self.known.contains(&c) {
                    continue; // 変換済み・登録済み（循環リンクもここで止まる）
                }
                if !self.roots.iter().any(|r| c.starts_with(r)) {
                    if self.notified.insert(c.clone()) {
                        eprintln!(
                            "範囲外のため変換しません（リンクは .html に書き換え済み）: {}",
                            c.display()
                        );
                    }
                    continue;
                }
                if self.known.len() >= MAX_RECURSIVE_FILES {
                    if self.notified.insert(c.clone()) {
                        eprintln!(
                            "上限 {MAX_RECURSIVE_FILES} ファイルに達したため変換しません: {}",
                            c.display()
                        );
                    }
                    continue;
                }
                self.known.insert(c.clone());
                self.files.push(c.clone());
                queue.push_back(c);
            }
        }
        count
    }

    /// 更新日時をポーリングし、変更されたファイルだけ再変換する。
    /// --recursive 時は新たに増えたリンク先も変換して監視対象に加える
    fn watch(&mut self) {
        println!("変更を監視しています… (Ctrl+C で終了)");
        let mut last: HashMap<PathBuf, Option<SystemTime>> = HashMap::new();
        loop {
            // 新しく加わったファイルの現在の更新日時を記録（変更扱いにしない）
            for f in &self.files {
                last.entry(f.clone()).or_insert_with(|| modified_time(f));
            }
            std::thread::sleep(Duration::from_millis(500));

            let changed: Vec<PathBuf> = self
                .files
                .iter()
                .filter(|f| {
                    let now = modified_time(f);
                    now.is_some() && last.get(*f) != Some(&now)
                })
                .cloned()
                .collect();
            for f in changed {
                last.insert(f.clone(), modified_time(&f));
                self.process(std::slice::from_ref(&f));
            }
        }
    }
}

fn modified_time(path: &Path) -> Option<SystemTime> {
    fs::metadata(path).and_then(|m| m.modified()).ok()
}

// ── Markdown レンダラー ──────────────────────────────────────────────────────

/// Markdown を HTML に変換する。戻り値は (HTML, 数式を含むか, 存在するローカル Markdown リンク先)
fn render_markdown(md: &str, base_dir: &Path, opts: &ConvertOptions) -> (String, bool, Vec<PathBuf>) {
    let mut popts = pulldown_cmark::Options::empty();
    popts.insert(pulldown_cmark::Options::ENABLE_TABLES);
    popts.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
    popts.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    popts.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    popts.insert(pulldown_cmark::Options::ENABLE_MATH);
    popts.insert(pulldown_cmark::Options::ENABLE_GFM);

    let raw_events: Vec<Event> = pulldown_cmark::Parser::new_ext(md, popts).collect();
    let has_math = raw_events
        .iter()
        .any(|e| matches!(e, Event::InlineMath(_) | Event::DisplayMath(_)));

    let mut events = transform_events(raw_events);
    if opts.embed_images {
        embed_images(&mut events, base_dir);
    }
    let links = if opts.md_links {
        rewrite_md_links(&mut events, base_dir)
    } else {
        Vec::new()
    };

    let mut html_out = String::new();
    push_html(&mut html_out, events.into_iter());
    (html_out, has_math, links)
}

/// コードブロックをハイライト済み HTML に置き換え、テーブルをラッパーで囲む
fn transform_events(events: Vec<Event>) -> Vec<Event> {
    let mut out: Vec<Event> = Vec::with_capacity(events.len());
    let mut iter = events.into_iter();

    while let Some(ev) = iter.next() {
        match ev {
            // コードブロック: 中のテキストを集めて syntect でハイライト
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.into_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                let mut code = String::new();
                for inner in iter.by_ref() {
                    match inner {
                        Event::Text(t) => code.push_str(&t),
                        Event::End(TagEnd::CodeBlock) => break,
                        _ => {}
                    }
                }
                out.push(Event::Html(highlight::highlight_code(&code, &lang).into()));
            }
            // テーブル: ラッパーで囲み、短い列は折り返し禁止にする
            Event::Start(Tag::Table(aligns)) => {
                let mut body = Vec::new();
                for inner in iter.by_ref() {
                    if matches!(inner, Event::End(TagEnd::Table)) {
                        break;
                    }
                    body.push(inner);
                }
                out.push(Event::Html("<div class=\"table-wrap\">".into()));
                out.push(Event::Start(Tag::Table(aligns.clone())));
                out.extend(rewrite_table_cells(body, &aligns));
                out.push(Event::End(TagEnd::Table));
                out.push(Event::Html("</div>".into()));
            }
            _ => out.push(ev),
        }
    }
    out
}

/// 全セルがこの幅（半角換算）以下の列は折り返さない
const NOWRAP_MAX_WIDTH: usize = 20;

/// 表示幅の概算（ASCII は 1、それ以外は 2）
fn display_width(s: &str) -> usize {
    s.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum()
}

/// テーブル内のセルを自前の <th>/<td> に置き換え、短い列に class="nowrap" を付ける
fn rewrite_table_cells<'a>(
    body: Vec<Event<'a>>,
    aligns: &[pulldown_cmark::Alignment],
) -> Vec<Event<'a>> {
    // 1パス目: 列ごとの最大表示幅を求める
    let mut widths = vec![0usize; aligns.len()];
    let (mut col, mut cur) = (0usize, 0usize);
    for ev in &body {
        match ev {
            Event::Start(Tag::TableHead | Tag::TableRow) => col = 0,
            Event::Start(Tag::TableCell) => cur = 0,
            Event::Text(t) | Event::Code(t) => cur += display_width(t),
            Event::End(TagEnd::TableCell) => {
                if let Some(w) = widths.get_mut(col) {
                    *w = (*w).max(cur);
                }
                col += 1;
            }
            _ => {}
        }
    }

    // 2パス目: セルの開始・終了タグを差し替える
    let mut out = Vec::with_capacity(body.len());
    let (mut col, mut in_head) = (0usize, false);
    for ev in body {
        match ev {
            Event::Start(Tag::TableHead) => {
                in_head = true;
                col = 0;
                out.push(ev);
            }
            Event::End(TagEnd::TableHead) => {
                in_head = false;
                out.push(ev);
            }
            Event::Start(Tag::TableRow) => {
                col = 0;
                out.push(ev);
            }
            Event::Start(Tag::TableCell) => {
                let tag = if in_head { "th" } else { "td" };
                let align = match aligns.get(col) {
                    Some(pulldown_cmark::Alignment::Left) => " style=\"text-align: left\"",
                    Some(pulldown_cmark::Alignment::Center) => " style=\"text-align: center\"",
                    Some(pulldown_cmark::Alignment::Right) => " style=\"text-align: right\"",
                    _ => "",
                };
                let nowrap = match widths.get(col) {
                    Some(&w) if w <= NOWRAP_MAX_WIDTH => " class=\"nowrap\"",
                    _ => "",
                };
                out.push(Event::Html(format!("<{tag}{nowrap}{align}>").into()));
            }
            Event::End(TagEnd::TableCell) => {
                let tag = if in_head { "th" } else { "td" };
                out.push(Event::Html(format!("</{tag}>").into()));
                col += 1;
            }
            _ => out.push(ev),
        }
    }
    out
}

// ── Markdown リンクの書き換え ────────────────────────────────────────────────

/// Markdown ファイルへのリンクを .html へのリンクに書き換え、存在するリンク先のパスを返す
fn rewrite_md_links(events: &mut [Event], base_dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for ev in events.iter_mut() {
        if let Event::Start(Tag::Link { dest_url, .. }) = ev {
            if let Some((new_url, md_path)) = md_link_to_html(dest_url) {
                let local = base_dir.join(md_path.replace("%20", " "));
                if !local.is_file() {
                    eprintln!(
                        "警告: リンク先の Markdown が見つかりません: {} (リンク: {})",
                        local.display(),
                        dest_url
                    );
                } else {
                    found.push(local);
                }
                *dest_url = CowStr::from(new_url);
            }
        }
    }
    found
}

/// リンク先が Markdown ファイルなら (書き換え後の URL, パス部分) を返す。
/// 外部 URL・mailto・ページ内アンカー・他の拡張子は None
fn md_link_to_html(url: &str) -> Option<(String, String)> {
    if url.is_empty() || url.starts_with('#') || has_url_scheme(url) {
        return None;
    }
    // パス部分と、? や # 以降（クエリ・アンカー）を分ける
    let split_at = url.find(['?', '#']).unwrap_or(url.len());
    let (path, suffix) = url.split_at(split_at);

    let dot = path.rfind('.')?;
    if path[dot..].contains('/') {
        return None; // ディレクトリ名中のドットは拡張子ではない
    }
    let ext = path[dot + 1..].to_ascii_lowercase();
    if !matches!(ext.as_str(), "md" | "markdown" | "mdown") {
        return None;
    }
    Some((format!("{}.html{}", &path[..dot], suffix), path.to_string()))
}

/// "https:" や "mailto:" のような URL スキームで始まるか
fn has_url_scheme(url: &str) -> bool {
    match url.find(':') {
        Some(i) if i > 0 => url[..i]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.')),
        _ => false,
    }
}

// ── 画像埋め込み ─────────────────────────────────────────────────────────────

/// ローカル画像のパスを data URI に書き換える（読めない画像はそのまま残す）
fn embed_images(events: &mut [Event], base_dir: &Path) {
    for ev in events.iter_mut() {
        if let Event::Start(Tag::Image { dest_url, .. }) = ev {
            if let Some(data_uri) = image_to_data_uri(dest_url, base_dir) {
                *dest_url = CowStr::from(data_uri);
            }
        }
    }
}

fn image_to_data_uri(url: &str, base_dir: &Path) -> Option<String> {
    if url.contains("://") || url.starts_with("data:") {
        return None; // リモート画像・埋め込み済みは対象外
    }
    let mime = match Path::new(url)
        .extension()?
        .to_str()?
        .to_ascii_lowercase()
        .as_str()
    {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "ico" => "image/x-icon",
        "avif" => "image/avif",
        _ => return None,
    };
    let path = base_dir.join(url.replace("%20", " "));
    match fs::read(&path) {
        Ok(bytes) => Some(format!("data:{mime};base64,{}", base64_encode(&bytes))),
        Err(e) => {
            eprintln!("画像を埋め込めませんでした [{}]: {}", path.display(), e);
            None
        }
    }
}

fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { TABLE[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[n as usize & 63] as char } else { '=' });
    }
    out
}

// ── ファイル変換 ─────────────────────────────────────────────────────────────

/// 数式を KaTeX で描画するためのタグ（数式を含む文書にだけ挿入する）
const KATEX_HEAD: &str = r#"<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.js"></script>
<script>
document.addEventListener("DOMContentLoaded", function () {
  document.querySelectorAll(".math").forEach(function (el) {
    katex.render(el.textContent, el, {
      displayMode: el.classList.contains("math-display"),
      throwOnError: false
    });
  });
});
</script>"#;

fn convert_md(target: &Path, opts: &ConvertOptions) -> MyResult<Vec<PathBuf>> {
    let mdcss = mycss::gen_mdcss(&opts.width, opts.table_scale);
    let markdownstr = fs::read_to_string(target)?;

    // 出力先パスを組み立て
    let mut out_path = env::current_dir()?;
    out_path.push(target);
    out_path.set_extension("html");

    let filetitle = out_path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "document".to_owned());

    let base_dir = target.parent().unwrap_or_else(|| Path::new("."));
    let (body_html, has_math, links) = render_markdown(&markdownstr, base_dir, opts);

    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (filetitle) }
                style { (PreEscaped(mdcss)) }
                @if has_math { (PreEscaped(KATEX_HEAD)) }
            }
            body {
                (PreEscaped(body_html))
            }
        }
    };

    fs::write(&out_path, markup.into_string())?;
    println!("生成しました: {}", out_path.display());
    Ok(links)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_matches_rfc4648() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn width_parsing() {
        assert_eq!(parse_width("1000").unwrap(), "1000px");
        assert_eq!(parse_width("90%").unwrap(), "90%");
        assert_eq!(parse_width("60em").unwrap(), "60em");
        assert!(parse_width("abc").is_err());
        assert!(parse_width("10px;}").is_err());
    }

    #[test]
    fn md_link_rewriting() {
        let conv = |u: &str| md_link_to_html(u).map(|(url, _)| url);
        assert_eq!(conv("design.md").as_deref(), Some("design.html"));
        assert_eq!(conv("docs/setup.markdown#install").as_deref(), Some("docs/setup.html#install"));
        assert_eq!(conv("README.MD?v=1").as_deref(), Some("README.html?v=1"));
        assert_eq!(conv("../a.mdown").as_deref(), Some("../a.html"));
        assert_eq!(conv("https://example.com/a.md"), None);
        assert_eq!(conv("mailto:a@example.com"), None);
        assert_eq!(conv("#top"), None);
        assert_eq!(conv("pic.png"), None);
        assert_eq!(conv("v1.md/index"), None);
        assert_eq!(md_link_to_html("a.md#x").unwrap().1, "a.md");
    }

    #[test]
    fn table_scale_parsing() {
        assert_eq!(parse_table_scale("1.4").unwrap(), 1.4);
        assert!(parse_table_scale("0.5").is_err());
        assert!(parse_table_scale("4").is_err());
        assert!(parse_table_scale("abc").is_err());
    }
}
