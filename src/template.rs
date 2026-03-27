/// Generate the full HTML document with beautiful styling
pub fn generate_html(content: &str, title: &str, theme: &str) -> String {
    let theme_class = match theme {
        "light" => "light",
        "dark" => "dark",
        _ => "", // auto - uses prefers-color-scheme
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="en" class="{theme_class}" data-theme="{theme_data}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="color-scheme" content="light dark">
    <title>{title}</title>
    <style>
{CSS}
    </style>
</head>
<body>
    <div class="theme-toggle" role="group" aria-label="Theme selection">
        <button class="theme-btn" data-theme="light" aria-label="Light theme" title="Light">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="5"></circle>
                <line x1="12" y1="1" x2="12" y2="3"></line>
                <line x1="12" y1="21" x2="12" y2="23"></line>
                <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                <line x1="1" y1="12" x2="3" y2="12"></line>
                <line x1="21" y1="12" x2="23" y2="12"></line>
                <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
            </svg>
        </button>
        <button class="theme-btn" data-theme="auto" aria-label="Auto theme" title="Auto">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <path d="M12 2a10 10 0 0 0 0 20"></path>
            </svg>
        </button>
        <button class="theme-btn" data-theme="dark" aria-label="Dark theme" title="Dark">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
            </svg>
        </button>
    </div>
    <article class="markdown-body">
{content}
    </article>
    <script>
{JS}
    </script>
</body>
</html>"##,
        theme_class = theme_class,
        theme_data = if theme_class.is_empty() {
            "auto"
        } else {
            theme_class
        },
        title = html_escape(title),
        content = content,
        CSS = CSS,
        JS = JS,
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const CSS: &str = r##"
/* ========================================
   Ona - Beautiful Markdown Styles
   Inspired by GitHub, Notion, and Linear
   ======================================== */

:root {
    /* Light theme colors */
    --color-bg: #ffffff;
    --color-bg-secondary: #f6f8fa;
    --color-bg-tertiary: #f0f2f5;
    --color-text: #1f2328;
    --color-text-secondary: #656d76;
    --color-text-tertiary: #8b949e;
    --color-border: #d0d7de;
    --color-border-light: #e8ebef;
    --color-link: #0969da;
    --color-link-hover: #0550ae;
    --color-code-bg: #f6f8fa;
    --color-code-text: #1f2328;
    --color-blockquote-border: #d0d7de;
    --color-blockquote-text: #57606a;
    --color-table-border: #d0d7de;
    --color-table-row-alt: #f6f8fa;
    --color-kbd-bg: #f6f8fa;
    --color-kbd-border: #d0d7de;
    --color-mark-bg: #fff8c5;
    --color-hr: #d8dee4;
    --color-shadow: rgba(31, 35, 40, 0.04);
    --color-shadow-medium: rgba(31, 35, 40, 0.08);
    
    /* Code block colors - light */
    --code-bg: #1e2128;
    --code-text: #e6edf3;
    --code-header-bg: #2d333b;
    --code-lang: #8b949e;
    --code-copy-hover: rgba(255, 255, 255, 0.1);
    
    /* Syntax highlighting - light */
    --syntax-comment: #8b949e;
    --syntax-keyword: #cf222e;
    --syntax-string: #0a3069;
    --syntax-number: #0550ae;
    --syntax-function: #8250df;
    --syntax-variable: #953800;
    --syntax-operator: #cf222e;
    --syntax-class: #953800;
    
    /* Typography */
    --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
    --font-mono: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono", monospace;
    
    /* Spacing */
    --content-width: 860px;
    --content-padding: 48px;
    --content-padding-mobile: 24px;
}

/* Dark theme */
@media (prefers-color-scheme: dark) {
    :root:not(.light) {
        --color-bg: #0d1117;
        --color-bg-secondary: #161b22;
        --color-bg-tertiary: #21262d;
        --color-text: #e6edf3;
        --color-text-secondary: #8b949e;
        --color-text-tertiary: #6e7681;
        --color-border: #30363d;
        --color-border-light: #21262d;
        --color-link: #58a6ff;
        --color-link-hover: #79c0ff;
        --color-code-bg: #161b22;
        --color-code-text: #e6edf3;
        --color-blockquote-border: #30363d;
        --color-blockquote-text: #8b949e;
        --color-table-border: #30363d;
        --color-table-row-alt: #161b22;
        --color-kbd-bg: #161b22;
        --color-kbd-border: #30363d;
        --color-mark-bg: #bb800926;
        --color-hr: #30363d;
        --color-shadow: rgba(0, 0, 0, 0.2);
        --color-shadow-medium: rgba(0, 0, 0, 0.3);
        
        /* Code block colors - dark */
        --code-bg: #161b22;
        --code-text: #e6edf3;
        --code-header-bg: #21262d;
        --code-lang: #8b949e;
        
        /* Syntax highlighting - dark */
        --syntax-comment: #8b949e;
        --syntax-keyword: #ff7b72;
        --syntax-string: #a5d6ff;
        --syntax-number: #79c0ff;
        --syntax-function: #d2a8ff;
        --syntax-variable: #ffa657;
        --syntax-operator: #ff7b72;
        --syntax-class: #ffa657;
    }
}

/* Forced dark theme */
html.dark {
    --color-bg: #0d1117;
    --color-bg-secondary: #161b22;
    --color-bg-tertiary: #21262d;
    --color-text: #e6edf3;
    --color-text-secondary: #8b949e;
    --color-text-tertiary: #6e7681;
    --color-border: #30363d;
    --color-border-light: #21262d;
    --color-link: #58a6ff;
    --color-link-hover: #79c0ff;
    --color-code-bg: #161b22;
    --color-code-text: #e6edf3;
    --color-blockquote-border: #30363d;
    --color-blockquote-text: #8b949e;
    --color-table-border: #30363d;
    --color-table-row-alt: #161b22;
    --color-kbd-bg: #161b22;
    --color-kbd-border: #30363d;
    --color-mark-bg: #bb800926;
    --color-hr: #30363d;
    --color-shadow: rgba(0, 0, 0, 0.2);
    --color-shadow-medium: rgba(0, 0, 0, 0.3);
    --code-bg: #161b22;
    --code-text: #e6edf3;
    --code-header-bg: #21262d;
    --code-lang: #8b949e;
    --syntax-comment: #8b949e;
    --syntax-keyword: #ff7b72;
    --syntax-string: #a5d6ff;
    --syntax-number: #79c0ff;
    --syntax-function: #d2a8ff;
    --syntax-variable: #ffa657;
    --syntax-operator: #ff7b72;
    --syntax-class: #ffa657;
}

/* Forced light theme */
html.light {
    --color-bg: #ffffff;
    --color-bg-secondary: #f6f8fa;
    --color-bg-tertiary: #f0f2f5;
    --color-text: #1f2328;
    --color-text-secondary: #656d76;
    --color-text-tertiary: #8b949e;
    --color-border: #d0d7de;
    --color-border-light: #e8ebef;
    --color-link: #0969da;
    --color-link-hover: #0550ae;
    --color-code-bg: #f6f8fa;
    --color-code-text: #1f2328;
    --color-blockquote-border: #d0d7de;
    --color-blockquote-text: #57606a;
    --color-table-border: #d0d7de;
    --color-table-row-alt: #f6f8fa;
    --color-kbd-bg: #f6f8fa;
    --color-kbd-border: #d0d7de;
    --color-mark-bg: #fff8c5;
    --color-hr: #d8dee4;
    --color-shadow: rgba(31, 35, 40, 0.04);
    --color-shadow-medium: rgba(31, 35, 40, 0.08);
    --code-bg: #1e2128;
    --code-text: #e6edf3;
    --code-header-bg: #2d333b;
    --code-lang: #8b949e;
    --syntax-comment: #8b949e;
    --syntax-keyword: #cf222e;
    --syntax-string: #0a3069;
    --syntax-number: #0550ae;
    --syntax-function: #8250df;
    --syntax-variable: #953800;
    --syntax-operator: #cf222e;
    --syntax-class: #953800;
}

/* Reset & Base */
*, *::before, *::after {
    box-sizing: border-box;
}

html {
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-rendering: optimizeLegibility;
    font-feature-settings: "kern" 1, "liga" 1;
}

body {
    margin: 0;
    padding: 0;
    background-color: var(--color-bg);
    color: var(--color-text);
    font-family: var(--font-sans);
    font-size: 16px;
    line-height: 1.6;
    transition: background-color 0.2s ease, color 0.2s ease;
}

/* Main content container */
.markdown-body {
    max-width: var(--content-width);
    margin: 0 auto;
    padding: var(--content-padding);
    word-wrap: break-word;
}

@media (max-width: 768px) {
    .markdown-body {
        padding: var(--content-padding-mobile);
    }
}

/* Typography */
.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
    margin-top: 1.5em;
    margin-bottom: 0.5em;
    font-weight: 600;
    line-height: 1.25;
    color: var(--color-text);
}

.markdown-body h1 {
    font-size: 2.25em;
    font-weight: 700;
    padding-bottom: 0.3em;
    border-bottom: 1px solid var(--color-border-light);
    margin-top: 0;
}

.markdown-body h2 {
    font-size: 1.65em;
    padding-bottom: 0.25em;
    border-bottom: 1px solid var(--color-border-light);
}

.markdown-body h3 {
    font-size: 1.35em;
}

.markdown-body h4 {
    font-size: 1.15em;
}

.markdown-body h5 {
    font-size: 1em;
}

.markdown-body h6 {
    font-size: 0.9em;
    color: var(--color-text-secondary);
}

.markdown-body p {
    margin-top: 0;
    margin-bottom: 1em;
}

.markdown-body > *:first-child {
    margin-top: 0 !important;
}

.markdown-body > *:last-child {
    margin-bottom: 0 !important;
}

/* Links */
.markdown-body a {
    color: var(--color-link);
    text-decoration: none;
    transition: color 0.15s ease;
}

.markdown-body a:hover {
    color: var(--color-link-hover);
    text-decoration: underline;
}

/* Lists */
.markdown-body ul,
.markdown-body ol {
    margin-top: 0;
    margin-bottom: 1em;
    padding-left: 2em;
}

.markdown-body ul ul,
.markdown-body ul ol,
.markdown-body ol ol,
.markdown-body ol ul {
    margin-top: 0.25em;
    margin-bottom: 0;
}

.markdown-body li {
    margin-bottom: 0.25em;
}

.markdown-body li + li {
    margin-top: 0.25em;
}

.markdown-body li > p {
    margin-top: 1em;
}

/* Task lists */
.markdown-body ul.contains-task-list {
    list-style-type: none;
    padding-left: 0;
}

.markdown-body li.task-list-item {
    position: relative;
    padding-left: 1.75em;
}

.markdown-body input[type="checkbox"] {
    position: absolute;
    left: 0;
    top: 0.3em;
    margin: 0;
    width: 1em;
    height: 1em;
    accent-color: var(--color-link);
}

/* Blockquotes */
.markdown-body blockquote {
    margin: 1em 0;
    padding: 0.5em 1em;
    color: var(--color-blockquote-text);
    border-left: 4px solid var(--color-blockquote-border);
    background-color: var(--color-bg-secondary);
    border-radius: 0 6px 6px 0;
}

.markdown-body blockquote > :first-child {
    margin-top: 0;
}

.markdown-body blockquote > :last-child {
    margin-bottom: 0;
}

/* Inline code */
.markdown-body code {
    padding: 0.2em 0.4em;
    margin: 0;
    font-size: 0.875em;
    font-family: var(--font-mono);
    background-color: var(--color-code-bg);
    color: var(--color-code-text);
    border-radius: 6px;
}

/* Code blocks */
.markdown-body .code-block {
    margin: 1.5em 0;
    border-radius: 8px;
    overflow: hidden;
    background-color: var(--code-bg);
    box-shadow: 0 2px 8px var(--color-shadow-medium);
}

.markdown-body .code-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    background-color: var(--code-header-bg);
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.markdown-body .code-lang {
    font-size: 12px;
    font-family: var(--font-mono);
    color: var(--code-lang);
    text-transform: lowercase;
    font-weight: 500;
}

.markdown-body .copy-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 6px;
    background: transparent;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    color: var(--code-lang);
    transition: background-color 0.15s ease, color 0.15s ease;
}

