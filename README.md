# Ona

A fast, elegant CLI tool that transforms Markdown into beautiful HTML and opens it in your default browser.

## Features

- **Beautiful rendering** - GitHub-inspired styling with elegant typography
- **Dark/Light mode** - Automatic system theme detection or manual override
- **Syntax highlighting** - Code blocks with language detection and highlighting
- **Copy code button** - One-click copy for code blocks (SVG icons)
- **Fast** - Built with Rust for instant rendering
- **Zero dependencies** - Single binary, works offline

## Installation

### Homebrew (macOS & Linux)

```bash
brew install proteusiq/tap/ona
```

### Shell installer (macOS & Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Proteusiq/ona/releases/latest/download/ona-installer.sh | sh
```

### PowerShell installer (Windows)

```powershell
powershell -c "irm https://github.com/Proteusiq/ona/releases/latest/download/ona-installer.ps1 | iex"
```

### Cargo (any platform with a Rust toolchain)

```bash
cargo install --locked ona
```

### From source

```bash
git clone https://github.com/Proteusiq/ona.git
cd ona
cargo install --path .
```

### Manual download

Prebuilt binaries for macOS, Linux (glibc & musl), and Windows are attached to
every [GitHub Release](https://github.com/Proteusiq/ona/releases).

## Usage

```bash
# Open a markdown file in your browser
ona README.md

# Read from stdin
cat document.md | ona -

# Force dark mode
ona --theme dark README.md

# Force light mode  
ona --theme light README.md

# Save to file instead of opening browser
ona -o output.html README.md

# Print HTML to stdout
ona -p README.md

# Custom title
ona --title "My Document" README.md
```

## Options

```
Usage: ona [OPTIONS] [FILE]

Arguments:
  [FILE]  Path to a markdown file (use '-' for stdin)

Options:
  -t, --theme <THEME>  Theme mode: auto, light, or dark [default: auto]
  -o, --output <FILE>  Output to file instead of opening in browser
  -p, --print          Print the generated HTML to stdout
      --title <TITLE>  Title for the HTML page
  -h, --help           Print help
  -V, --version        Print version
```

## Supported Markdown Features

- Headings (h1-h6)
- Bold, italic, strikethrough
- Links and images
- Code blocks with syntax highlighting
- Inline code
- Blockquotes
- Ordered and unordered lists
- Task lists (checkboxes)
- Tables
- Horizontal rules
- Footnotes

## Building

```bash
git clone https://github.com/proteusiq/ona.git
cd ona
cargo build --release
```

The binary will be at `target/release/ona`.

## License

MIT

## Author

Prayson Wilfred Daniel ([@proteusiq](https://github.com/proteusiq))
