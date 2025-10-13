use crate::lexer::mdv1;
use crate::lexer::types::MDToken;
use crate::lexer::tokens::MDTokensType;
use crate::reader::VecReader;

use super::types::{MarkdownError, Node};
use super::enums::{Markdown, MarkdownErrorType};

/// The Markdown V1 parser. this function use the markdown v1 lexer, it will
/// lex the document and then parse it to an AST.
/// 
/// ### Example: 
/// ```
/// use telemark::parser::mdv1;
/// use telemark::parser::enums::Markdown;
/// 
/// if let Ok(ast) = mdv1::parser("*bold*") {
///     assert_eq!(ast.value(), &Markdown::Document);
///     assert_eq!(ast.inner().len(), 1);
///     assert_eq!( 
///         ast.inner()[0].value(), 
///         &Markdown::Bold("bold".to_string())
///     );
/// }
/// 
/// use telemark::parser::enums::MarkdownErrorType;
/// // This is an error:
/// if let Err(err) = mdv1::parser("*bold") {
///     assert_eq!(err.err(), &MarkdownErrorType::StarOpen);
///     assert_eq!(err.offset(), 0);
/// }
/// ```
pub fn parser(document: &str) -> Result<Node, MarkdownError> {
    let mut tokens: VecReader<MDToken<'_>> = VecReader::new(mdv1::lex(document));
    let mut node = Node::new(Markdown::Document);
    let mut escaped_count: usize = 0;
    let mut _start: usize = 0; // for the errors position
    while let Some(t) = tokens.next() {
        _start = t.offset();
        match t.token_type() {
            MDTokensType::Escape => {
                escaped_count += 1;
                let escaped = [
                    MDTokensType::Star, MDTokensType::Underscore, MDTokensType::Backtick, 
                    MDTokensType::Backticks, MDTokensType::SquareBracketsOpen
                ];
                if let Some(nt) = tokens.next() {
                    if let Some(_) = escaped.iter().find(|&st| st == nt.token_type()) {
                        if escaped_count%2 != 0 {
                            node.add_inner(
                                Node::new(
                                    Markdown::Text(
                                        nt.value().to_string()
                                    )
                                )
                            );
                            escaped_count = 0;
                        } else {
                            tokens.go_back();
                        }
                    }
                }
                continue;
            },
            MDTokensType::Star => node.add_inner(
                Node::new(
                    Markdown::Bold(
                        parse_until(
                            &mut tokens, 
                            MDTokensType::Star, 
                            _start, 
                            MarkdownErrorType::StarOpen
                        )?
                    )
                )
            ),
            MDTokensType::Underscore => node.add_inner(
                Node::new(
                    Markdown::Italic(
                        parse_until(
                            &mut tokens, 
                            MDTokensType::Underscore,
                            _start,
                            MarkdownErrorType::UnderscoreOpen
                        )?
                    )
                )
            ),
            MDTokensType::Backtick => node.add_inner(
                Node::new(
                    Markdown::FixedWidthCode(
                        parse_until(
                            &mut tokens, 
                            MDTokensType::Backtick,
                            _start,
                            MarkdownErrorType::BacktickOpen
                        )?
                    )
                )
            ),
            MDTokensType::Backticks => {
                let value: String = parse_until(
                    &mut tokens, 
                    MDTokensType::Backticks,
                    _start,
                    MarkdownErrorType::BackticksOpen
                )?;
                let lines: Vec<&str> = value.split('\n').collect();
                // ```\n code \n```
                if lines.len() > 1 {
                    node.add_inner(Node::new(
                        Markdown::PreFormattedFixedWidthCode { 
                            lang: if lines[0].len() > 0 { Some(lines[0].to_string()) } else { None }, 
                            code: lines[1..].join("\n").to_string()
                        }
                    ));
                } 
                // ``` code ```
                else {
                    node.add_inner(Node::new(
                        Markdown::PreFormattedFixedWidthCode { 
                            lang: None, 
                            code: value
                        }
                    ));
                }
            },
            MDTokensType::SquareBracketsOpen => {
                let title = parse_until(
                    &mut tokens, 
                    MDTokensType::SquareBracketsClose,
                    _start,
                    MarkdownErrorType::SquareBracketsOpen
                )?;
                if let Some(nt) = tokens.next() {
                    if *nt.token_type() == MDTokensType::ParenthesesOpen {
                        node.add_inner(Node::new(
                            Markdown::InlineURL { 
                                title, 
                                url: parse_until(
                                    &mut tokens, 
                                    MDTokensType::ParenthesesClose,
                                    _start,
                                    MarkdownErrorType::ParenthesesOpen
                                )?
                            }
                        ));
                    }
                    // It is not Inline URL
                    else {
                        tokens.go_back();
                        node.add_inner(Node::new(Markdown::Text(title)));
                    }
                }
                // It is not Inline URL
                else {
                    node.add_inner(Node::new(Markdown::Text(title)));
                }
            },
            MDTokensType::ParenthesesOpen => node.add_inner(
                Node::new(
                    Markdown::Text(
                        format!(
                            "({})", 
                            parse_until(
                                &mut tokens, 
                                MDTokensType::ParenthesesClose,
                                _start,
                                MarkdownErrorType::ParenthesesOpen
                            )?
                        )
                    )
                )
            ),
            _ => node.add_inner(
                Node::new(
                    Markdown::Text(
                        t.value().to_string()
                    )
                )
            )
        }
        escaped_count = 0;
    }
    return Ok(node);
}

fn parse_until(
    tokens: &mut VecReader<MDToken<'_>>, 
    to: MDTokensType, 
    start: usize, 
    err: MarkdownErrorType
) -> Result<String, MarkdownError> {
    let mut value = String::new();
    let mut found = false;
    let mut escaped_counter: usize = 0;
    while let Some(t) = tokens.next() {
        if *t.token_type() == MDTokensType::Escape {
            escaped_counter += 1;
            continue;
        } else if *t.token_type() == to && escaped_counter%2 == 0 {
            found = true; 
            break;
        }
        value += t.value();
        escaped_counter = 0;
    }
    if !found {
        return Err(
            MarkdownError::new(
                err, 
                start
            )
        )
    }
    return Ok(value);
}