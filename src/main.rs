use anyhow::Result;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use pulldown_cmark::{html::push_html, Options, Parser};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() -> Result<()> {
    let app_args: Vec<String> = env::args().collect();
    let readfiles = &app_args[1..];
    markdown_to_html(readfiles)?;
    Ok(())
}
fn getcss() -> String {
    let mut cssfile = File::open("./mdcss.css").expect("can not open css file");
    let mut css_content = String::new();
    cssfile
        .read_to_string(&mut css_content)
        .expect("can not read css file");
    css_content
}
fn markdown_to_html(targets: &[String]) -> Result<()> {
    // markdownパーサで使用するオプションを指定
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let mdcss = &getcss();
    for target in targets {
        // markdownファイルを読み込み
        let targetfile = File::open(target)?;
        let mut reader = BufReader::new(targetfile);
        let mut markdownstr = String::new();
        reader.read_to_string(&mut markdownstr)?;

        // htmlファイルの名前を生成
        let filename: Vec<&str> = target.split(".").collect();
        let filename = [filename[0], "html"].join(".");

        // ヘッダを生成
        let headtext = html! {
            header{
            meta charset="utf-8";
            (html!{
                style{
                    (mdcss);
                }
            })
            title { "Document" };
            }
        };

        // markdownをhtmlに変換
        let markuptext = html! {
            (DOCTYPE)html{
                (headtext)
                (Markdown(&markdownstr).render())
            }
        };
        // htmlをファイルに書き込み
        let mut writebuffer = File::create(filename)?;

        writebuffer.write(markuptext.render().into_string().as_bytes())?;
    }
    Ok(())
}

/// Renders a block of Markdown using `pulldown-cmark`.
struct Markdown<T: AsRef<str>>(T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        // Generate raw HTML
        let mut my_html = String::new();
        let parser = Parser::new(self.0.as_ref());
        push_html(&mut my_html, parser);
        PreEscaped(my_html)
    }
}