.markdown-body .copy-btn:hover {
    background-color: var(--code-copy-hover);
    color: var(--code-text);
}

.markdown-body .copy-btn.copied {
    color: #3fb950;
}

.markdown-body .code-content {
    margin: 0;
    padding: 0;
    overflow-x: auto;
    font-size: 14px;
    line-height: 1.5;
}

/* Style syntect-generated pre tags */
.markdown-body .code-content pre {
    margin: 0 !important;
    padding: 16px !important;
    background-color: var(--code-bg) !important;
    border-radius: 0 !important;
    overflow-x: auto;
}

.markdown-body .code-content code,
.markdown-body .code-content pre code {
    padding: 0;
    margin: 0;
    background: transparent !important;
    border-radius: 0;
    font-size: 14px;
    font-family: var(--font-mono);
    line-height: 1.5;
    white-space: pre;
}

/* Ensure syntect span colors are visible */
.markdown-body .code-content span {
    font-family: var(--font-mono);
}

/* Fallback pre/code without wrapper */
.markdown-body pre {
    margin: 1.5em 0;
    padding: 16px;
    overflow-x: auto;
    font-size: 14px;
    line-height: 1.5;
    background-color: var(--code-bg);
    border-radius: 8px;
}

.markdown-body pre > code {
    padding: 0;
    margin: 0;
    font-size: inherit;
    color: var(--code-text);
    background-color: transparent;
    border-radius: 0;
    white-space: pre;
}

