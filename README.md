# mdtohtml

English | [日本語](README.ja.md)

A command-line tool that converts Markdown into clean, readable HTML.
The CSS is embedded in the output, so each generated file can be opened directly in a browser.

> Messages and `--help` output are in Japanese.

## Installation

### Prebuilt binaries (no Rust needed)

Download the zip for your OS from [Releases](https://github.com/KIwGucci/mdtohtml/releases) and extract it.

| OS | File |
|---|---|
| Windows | `mdtohtml-x86_64-pc-windows-msvc.zip` |
| macOS (Apple Silicon) | `mdtohtml-aarch64-apple-darwin.zip` |
| macOS (Intel) | `mdtohtml-x86_64-apple-darwin.zip` |
| Linux | `mdtohtml-x86_64-unknown-linux-gnu.zip` |

Put the extracted `mdtohtml` (`mdtohtml.exe` on Windows) in a folder on your PATH.

- **Windows**: if SmartScreen shows a warning the first time, choose "More info" → "Run anyway".
- **macOS**: the binary is not signed. If it is blocked, run `xattr -d com.apple.quarantine mdtohtml`.

### Build from source

```bash
cargo install --path .
```

After installing, you can run `mdtohtml` from any directory.
To try it without installing, run something like `cargo run -- test.markdown` in the project directory.

## Usage

```bash
mdtohtml README.md                  # generates README.html
mdtohtml a.md b.md docs/*.md        # convert several files at once
mdtohtml -w memo.md                 # re-convert automatically on every save
mdtohtml -r index.md                # also convert the Markdown files it links to
mdtohtml -e report.md               # embed images so the HTML is a single shareable file
```

The HTML file is written next to the source file, with the extension changed to `.html`.
An existing HTML file with the same name is overwritten.
Supported extensions are `.md`, `.markdown`, and `.mdown`.

## Options

| Option | Description |
|---|---|
| `-e`, `--embed-images` | Embed local images into the HTML as base64 |
| `-w`, `--watch` | Watch the files and re-convert on every change (Ctrl+C to stop) |
| `--width <WIDTH>` | Maximum width of the body text (default `800px`). A plain number is treated as px; values such as `90%` or `60em` also work |
| `--table-scale <RATIO>` | How far tables may extend beyond the body width (default `1.2`, range 1.0–3.0) |
| `-l`, `--md-links` | Rewrite links to Markdown files so they point to the `.html` files |
| `-r`, `--recursive` | Also convert the Markdown files that are linked to (implies `-l`) |
| `-h`, `--help` | Show help |
| `-V`, `--version` | Show version |

## Supported Markdown

- Tables (GFM), strikethrough, and task lists (`- [x]`)
- Syntax highlighting in code blocks (Rust, Python, JavaScript, Bash, and more)
- Math: `$...$` (inline) and `$$...$$` (block), rendered with KaTeX
- Automatic heading ids, using the same rules as GitHub
  - Examples: `## Getting Started` → `#getting-started`, `## 設定方法` → `#設定方法`
  - Duplicate headings get `-1`, `-2`, and so on
  - To set an id yourself, write `## Heading {#custom-id}`

## Display

- **Tables**: a table with a lot of content extends beyond the body text, up to 1.2 times its width. Anything wider scrolls horizontally inside the table
- **Short columns**: columns that only hold short values, such as IDs or statuses, stay on one line instead of wrapping
- **Heading links**: when a `#heading` link is followed, the target heading is briefly highlighted
- **Printing**: colors such as checkbox fills are kept when printing

## Link conversion (`-l` / `-r`)

With `-l`, a link such as `[Design](design.md#overview)` becomes `design.html#overview`.

- External URLs, `mailto:`, in-page links (`#...`), and links to non-Markdown files are left unchanged
- If the linked Markdown file does not exist, a warning is shown and the link keeps its `.md` extension

With `-r`, the linked Markdown files are converted too.

- Only files under the folder of the file you specified are followed
  - Links to files outside that folder are not converted and keep their `.md` extension
  - If a matching `.html` file already exists, the link points to it instead
- Each file is converted only once, so circular links do not cause loops
- At most 30 files are converted in one run
- Combined with `-w`, every file that was followed is watched. Files linked from new links are converted and watched automatically

## Notes

- **Math**: KaTeX is loaded from a CDN, so math is shown as plain text when the file is opened offline. KaTeX is only loaded for documents that contain math
- **Embedding images (`-e`)**: only local images are embedded. Images referenced by URL are not
- **Watching (`-w`)**: only the Markdown files are watched. Replacing an image does not trigger a re-conversion

## Customizing the look

Colors are set with the CSS variables in `:root { ... }` at the top of [src/mycss.rs](src/mycss.rs).
The main variables are:

| Variable | Purpose |
|---|---|
| `--color-heading` | Heading text color |
| `--color-link` | Link text color |
| `--color-th-bg` | Table header background |
| `--color-inline-code-*` | Inline code colors |
| `--color-checkbox` | Color of checked checkboxes |
| `--color-target-flash` | Highlight color for the heading a link jumps to |

After making changes, reinstall with `cargo install --path .`.
To change the color theme of code blocks, replace `"Solarized (light)"` in [src/highlight.rs](src/highlight.rs).
