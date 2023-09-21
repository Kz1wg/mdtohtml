pub fn gen_mdcss() -> String {
  r#"
  /*//////////////////////////////////////////////*/
  /* common */
  /*//////////////////////////////////////////////*/
 
  body {
     font-family:'Courier New', Courier, monospace;
     color: #1b1b1b;
     font-size: medium;
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
      background-color: darkslateblue;
      color: cornsilk;
      font-size: 90%;
      font-style: normal;
      padding: 0 5px;
      border-radius: 3px;
  }
 
 
  /*//////////////////////////////////////////////*/
  /* list */
  /*//////////////////////////////////////////////*/
 
  ul,
  ol {
      padding: 0 0 0.5em 1.5em;
  }
 
 
 
 
 
 
 
  /*//////////////////////////////////////////////*/
  /* table */
  /*//////////////////////////////////////////////*/
 
  table {
      border-collapse: collapse;
      border: 1px solid #b7bdbc !important;
      line-height: 1.5;
      margin: 10px 0;
      /* width: 100%; */
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
      background: #c9d2d2;
      border: 2px solid #b8bfbe;
      border-radius: 10px;
      margin: 20px 0;
      padding: 1em;
  }
 
  .box_pink {
      background: #ddcccc;
      border: 2px solid #d47e7e;
      border-radius: 10px;
      margin: 20px 0;
      padding: 1em;
  }
 
  .box_pink ul,
  .box_pink ol {
      padding: 0 0 0 1.5em;
  }
 
 
  .box_gray ul,
  .box_gray ol {
      padding: 0 0 0 1.5em;
  }  
  "#
    .to_string()
}
