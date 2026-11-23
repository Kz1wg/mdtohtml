# mdtohtml

[English](README.md) | 日本語

Markdown を、見やすい HTML ファイルに変換するコマンドラインツールです。
CSS は HTML の中に埋め込むので、生成されたファイル1つをブラウザで開くだけで表示できます。

## インストール

### ビルド済みの実行ファイルを使う（Rust 不要）

[Releases](https://github.com/Kz1wg/mdtohtml/releases) から、使っている OS 向けの zip をダウンロードして展開します。

| OS | ファイル |
|---|---|
| Windows | `mdtohtml-x86_64-pc-windows-msvc.zip` |
| Mac（Apple Silicon） | `mdtohtml-aarch64-apple-darwin.zip` |
| Mac（Intel） | `mdtohtml-x86_64-apple-darwin.zip` |
| Linux | `mdtohtml-x86_64-unknown-linux-gnu.zip` |

展開した `mdtohtml`（Windows では `mdtohtml.exe`）を、PATH の通ったフォルダに置いてください。

- **Windows**：初回の実行時に SmartScreen の警告が出たら、「詳細情報」→「実行」を選びます。
- **Mac**：署名していないため、ブロックされたら `xattr -d com.apple.quarantine mdtohtml` を実行します。

### ソースからビルドする

```bash
cargo install --path .
```

インストールすると、どのフォルダからでも `mdtohtml` コマンドを使えます。
インストールせずに試すときは、プロジェクトのフォルダで `cargo run -- test.markdown` のように実行します。

## 使い方

```bash
mdtohtml README.md                  # README.html を生成
mdtohtml a.md b.md docs/*.md        # 複数ファイルをまとめて変換
mdtohtml -r index.md                # リンク先の Markdown も辿って一括変換
mdtohtml -e report.md               # 画像を埋め込み、HTML 1ファイルで共有できるようにする
```

HTML は、元の Markdown と同じフォルダに、拡張子を `.html` に変えた名前で出力されます。
同名の HTML がすでにある場合は上書きします。
変換できる拡張子は `.md` / `.markdown` / `.mdown` です。

## オプション

| オプション | 説明 |
|---|---|
| `-e`, `--embed-images` | ローカル画像を base64 で HTML に埋め込む |
| `--width <WIDTH>` | 本文の最大幅（既定 `800px`）。数値だけなら px として扱う。`90%`、`60em` なども指定できる |
| `--table-scale <倍率>` | テーブルが本文幅の何倍まで広がれるか（既定 `1.2`、1.0〜3.0） |
| `-l`, `--md-links` | Markdown ファイルへのリンクを `.html` へのリンクに書き換える |
| `-r`, `--recursive` | リンク先の Markdown も辿って一括変換する（`-l` を含む） |
| `--css <FILE>` | 追加の CSS ファイルを読み込み、色などを上書きする（複数指定可） |
| `-h`, `--help` | ヘルプを表示する |
| `-V`, `--version` | バージョンを表示する |

## 対応している Markdown 記法

- 表（GFM）、取り消し線、タスクリスト（`- [x]`）
- コードブロックのシンタックスハイライト（Rust, Python, JavaScript, Bash など）
- 数式：`$...$`（インライン）と `$$...$$`（ブロック）を KaTeX で表示する
- 見出しへの id の自動付与（GitHub と同じ規則）
  - 例：`## Getting Started` → `#getting-started`、`## 設定方法` → `#設定方法`
  - 同じ見出しが重なったときは `-1`、`-2` を付ける
  - 手動で指定するときは `## 見出し {#custom-id}` と書く

## 表示の特徴

- **テーブル**：内容が多いときだけ、本文幅の 1.2 倍まで左右に広がる。それ以上は表の中で横スクロールする
- **短い列**：ID や状態のように値が短い列は、折り返さずに1行で表示する
- **見出しへのリンク**：`#見出し` へのリンクで飛ぶと、飛んだ先の見出しが一瞬ハイライトされる
- **印刷**：チェックボックスなどの色は、印刷しても消えない

## リンクの変換（`-l` / `-r`）

`-l` を付けると、`[設計](design.md#概要)` は `design.html#概要` に書き換わります。

- 外部 URL、`mailto:`、ページ内リンク（`#...`）、Markdown 以外のファイルへのリンクは書き換えません
- リンク先の Markdown が存在しない場合は、警告を出してリンクを `.md` のまま残します

`-r` を付けると、リンク先の Markdown も辿って変換します。

- 辿るのは、最初に指定したファイルのフォルダ以下だけです
  - その外にあるリンク先は変換せず、リンクも `.md` のまま残します
  - ただし、対応する `.html` がすでにある場合は、そちらへリンクします
- 同じファイルを2回変換することはありません。循環しているリンクでも止まります
- 一度に変換するのは最大 30 ファイルまでです

## 注意事項

- **数式**：KaTeX をネットから読み込むので、オフラインで開くと数式はそのままの文字で表示されます。KaTeX を読み込むのは、数式を含む文書だけです
- **画像の埋め込み（`-e`）**：埋め込むのはローカルの画像だけです。URL で指定した画像は埋め込みません

## 見た目のカスタマイズ

### `--css` で上書きする（おすすめ）

自分で用意した CSS ファイルを `--css` で指定すると、既定の CSS の後に読み込まれます。
変えたい色の変数だけを書けば、その部分だけが上書きされます。

```css
/* company.css：会社の資料用 */
:root {
    --color-th-bg:   #00598f;
    --color-heading: #00395c;
}
```

```bash
mdtohtml --css company.css report.md
```

- `--css` は複数指定できます。後に指定したものが優先されます
- 変数だけでなく、普通の CSS ルール（例：`body { font-family: ...; }`）も書けます

主な変数は次のとおりです。全部の変数は [src/mycss.rs](src/mycss.rs) の冒頭にある `:root { ... }` にあります。

| 変数 | 用途 |
|---|---|
| `--color-heading` | 見出しの文字色 |
| `--color-link` | リンクの文字色 |
| `--color-th-bg` | テーブル見出しの背景色 |
| `--color-inline-code-*` | インラインコードの配色 |
| `--color-checkbox` | チェック済みチェックボックスの色 |
| `--color-target-flash` | リンクで飛んだ先の見出しのハイライト色 |

### 既定の見た目を変える

[src/mycss.rs](src/mycss.rs) を直接書き換えて、`cargo install --path .` で再インストールします。
コードブロックの配色テーマは、[src/highlight.rs](src/highlight.rs) の `"Solarized (light)"` を書き換えると変えられます。
