use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() -> Result<()> {
    let app_args: Vec<String> = env::args().collect();
    let readfiles = &app_args[1..];
    markdown_to_html(readfiles)?;
    Ok(())
}

fn markdown_to_html(targets: &[String]) -> Result<()> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    for target in targets {
        let targetfile = File::open(target)?;
        let mut reader = BufReader::new(targetfile);
        let mut markdownstr = String::new();
        reader.read_to_string(&mut markdownstr)?;
        let parser = Parser::new_ext(&markdownstr, options);

        // Write to String buffer.
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        let filename: Vec<&str> = target.split(".").collect();
        let filename = [filename[0], "html"].join(".");
        let markuptext = markup(html_output);
        let mut writebuffer = File::create(filename)?;
        writebuffer.write(markuptext.as_bytes())?;
    }
    Ok(())
}

fn markup(bodytext: String) -> String {
    use maud::{html, DOCTYPE};

    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Markdown" }
            }
        }
    };
    markup.into_string() + &bodytext
}