/* Tables */
.markdown-body table {
    width: 100%;
    margin: 1.5em 0;
    border-collapse: collapse;
    border-spacing: 0;
    overflow: auto;
    display: block;
}

.markdown-body table th,
.markdown-body table td {
    padding: 10px 16px;
    border: 1px solid var(--color-table-border);
}

.markdown-body table th {
    font-weight: 600;
    background-color: var(--color-bg-secondary);
    text-align: left;
}

.markdown-body table tr:nth-child(2n) {
    background-color: var(--color-table-row-alt);
}

/* Horizontal rules */
.markdown-body hr {
    height: 1px;
    margin: 2em 0;
    padding: 0;
    background-color: var(--color-hr);
    border: 0;
}

/* Images */
.markdown-body img {
    max-width: 100%;
    height: auto;
    border-radius: 8px;
    display: block;
    margin: 1.5em 0;
    box-shadow: 0 2px 12px var(--color-shadow-medium);
}

.markdown-body img[align="left"] {
    float: left;
    margin-right: 1.5em;
}

.markdown-body img[align="right"] {
    float: right;
    margin-left: 1.5em;
}

/* Keyboard */
.markdown-body kbd {
    display: inline-block;
    padding: 3px 8px;
    font-size: 11px;
    line-height: 1.4;
    font-family: var(--font-mono);
    color: var(--color-text);
    vertical-align: middle;
    background-color: var(--color-kbd-bg);
    border: 1px solid var(--color-kbd-border);
    border-radius: 6px;
    box-shadow: inset 0 -1px 0 var(--color-kbd-border);
}

