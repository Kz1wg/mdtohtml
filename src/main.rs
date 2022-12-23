use anyhow::Result;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use pulldown_cmark::{html::push_html, Options, Parser};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() -> Result<()> {
    let app_args: Vec<String> = env::args().collect();
    let readfiles = &app_args[1..]; //可変長引数
    markdown_to_html(readfiles)?;
    Ok(())
}

fn markdown_to_html(targets: &[String]) -> Result<()> {
    let mdcss = gen_mdcss();
    for target in targets {
        // markdownファイルを読み込み
        let targetfile = File::open(target)?;
        let mut reader = BufReader::new(targetfile);
        let mut markdownstr = String::new();
        reader.read_to_string(&mut markdownstr)?;

        // htmlファイルの名前を生成
        let filename: Vec<&str> = target.split('.').collect();
        let filetitle = filename[0].to_owned();
        let filename = [filename[0], "html"].join(".");
        // ヘッダを生成
        let headtext = html! {
            header{
            meta charset="utf-8";
            title {(filetitle)};
            style {(mdcss)};
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
        let outcontent = markuptext.render().into_string();
        let _wtlen = writebuffer.write(outcontent.as_bytes())?;
    }
    Ok(())
}

/// Renders a block of Markdown using `pulldown-cmark`.
struct Markdown<T: AsRef<str>>(T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        // Generate raw HTML
        let mut parseoption = Options::empty();
        parseoption.insert(Options::ENABLE_TABLES);
        parseoption.insert(Options::ENABLE_SMART_PUNCTUATION);
        parseoption.insert(Options::ENABLE_STRIKETHROUGH);
        let mut my_html = String::new();
        let parser = Parser::new_ext(self.0.as_ref(), parseoption);
        push_html(&mut my_html, parser);
        PreEscaped(my_html)
    }
}

fn gen_mdcss() -> String {
    r###"
/* MIT LICENSE
Copyright (c) 2020 hachian
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the 'Software'), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

/*//////////////////////////////////////////////*/
/* common */
/*//////////////////////////////////////////////*/

body {
    background-color: #e5e8e8;
    color: #1b1b1b;
  }
  
  /*//////////////////////////////////////////////*/
  /* title */
  /*//////////////////////////////////////////////*/
  
  h1 {
    font-size: 36px;
    font-weight: bold;
    padding: 20px 0;
  }
  h2 {
    font-size: 22px;
    font-weight: bold;
    background-color: #33ad9d;
    color: #e5e8e8;
    padding: 10px 15px;
    border-radius: 5px;
    margin: 40px 0 10px;
  }
  h3 {
    font-size: 22px;
    font-weight: bold;
    border-bottom: 3px solid #33ad9d;
    margin: 40px 0 10px;
  }
  h4 {
    font-size: 18px;
    font-weight: bold;
    border-left: 3px solid #33ad9d;
    margin: 40px 0 10px;
    padding: 0 0 0 10px;
  }
  h5 {
    font-size: 16px;
    margin: 40px 0 10px;
    font-weight: bold;
  }
  
  /*//////////////////////////////////////////////*/
  /* tags */
  /*//////////////////////////////////////////////*/
  
  hr {
    height: 1px;
    background-color: #b8bfbe;
    margin: 20px 0;
  }
  em {
    background: linear-gradient(transparent 80%, #e7ff46 80%);
    padding: 0 5px;
    font-style: normal;
  }
  strong {
    background: linear-gradient(transparent 0%, #e7ff46 0%);
    padding: 0 5px;
    border-radius: 5px;
  }
  p code,
  li code,
  td code {
    font-family: 'Consolas', 'Courier New', monospace;
    background-color: #d0d8d7;
    font-size: 90%;
    font-style: normal;
    padding: 0 5px;
    border-radius: 3px;
    color: #1b1b1b;
  }
  pre {
    background-color: #272b2b !important;
  }
  
  /*//////////////////////////////////////////////*/
  /* list */
  /*//////////////////////////////////////////////*/
  
  ul,ol {
    padding: 0 0 0.5em 1.5em;
  }


  ul ul li::after {
    display: block;
    content: '';
    position: absolute;
    top: 0.75em;
    left: -1em;
    width: 6px;
    height: 2px;
    background-color: #33ad9d;
  }
  ul ul ul li::after {
    display: block;
    content: '';
    position: absolute;
    top: 0.5em;
    left: -1em;
    width: 6px;
    height: 6px;
    border-right: 2px solid #33ad9d;
    border-bottom: 2px solid #33ad9d;
    background-color: inherit;
    border-radius: 0;
    -webkit-transform: rotate(-45deg);
    transform: rotate(-45deg);
  }

  

  

  
  /*//////////////////////////////////////////////*/
  /* table */
  /*//////////////////////////////////////////////*/
  
  table {
    border-collapse: collapse;
    border: 1px solid #b7bdbc !important;
    line-height: 1.5;
    margin: 10px 0;
    width: 100%;
  }
  table th {
    padding: 10px;
    font-size: 12px;
    font-weight: bold;
    vertical-align: top;
    text-align: left;
    background: #c6cbca;
    border: 1px solid #b7bdbc !important;
  }
  table td {
    font-size: 12px;
    padding: 10px;
    vertical-align: top;
    border: 1px solid #b7bdbc !important;
  }
  
  /* stripe */
  table.stripe tr:nth-child(even) {
    background: #dcdfdf;
  }
  
  /* dark th */
  table.dark th {
    color: #e5e8e8;
    background: #272b2b;
  }
  
  /*//////////////////////////////////////////////*/
  /* box */
  /*//////////////////////////////////////////////*/
  
  .box_gray {
    background: #dcdfdf;
    border: 2px solid #b8bfbe;
    border-radius: 10px;
    margin: 20px 0;
    padding: 1em;
  }
  
  .box_pink {
    background: #e7d5d5;
    border: 2px solid #d47e7e;
    border-radius: 10px;
    margin: 20px 0;
    padding: 1em;
  }
  
  .box_pink ul, .box_pink ol {
    padding: 0 0 0 1.5em;
  }
  
  
  .box_gray ul, .box_gray ol {
    padding: 0 0 0 1.5em;
  }
  
 
  "###.to_string()
}
