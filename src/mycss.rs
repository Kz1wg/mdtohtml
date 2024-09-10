pub fn gen_mdcss() -> String {
    r#"
body {
    font-family: 'Segoe UI', Arial, sans-serif;
    line-height: 1.6;
    color: #333;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    background-color: #f5f5f5;
}

h1,
h2,
h3,
h4,
h5,
h6 {
    color: #2c3e50;
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
}

h1 {
    font-size: 2.5em;
    border-bottom: 1px solid #ddd;
    padding-bottom: 10px;
}

h2 {
    font-size: 2em;
}

h3 {
    font-size: 1.5em;
}

a {
    color: #3498db;
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}


pre {
    background-color: #f8f8f8;
    border: 1px solid #ddd;
    border-radius: 3px;
    padding: 16px;
    overflow: auto;
}


blockquote {
    border-left: 4px solid #3498db;
    margin: 0;
    padding-left: 20px;
    color: #555;
}

table {
    border-collapse: collapse;
    width: 100%;
    margin-bottom: 16px;
}

th,
td {
    text-align: left;
    padding: 12px;
    border-bottom: 1px solid #ddd;
}

th {
    background-color: #3498db;
    color: white;
}

tr:nth-child(even) {
    background-color: #f2f2f2;
}

img {
    max-width: 100%;
    height: auto;
}

hr {
    border: 0;
    border-top: 1px solid #ddd;
    margin: 20px 0;
}


/* インラインコードのスタイル */
code {
    background-color: #1e1e1e;
    color: #d4d4d4;
    border: none;
    border-radius: 5px;
    font-family: Consolas, Monaco, monospace;
    font-size: 0.9em;
    padding: 2px 4px;
    position: relative;
}


/* コードブロックのスタイル */

/* 背景と文字色の設定 */
pre code {
    background-color: #2d2d2d;
    /* ダーク背景 */
    color: #f8f8f2;
    /* 文字色 */
    padding: 10px;
    border-radius: 5px;
    display: block;
    overflow-x: auto;
}

/* コメント */
pre code .comment {
    color: #75715e;
    font-style: italic;
}

/* 文字列 */
pre code .string {
    color: #e6db74;
}

/* キーワード */
pre code .keyword {
    color: #f92672;
    font-weight: bold;
}

/* 関数名 */
pre code .function {
    color: #a6e22e;
}

/* 数値 */
pre code .number {
    color: #ae81ff;
}

/* 定数や変数 */
pre code .variable {
    color: #f8f8f2;
}

/* セレクタなど */
pre code .selector {
    color: #66d9ef;
}


  "#
    .to_string()
}
