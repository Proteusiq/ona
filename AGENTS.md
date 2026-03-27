# Ona - Agent Instructions

> Beautiful markdown to HTML renderer with browser preview

## Project Overview

**Ona** transforms markdown into stunning HTML and opens it in your default browser. Built with Rust for speed and elegance.

```
┌─────────────────┐     ┌─────────────┐     ┌──────────────┐
│   Markdown      │     │   Ona CLI   │     │   Browser    │
│   README.md     │ ──▶ │   (Rust)    │ ──▶ │   Preview    │
└─────────────────┘     └─────────────┘     └──────────────┘
```

## Architecture

```
src/
├── main.rs        # CLI entry, argument parsing, markdown rendering
└── template.rs    # HTML generation, CSS styles, JavaScript
```

### Core Components

| Component | Crate | Purpose |
|-----------|-------|---------|
| CLI | `clap` (derive) | Argument parsing with derive macros |
| Markdown | `pulldown-cmark` | GFM parsing (tables, task lists, footnotes) |
| Syntax | `syntect` | Code block highlighting |
| Browser | `open` | Cross-platform browser opening |
| Temp files | `tempfile` | Temporary HTML file creation |

### Data Flow

```
┌──────────────────────────────────────────────────────────────┐
│  INPUT                                                       │
│  ├── File path (README.md)                                   │
│  ├── Stdin (cat file.md | ona -)                             │
│  └── Piped input                                             │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│  PARSING (main.rs)                                           │
│  ├── pulldown-cmark: Markdown → Events                       │
│  ├── Code blocks intercepted for syntax highlighting         │
│  └── syntect: Code → Highlighted HTML                        │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│  TEMPLATE (template.rs)                                      │
│  ├── CSS: GitHub-inspired styles, dark/light themes          │
│  ├── JS: Theme toggle, copy button, smooth scroll            │
│  └── HTML: Full document with embedded styles                │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│  OUTPUT                                                      │
│  ├── Browser: Temp file opened with `open` crate             │
│  ├── File: -o output.html                                    │
│  └── Stdout: -p flag                                         │
└──────────────────────────────────────────────────────────────┘
```

## Key Files

### `src/main.rs`

- `Cli` struct: clap derive for argument parsing
- `read_input()`: Handle file, stdin, or piped input
- `render_markdown()`: Parse markdown, intercept code blocks
- `highlight_code()`: Syntax highlighting with syntect

### `src/template.rs`

- `generate_html()`: Combine content with CSS/JS template
- `CSS`: ~800 lines of GitHub-inspired styles
- `JS`: Theme toggle with localStorage, copy code functionality

## Theme System

```
┌─────────────────────────────────────────────────────────────┐
│  THEME MODES                                                │
├─────────────────────────────────────────────────────────────┤
│  auto   │  Uses prefers-color-scheme media query            │
│  light  │  Forces html.light class                          │
│  dark   │  Forces html.dark class                           │
└─────────────────────────────────────────────────────────────┘
```

CSS uses CSS variables for theming:
- `--color-bg`, `--color-text`, `--color-link`, etc.
- Dark mode overrides via `@media (prefers-color-scheme: dark)`
- Forced modes via `.light` and `.dark` classes

## Development

### Verification Loop

```bash
cargo fmt                      # Format
cargo clippy -- -D warnings    # Lint (zero warnings)
cargo test                     # Test (12 tests)
cargo build --release          # Build
```

### Install Locally

```bash
cp target/release/ona ~/.local/bin/
```

## Future Enhancements

### High Priority

- [ ] **Watch mode**: Auto-reload on file changes (`--watch`)
- [ ] **Custom CSS**: Allow user-provided stylesheets (`--css`)
- [ ] **PDF export**: Generate PDF via headless browser
- [ ] **TOC generation**: Auto table of contents for long docs

### Medium Priority

- [ ] **Mermaid diagrams**: Render mermaid code blocks as SVG
- [ ] **Math support**: KaTeX/MathJax for LaTeX equations
- [ ] **Frontmatter**: Parse YAML frontmatter for title/metadata
- [ ] **Multiple files**: Combine multiple markdown files

### Low Priority

- [ ] **Presentation mode**: Slide-based view (like Marp)
- [ ] **Custom templates**: User-provided HTML templates
- [ ] **Plugin system**: Extensible rendering pipeline

## Testing

Tests follow AAA pattern (Arrange, Act, Assert):

```rust
#[test]
fn test_render_markdown_basic() {
    // Arrange
    let markdown = "# Hello\n\nThis is **bold** text.";

    // Act
    let result = render_markdown(markdown).unwrap();

    // Assert
    assert!(result.contains("<h1>Hello</h1>"));
    assert!(result.contains("<strong>bold</strong>"));
}
```

### Test Coverage

| Module | Tests | Coverage |
|--------|-------|----------|
| `html_escape` | 3 | XSS, ampersand, quotes |
| `render_markdown` | 5 | Headings, code, tables, tasks, links |
| `template` | 4 | HTML generation, theme modes |

## Dependencies

```toml
[dependencies]
clap = { version = "4.5", features = ["derive", "cargo"] }
pulldown-cmark = { version = "0.12", features = ["simd"] }
syntect = { version = "5.2", default-features = false, features = ["default-fancy"] }
open = "5"
tempfile = "3"
anyhow = "1"
thiserror = "2"
atty = "0.2"
```

## Commit Convention

Follow the standard format:

| Type | Use |
|------|-----|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `docs:` | Documentation |
| `test:` | Tests |
| `refactor:` | Code restructure |
| `chore:` | Maintenance |

## Resources

- [pulldown-cmark docs](https://docs.rs/pulldown-cmark)
- [syntect docs](https://docs.rs/syntect)
- [clap derive tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
- [GitHub Markdown CSS](https://github.com/sindresorhus/github-markdown-css)
