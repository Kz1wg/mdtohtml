pub fn gen_mdcss() -> String {
    r#"
    body {
        font-family: sans-serif;
        font-size: 16px;
        line-height: 1.5;
        color: #333;
      }
      
      h1, h2, h3, h4, h5, h6 {
        font-weight: bold;
        margin-top: 0;
      }
      
      p {
        margin: 0 0 10px;
      }
      
      ul, ol {
        margin: 0 0 10px;
        padding: 0;
      }
      
      li {
        list-style: none;
        margin: 0 0 5px;
      }
      
      a {
        color: #000088;
        text-decoration: none;
      }
      
      a:hover {
        color: #0000cc;
      }
      
      blockquote {
        margin: 0 0 10px;
        padding: 10px 20px;
        border-left: 5px solid #ccc;
        font-style: italic;
      }
      
      code {
        font-family: monospace;
        color: #000088;
        background-color: #eee;
        padding: 5px;
      }
      
      pre {
        margin: 0 0 10px;
        padding: 10px 20px;
        border: 1px solid #ccc;
        font-family: monospace;
      }
      
      img {
        max-width: 100%;
        height: auto;
      }
      
  "#
    .to_string()
}