/* Mark/Highlight */
.markdown-body mark {
    background-color: var(--color-mark-bg);
    padding: 0.1em 0.3em;
    border-radius: 4px;
}

/* Abbreviations */
.markdown-body abbr[title] {
    text-decoration: underline dotted;
    cursor: help;
    border-bottom: none;
}

/* Subscript/Superscript */
.markdown-body sub,
.markdown-body sup {
    font-size: 75%;
    line-height: 0;
    position: relative;
    vertical-align: baseline;
}

.markdown-body sub {
    bottom: -0.25em;
}

.markdown-body sup {
    top: -0.5em;
}

/* Definition lists */
.markdown-body dl {
    margin: 1em 0;
    padding: 0;
}

.markdown-body dt {
    font-weight: 600;
    margin-top: 1em;
}

.markdown-body dd {
    margin-left: 2em;
    margin-bottom: 0.5em;
}

/* Footnotes */
.markdown-body .footnote-definition {
    margin: 1.5em 0;
    padding-left: 2em;
    font-size: 0.9em;
    color: var(--color-text-secondary);
}

.markdown-body .footnote-definition-label {
    position: absolute;
    margin-left: -2em;
    font-weight: 600;
}

.markdown-body .footnote-reference {
    font-size: 0.8em;
    vertical-align: super;
}

/* Strikethrough */
.markdown-body del {
    color: var(--color-text-secondary);
}

/* Details/Summary */
.markdown-body details {
    margin: 1em 0;
    padding: 0.5em 1em;
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 8px;
}

.markdown-body summary {
    font-weight: 600;
    cursor: pointer;
    padding: 0.25em 0;
}

.markdown-body details[open] > summary {
    margin-bottom: 0.5em;
}

/* Theme Toggle */
.theme-toggle {
    position: fixed;
    top: 20px;
    right: 20px;
    display: flex;
    gap: 4px;
    padding: 4px;
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 10px;
    box-shadow: 0 2px 8px var(--color-shadow-medium);
    z-index: 1000;
    transition: background-color 0.2s ease, border-color 0.2s ease;
}

