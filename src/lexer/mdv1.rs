use super::types::MDToken;
use super::tokens::MDTokensType;
use crate::char_reader::CharReader;

/// The Markdown V1 Lexer.
/// 
/// ### Example:
/// ```
/// use telemark::lexer::mdv1;
/// use telemark::lexer::types::MDToken;
/// use telemark::lexer::tokens::MDTokensType;
/// 
/// let tokens = mdv1::lex("this is *bold*");
/// assert_eq!(tokens.len(), 4);
/// assert_eq!(tokens[0], MDToken::new(MDTokensType::Text, "this is "));
/// assert_eq!(tokens[1], MDToken::new(MDTokensType::Star, "*"));
/// assert_eq!(tokens[2], MDToken::new(MDTokensType::Text, "bold"));
/// assert_eq!(tokens[3], MDToken::new(MDTokensType::Star, "*"));
/// ```
pub fn lex(document: &str) -> Vec<MDToken> {
    let mut char_reader = CharReader::new(document);
    let mut tokens: Vec<MDToken> = Vec::new();
    while let Some(c) = char_reader.next_char() {
        match c {
            '\\' => tokens.push(MDToken::new(MDTokensType::Escape, "\\")),
            '*'  => tokens.push(MDToken::new(MDTokensType::Star, "*")),
            '_'  => tokens.push(MDToken::new(MDTokensType::Underscore, "_")),
            '['  => tokens.push(MDToken::new(MDTokensType::SquareBracketsOpen, "[")),
            ']'  => tokens.push(MDToken::new(MDTokensType::SquareBracketsClose, "]")),
            '('  => tokens.push(MDToken::new(MDTokensType::ParenthesesOpen, "(")),
            ')'  => tokens.push(MDToken::new(MDTokensType::ParenthesesClose, ")")),
            '`'  => {
                if let Some(ncs) = char_reader.get_string(3) {
                    if ncs.iter().collect::<String>() == "``" {
                        char_reader.next_char(); // Delete the next `
                        char_reader.next_char(); // Delete the next `
                        tokens.push(MDToken::new(MDTokensType::Backticks, "```"));
                        continue;
                    }
                }
                tokens.push(MDToken::new(MDTokensType::Backtick, "`"));
            }
            _ => {
                char_reader.go_back();
                let start_pos = char_reader.pos();
                move_to_text_end(&mut char_reader); // Moving the char_reader to the end of the text.
                tokens.push(MDToken::new(
                    MDTokensType::Text, 
                    &document[start_pos..char_reader.pos()]
                ));
            }
        }
    }
    return tokens;
}

/// Move the char_reader to the end of the text block.
/// The end of the text is any Markdown V1 Token.
fn move_to_text_end(char_reader: &mut CharReader) {
    let stops = ['\\', '*', '_', '`', '[', ']', '(', ')'];
    while let Some(c) = char_reader.next_char() {
        if let Some(_) = stops.iter().find(|&sc| sc == c) {
            break;
        }
    }
    char_reader.go_back();
}
