use super::tokens::MDTokensType;

/// The Markdown Token Node.
#[derive(PartialEq, Debug, Clone)]
pub struct MDToken<'a> {
    token_type: MDTokensType,
    value: &'a str,
}
impl<'a> MDToken<'a> {
    /// Create new `MDToken`.
    pub fn new(token_type: MDTokensType, value: &'a str) -> Self {
        return Self {
            token_type,
            value
        };
    }
}