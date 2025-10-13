use super::tokens::MDTokensType;

/// The Markdown Token Node.
#[derive(PartialEq, Debug, Clone)]
pub struct MDToken<'a> {
    token_type: MDTokensType,
    value: &'a str,
    offset: usize
}
impl<'a> MDToken<'a> {
    /// Create new `MDToken`.
    pub fn new(token_type: MDTokensType, value: &'a str, offset: usize) -> Self {
        return Self {
            token_type,
            value,
            offset
        };
    }

    /// Get the token type.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::lexer::types::MDToken;
    /// use telemark::lexer::tokens::MDTokensType;
    /// 
    /// let token = MDToken::new(MDTokensType::Text, "text", 0);
    /// assert_eq!(token.token_type(), &MDTokensType::Text);
    /// ```
    pub fn token_type(&self) -> &MDTokensType {
        return &self.token_type ;
    }

    /// Get the token value.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::lexer::types::MDToken;
    /// use telemark::lexer::tokens::MDTokensType;
    /// 
    /// let token = MDToken::new(MDTokensType::Text, "text", 0);
    /// assert_eq!(token.value(), "text");
    /// ```
    pub fn value(&self) -> &'a str {
        return self.value;
    }

    /// Get the token offset.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::lexer::types::MDToken;
    /// use telemark::lexer::tokens::MDTokensType;
    /// 
    /// let token = MDToken::new(MDTokensType::Text, "text", 0);
    /// assert_eq!(token.offset(), 0);
    /// ```
    pub fn offset(&self) -> usize {
        return self.offset;
    }
}