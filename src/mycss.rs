/// CSS を文字列として返す。
/// 色のカスタマイズは冒頭の `:root { --変数名: 値; }` を編集するだけでOK。
pub fn gen_mdcss(content_width: &str, table_scale: f64) -> String {
    r#"
/* =====================================================================
   カラーテーマ変数 — ここだけ編集すれば全体の色が変わります
   ===================================================================== */
:root {
    /* 本文の最大幅（--width オプションで変更） */
    --content-width:      __CONTENT_WIDTH__;
    /* テーブルが本文幅の何倍まで広がれるか（--table-scale オプションで変更） */
    --table-scale:        __TABLE_SCALE__;

    /* ページ */
    --color-bg:           #f5f5f5;
    --color-text:         #333333;
    --color-text-muted:   #555555;

    /* 見出し */
    --color-heading:      #2c3e50;

    /* リンク */
    --color-link:         #3498db;

    /* 区切り線・ボーダー */
    --color-border:       #dddddd;

    /* テーブル */
    --color-th-bg:        #3498db;
    --color-th-text:      #ffffff;
    --color-tr-even:      #f2f2f2;

    /* 引用 */
    --color-blockquote:   #3498db;

    /* コードブロック背景（syntect が inline style を生成するため参考値） */
    --color-code-bg:      #ede8db;   /* base16-ocean.dark の背景色 */

    /* インラインコード */
    --color-inline-code-bg:     #eef3f8;   /* 淡いブルーグレー */
    --color-inline-code-text:   #1d4f7a;   /* 深い紺 */
    --color-inline-code-border: #d5e0eb;   /* 細い縁取り */

    /* タスクリストのチェックボックス */
    --color-checkbox:         #1f9d55;   /* チェック済みの塗り */
    --color-checkbox-border:  #888888;   /* 未チェックの枠線 */
}

/* =====================================================================
   ベーススタイル
   ===================================================================== */
/* 100vw のテーブルラッパーが縦スクロールバー分はみ出すのを防ぐ */
html { overflow-x: hidden; }

body {
    font-family: 'Segoe UI', Arial, sans-serif;
    line-height: 1.6;
    color: var(--color-text);
    max-width: var(--content-width);
    margin: 0 auto;
    padding: 20px;
    background-color: var(--color-bg);
}

h1, h2, h3, h4, h5, h6 {
    color: var(--color-heading);
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
}

h1 {
    font-size: 2.5em;
    border-bottom: 1px solid var(--color-border);
    padding-bottom: 10px;
}

h2 { font-size: 2em; }
h3 { font-size: 1.5em; }

a {
    color: var(--color-link);
    text-decoration: none;
}
a:hover { text-decoration: underline; }

hr {
    border: 0;
    border-top: 1px solid var(--color-border);
    margin: 20px 0;
}

img {
    max-width: 100%;
    height: auto;
}

/* =====================================================================
   タスクリスト — disabled なチェックボックスはブラウザ標準だと
   薄い水色になるため、自前で描画する
   ===================================================================== */
input[type="checkbox"] {
    appearance: none;
    -webkit-appearance: none;
    width: 1.1em;
    height: 1.1em;
    margin: 0 0.4em 0 0;
    vertical-align: -0.15em;
    border: 2px solid var(--color-checkbox-border);
    border-radius: 3px;
    background: #ffffff center / 85% no-repeat;
    print-color-adjust: exact;          /* 印刷時も色を残す */
    -webkit-print-color-adjust: exact;
}

input[type="checkbox"]:checked {
    border-color: var(--color-checkbox);
    background-color: var(--color-checkbox);
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3E%3Cpath d='M3 8.5l3.2 3.2L13 5' fill='none' stroke='white' stroke-width='2.4' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
}

/* =====================================================================
   引用
   ===================================================================== */
blockquote {
    border-left: 4px solid var(--color-blockquote);
    margin: 0;
    padding-left: 20px;
    color: var(--color-text-muted);
}

/* =====================================================================
   テーブル
   ===================================================================== */
/* テーブルのラッパー
   - 幅は内容に合わせて決まる（max-content）
   - 最小は本文幅、最大は「本文幅 × --table-scale」とウィンドウ幅の小さい方
   - 本文の中央を基準に左右対称にはみ出し、上限を超える分は横スクロール
   （ここでの 100% は本文の幅） */
.table-wrap {
    width: max-content;
    min-width: 100%;
    max-width: min(100vw - 40px, 100% * var(--table-scale));
    margin-left: 50%;
    transform: translateX(-50%);
    overflow-x: auto;
    margin-bottom: 16px;
}

table {
    border-collapse: collapse;
    width: 100%;
}

/* 短い値しかない列（ID・状態など）は縦書き状に潰れないよう折り返さない */
th.nowrap, td.nowrap {
    white-space: nowrap;
}

th, td {
    text-align: left;
    padding: 12px;
    border-bottom: 1px solid var(--color-border);
}

th {
    background-color: var(--color-th-bg);
    color: var(--color-th-text);
}

tr:nth-child(even) {
    background-color: var(--color-tr-even);
}

/* =====================================================================
   コード — syntect が <pre><code> に inline style を付与するため、
   ここではレイアウトと余白だけを制御する
   ===================================================================== */

/* コードブロック全体のラッパー */
pre {
    background-color: var(--color-code-bg);
    border-radius: 6px;
    padding: 0;          /* syntect の <pre> に余白を付けない（二重余白防止） */
    overflow: auto;
    margin-bottom: 16px;
}

pre code {
    display: block;
    padding: 16px;
    overflow-x: auto;
    font-family: Consolas, 'Cascadia Code', Monaco, monospace;
    font-size: 0.9em;
    line-height: 1.5;
    /* 色は syntect の inline style が上書きするため不要 */
}

/* インラインコード（コードブロック外） */
code {
    background-color: var(--color-inline-code-bg);
    color: var(--color-inline-code-text);
    border: 1px solid var(--color-inline-code-border);
    border-radius: 4px;
    font-family: Consolas, 'Cascadia Code', Monaco, monospace;
    font-size: 0.9em;
    padding: 2px 5px;
}

/* <pre> 内の <code> には上記インラインコード色を適用しない */
pre code {
    background-color: transparent;
    color: inherit;
    border: none;
    padding: 16px;   /* 上の pre code の余白を打ち消さないよう 16px を維持 */
    border-radius: 0;
}
"#
    .replace("__CONTENT_WIDTH__", content_width)
    .replace("__TABLE_SCALE__", &table_scale.to_string())
}
