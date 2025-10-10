
/// The Markdown V1 and V2 Tokens.
#[derive(PartialEq, Debug, Clone)]
pub enum MDTokensType {
    /// Any Text in the document
    Text,
    /// \
    Escape,
    /// *
    Star,
    /// **
    DoubleStar,
    /// _
    Underscore,
    /// __
    DoubleUnderscore,
    /// \`
    Backtick,
    /// \`\`\`
    Backticks,
    /// [
    SquareBracketsOpen,
    /// ]
    SquareBracketsClose,
    /// (
    ParenthesesOpen,
    /// )
    ParenthesesClose,
    /// ~
    Tilde,
    /// ||
    DoublePipe,
    /// !
    ExclamationMark,
    /// \>
    GreaterThan
}