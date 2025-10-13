use super::enums::{Markdown, MarkdownErrorType};

/// The main tree of the Markdown parser.
/// 
/// ### Example:
/// ```
/// use telemark::parser::types::Node;
/// use telemark::parser::enums::Markdown;
/// 
/// let mut md_tree = Node::new(Markdown::Document);
/// md_tree.add_inner(
///     Node::new(
///         Markdown::Bold("Bold Text".to_string())
///     )
/// );
/// assert_eq!(md_tree.inner().len(), 1);
/// assert_eq!(
///     md_tree.inner()[0], 
///     Node::new(
///         Markdown::Bold("Bold Text".to_string())
///     )
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    value: Markdown,
    inner: Vec<Node>
}
impl Node {
    /// Create new `Node`.
    pub fn new(value: Markdown) -> Self {
        return Self {
            value,
            inner: Vec::new()
        };
    }

    /// Add node to the inner list.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::parser::types::Node;
    /// use telemark::parser::enums::Markdown;
    /// 
    /// let mut md_tree = Node::new(Markdown::Document);
    /// md_tree.add_inner(
    ///     Node::new(
    ///         Markdown::Bold("Bold Text".to_string())
    ///     )
    /// );
    /// assert_eq!(md_tree.inner().len(), 1);
    /// ```
    pub fn add_inner(&mut self, node: Node)  {
        self.inner.push(node);
    }

    /// Get the node value.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::parser::types::Node;
    /// use telemark::parser::enums::Markdown;
    /// 
    /// let mut md_tree = Node::new(Markdown::Document);
    /// assert_eq!(md_tree.value(), &Markdown::Document);
    /// ```
    pub fn value(&self) -> &Markdown {
        return &self.value;
    }

    /// Get the inner list.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::parser::types::Node;
    /// use telemark::parser::enums::Markdown;
    /// 
    /// let mut md_tree = Node::new(Markdown::Document);
    /// md_tree.add_inner(
    ///     Node::new(
    ///         Markdown::Bold("Bold Text".to_string())
    ///     )
    /// );
    /// assert_eq!(md_tree.inner().len(), 1);
    /// ```
    pub fn inner(&self) -> &Vec<Node> {
        return &self.inner;
    }
}


/// The Markdown error.
/// 
/// ### Example:
/// ```
/// use telemark::parser::types::MarkdownError;
/// use telemark::parser::enums::MarkdownErrorType;
/// 
/// let md_err = MarkdownError::new(MarkdownErrorType::StarOpen, 0);
/// assert_eq!(md_err.err(), &MarkdownErrorType::StarOpen);
/// assert_eq!(md_err.offset(), 0);
/// ```
#[derive(Debug, PartialEq)]
pub struct MarkdownError {
    err: MarkdownErrorType,
    offset: usize
}
impl MarkdownError {
    /// Create new `MarkdownError`.
    pub fn new(err: MarkdownErrorType, offset: usize) -> Self {
        return Self {
            err,
            offset
        }
    }

    /// Get the error type.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::parser::types::MarkdownError;
    /// use telemark::parser::enums::MarkdownErrorType;
    /// 
    /// let md_err = MarkdownError::new(MarkdownErrorType::StarOpen, 0);
    /// assert_eq!(md_err.err(), &MarkdownErrorType::StarOpen);
    /// ```
    pub fn err(&self) -> &MarkdownErrorType {
        return &self.err;
    }

    /// Get the error position.
    /// 
    /// ### Example:
    /// ```
    /// use telemark::parser::types::MarkdownError;
    /// use telemark::parser::enums::MarkdownErrorType;
    /// 
    /// let md_err = MarkdownError::new(MarkdownErrorType::StarOpen, 0);
    /// assert_eq!(md_err.offset(), 0);
    /// ```
    pub fn offset(&self) -> usize {
        return self.offset;
    }
}