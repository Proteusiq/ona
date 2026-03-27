use anyhow::{Context, Result};
use clap::Parser;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser as MdParser, Tag, TagEnd};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use tempfile::NamedTempFile;

mod template;

/// Ona - Transform markdown into beautiful HTML
///
/// A fast, elegant CLI tool that renders markdown with stunning
/// typography and opens it in your default browser.
#[derive(Parser, Debug)]
#[command(name = "ona", version, about, long_about = None)]
#[command(author = "Prayson Wilfred Daniel <praysonpi@gmail.com>")]
#[command(propagate_version = true)]
struct Cli {
    /// Path to a markdown file (use '-' for stdin)
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    /// Theme mode: auto, light, or dark
    #[arg(short, long, default_value = "auto", value_parser = ["auto", "light", "dark"])]
    theme: String,

    /// Output to file instead of opening in browser
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Print the generated HTML to stdout
    #[arg(short, long)]
    print: bool,

    /// Title for the HTML page
    #[arg(long)]
    title: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Read markdown content
    let (markdown, title) = read_input(&cli)?;

    // Parse and render markdown to HTML
    let html_content = render_markdown(&markdown)?;

    // Generate full HTML page
    let full_html = template::generate_html(&html_content, &title, &cli.theme);

    // Output handling
    if cli.print {
        println!("{}", full_html);
    } else if let Some(output_path) = cli.output {
        fs::write(&output_path, &full_html)
            .with_context(|| format!("Failed to write to {}", output_path.display()))?;
        eprintln!("Wrote HTML to {}", output_path.display());
    } else {
        // Create temp file and open in browser
        let temp_file = NamedTempFile::with_suffix(".html")?;
        fs::write(temp_file.path(), &full_html)?;

        // Keep temp file alive by leaking it (it will be cleaned up when browser closes)
        let temp_path = temp_file.into_temp_path();
        let temp_path_kept = temp_path.keep()?;

        open::that(&temp_path_kept).with_context(|| "Failed to open browser")?;

        eprintln!("Opened in browser: {}", temp_path_kept.display());
    }

    Ok(())
}

fn read_input(cli: &Cli) -> Result<(String, String)> {
    let (content, filename) = match &cli.input {
        Some(path) if path.to_string_lossy() == "-" => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read from stdin")?;
            (buffer, "stdin".to_string())
        }
        Some(path) => {
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read file: {}", path.display()))?;
            let filename = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "Document".to_string());
            (content, filename)
        }
        None => {
            // If no input, check if stdin has data (piped)
            if atty::is(atty::Stream::Stdin) {
                // No input and no pipe, show help
                eprintln!("No input provided. Use 'ona --help' for usage information.");
                eprintln!("\nExamples:");
                eprintln!("  ona README.md          # Open markdown file in browser");
                eprintln!("  cat file.md | ona -    # Read from stdin");
                eprintln!("  ona -o out.html doc.md # Save to file");
                std::process::exit(1);
            } else {
                let mut buffer = String::new();
                io::stdin()
                    .read_to_string(&mut buffer)
                    .context("Failed to read from stdin")?;
                (buffer, "Document".to_string())
            }
        }
    };

    // Use custom title if provided, otherwise use filename
    let title = cli.title.clone().unwrap_or(filename);

    Ok((content, title))
}

fn render_markdown(markdown: &str) -> Result<String> {
    let mut options = Options::all();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = MdParser::new_ext(markdown, options);

    // We need to handle code blocks specially for syntax highlighting
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let mut in_code_block = false;
    let mut code_buffer = String::new();
    let mut code_lang = String::new();

    let mut events: Vec<Event> = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                code_buffer.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;

                // Highlight the code
                let highlighted = highlight_code(&code_buffer, &code_lang, &ss, &ts);
                events.push(Event::Html(highlighted.into()));

                code_buffer.clear();
                code_lang.clear();
            }
            Event::Text(text) if in_code_block => {
                code_buffer.push_str(&text);
            }
            _ => {
                events.push(event);
            }
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());

    Ok(html_output)
}

fn highlight_code(code: &str, lang: &str, ss: &SyntaxSet, ts: &ThemeSet) -> String {
    let lang = if lang.is_empty() { "txt" } else { lang };

    // Try to find the syntax, fall back to plain text
    let syntax = ss
        .find_syntax_by_token(lang)
        .or_else(|| ss.find_syntax_by_extension(lang))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    // Use a theme that works well with both light and dark modes
    // We'll use CSS variables to handle the theming
    let theme = &ts.themes["base16-ocean.dark"];

    // Generate the code content - either highlighted or plain
    let code_html = match highlighted_html_for_string(code, ss, syntax, theme) {
        Ok(highlighted) => highlighted,
        Err(_) => format!("<pre><code>{}</code></pre>", html_escape(code)),
    };

    // Wrap in our custom code block structure with copy button
    format!(
        r#"<div class="code-block">
<div class="code-header">
    <span class="code-lang">{lang}</span>
    <button class="copy-btn" onclick="copyCode(this)" aria-label="Copy code">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
        </svg>
    </button>
</div>
<div class="code-content">{code_html}</div>
</div>"#,
        lang = lang,
        code_html = code_html,
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
