

/// The Markdown types.
#[derive(Debug, Clone, PartialEq)]
pub enum Markdown {
    /// The start of the document.
    Document,
    /// Any Text, for example: `This is text`.
    Text(String),
    /// A Bold text, for example: `*bold text*`.
    Bold(String),
    /// A Italic text. for example: `_italic text_`.
    Italic(String),
    /// A Fixed Width Code, for example: \`fixed code\`.
    FixedWidthCode(String),
    /// A Pre-Fixed Width Code, for example: \`\`\`Pre-Fixed Width Code\`\`\`.
    PreFormattedFixedWidthCode {
        /// The language, for example: `Rust`.
        lang: Option<String>,
        /// The code, for example: `\nprintln!("hello");\n`.
        code: String
    },
    /// An Inline URL, for example: `[inline URL](https://example.com)`.
    InlineURL {
        /// The URL title, for example: `inline URL`.
        title: String,
        /// The URL, for example: `https://example.com`.
        url: String
    },
}


/// The Markdown errors.
#[derive(Debug, PartialEq)]
pub enum MarkdownErrorType {
    /// Star (`*`) is opne.
    StarOpen,
    /// Underscore (`_`) is opne.
    UnderscoreOpen,
    /// Backtick (\`) is opne.
    BacktickOpen,
    /// Backticks (\`\`\`) is opne.
    BackticksOpen,
    /// Square Brackets (`[`) is opne.
    SquareBracketsOpen,
    /// Parentheses (`(`) is opne.
    ParenthesesOpen
}