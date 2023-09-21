mod mycss;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use pulldown_cmark::{self, html::push_html};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_args: Vec<String> = env::args().collect();
    // プログラムの引数から対象ファイル名を取得
    let readfiles = &app_args[1..]; //可変長引数
    markdown_to_html(readfiles)?;
    Ok(())
}

fn markdown_to_html(targets: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mdcss = mycss::gen_mdcss();

    for target in targets {
        // markdownファイルを読み込み
        let targetfile = File::open(target)?;
        let mut reader = BufReader::new(targetfile);
        let mut markdownstr = String::new();
        reader.read_to_string(&mut markdownstr)?;

        // htmlファイルの名前を生成
        // カレントディレクトリを取得
        let mut filename = env::current_dir()?;
        // 対象マークダウンのファイル名を追加
        filename.push(target);
        // html titleにマークダウンの拡張子抜きのファイル名を割り当て
        let filetitle = filename.file_stem();

        if let Some(ft) = filetitle {
            let filetitle = ft.to_string_lossy();
            // ヘッダを生成
            let headtext = html! {
                header{
                meta charset="utf-8";
                title {(filetitle)};
                style {(mdcss)};
                }
            };

            // markdownをhtmlに変換準備
            let markuptext = html! {
                (DOCTYPE)html{
                    (headtext)
                    (Markdown(&markdownstr).render())
                }
            };
            // filenameの拡張子をhtmlに変更
            filename.set_extension("html");
            let filename = filename.to_str().unwrap();
            println!("file path:\n{}", filename);
            // htmlをファイルに書き込み
            // writerを準備
            let mut writebuffer = File::create(filename)?;
            // マークダウンをhtmlに変換レンダリング
            let outcontent = markuptext.render().into_string();
            // 書き込み
            let _wtlen = writebuffer.write(outcontent.as_bytes())?;
        };
    }
    Ok(())
}

/// Renders a block of Markdown using `pulldown-cmark`.
struct Markdown<T: AsRef<str>>(T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        // Generate raw HTML
        // マークアップの時のオプション機能を設定
        let mut parseoption = pulldown_cmark::Options::empty();
        // テーブル機能
        parseoption.insert(pulldown_cmark::Options::ENABLE_TABLES);
        // ハイフンや...を連続文字を適切な形に変形してくれる
        parseoption.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
        // 取り消し線
        parseoption.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        // タスクリスト
        parseoption.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
        
        let mut my_html = String::new();
        let parser = pulldown_cmark::Parser::new_ext(self.0.as_ref(), parseoption);
        push_html(&mut my_html, parser);
        PreEscaped(my_html)
    }
}