.theme-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    padding: 0;
    background: transparent;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    color: var(--color-text-secondary);
    transition: background-color 0.15s ease, color 0.15s ease, transform 0.1s ease;
}

.theme-btn:hover {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text);
}

.theme-btn:active {
    transform: scale(0.95);
}

.theme-btn.active {
    background-color: var(--color-link);
    color: white;
}

.theme-btn svg {
    width: 18px;
    height: 18px;
}

@media (max-width: 768px) {
    .theme-toggle {
        top: 12px;
        right: 12px;
    }
    
    .theme-btn {
        width: 32px;
        height: 32px;
    }
    
    .theme-btn svg {
        width: 16px;
        height: 16px;
    }
}

/* Animations */
@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(8px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.markdown-body {
    animation: fadeIn 0.3s ease-out;
}

/* Print styles */
@media print {
    body {
        background: white;
        color: black;
    }
    
    .theme-toggle {
        display: none;
    }
    
    .markdown-body {
        max-width: none;
        padding: 1em;
    }
    
    .markdown-body .code-block {
        box-shadow: none;
        border: 1px solid #ddd;
    }
    
    .markdown-body .copy-btn {
        display: none;
    }
    
    .markdown-body a {
        color: black;
        text-decoration: underline;
    }
    
    .markdown-body a[href^="http"]::after {
        content: " (" attr(href) ")";
        font-size: 0.8em;
        color: #666;
    }
}

/* Scrollbar styling */
::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

::-webkit-scrollbar-track {
    background: var(--color-bg-secondary);
    border-radius: 4px;
}

::-webkit-scrollbar-thumb {
    background: var(--color-border);
    border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
    background: var(--color-text-tertiary);
}

/* Selection */
::selection {
    background-color: var(--color-link);
    color: white;
}
"##;

const JS: &str = r##"
// Theme toggle functionality
(function() {
    const html = document.documentElement;
    const buttons = document.querySelectorAll('.theme-btn');
    const STORAGE_KEY = 'ona-theme';
    
    // Get initial theme from data attribute or localStorage
    function getInitialTheme() {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored) return stored;
        return html.dataset.theme || 'auto';
    }
    
    // Apply theme to document
    function applyTheme(theme) {
        html.dataset.theme = theme;
        
        if (theme === 'light') {
            html.classList.remove('dark');
            html.classList.add('light');
        } else if (theme === 'dark') {
            html.classList.remove('light');
            html.classList.add('dark');
        } else {
            // Auto - remove both classes, let prefers-color-scheme handle it
            html.classList.remove('light', 'dark');
        }
        
        // Update active button
        buttons.forEach(btn => {
            btn.classList.toggle('active', btn.dataset.theme === theme);
        });
        
        // Save preference
        localStorage.setItem(STORAGE_KEY, theme);
    }
    
    // Initialize
    const initialTheme = getInitialTheme();
    applyTheme(initialTheme);
    
    // Add click handlers
    buttons.forEach(btn => {
        btn.addEventListener('click', () => {
            applyTheme(btn.dataset.theme);
        });
    });
})();

// Copy code functionality
function copyCode(button) {
    const codeBlock = button.closest('.code-block');
    const code = codeBlock.querySelector('code');
    const text = code.textContent;
    
    navigator.clipboard.writeText(text).then(() => {
        button.classList.add('copied');
        button.innerHTML = `
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
        `;
        
        setTimeout(() => {
            button.classList.remove('copied');
            button.innerHTML = `
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                </svg>
            `;
        }, 2000);
    }).catch(err => {
        console.error('Failed to copy:', err);
    });
}

// Add smooth scroll for anchor links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            target.scrollIntoView({
                behavior: 'smooth',
                block: 'start'
            });
        }
    });
});

// Add heading anchors
document.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(heading => {
    if (heading.id) {
        const anchor = document.createElement('a');
        anchor.className = 'heading-anchor';
        anchor.href = '#' + heading.id;
        anchor.innerHTML = '#';
        anchor.style.cssText = `
            position: absolute;
            left: -1.5em;
            opacity: 0;
            color: var(--color-text-tertiary);
            text-decoration: none;
            font-weight: normal;
            transition: opacity 0.15s ease;
        `;
        heading.style.position = 'relative';
        heading.prepend(anchor);
        
        heading.addEventListener('mouseenter', () => anchor.style.opacity = '1');
        heading.addEventListener('mouseleave', () => anchor.style.opacity = '0');
    }
});
"##;
