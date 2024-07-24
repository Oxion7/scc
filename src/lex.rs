use std::fs::File;
use std::io::Read;
use crate::ast::*;
/// Lexes the contents of the given file into a vector of tokens.
///
/// # Arguments
///
/// * `file` - A `File` object representing the file to be lexed.
///
/// # Returns
///
/// A vector of `Token` objects representing the lexed tokens from the input file.
pub fn lex (mut file: File) -> Vec<Token> {
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");

    let mut tokens = Vec::new();
    let mut chars = contents.chars().peekable();
    while let Some(&ch) = chars.peek() {
        match ch {
            '{' => {
                tokens.push(Token::OpenBrace);
                chars.next();
            },
            '}' => {
                tokens.push(Token::CloseBrace);
                chars.next();
            },
            '(' => {
                tokens.push(Token::OpenParenthesis);
                chars.next();
            },
            ')' => {
                tokens.push(Token::CloseParenthesis);
                chars.next();
            },
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            },
            '-' => {
                tokens.push(Token::Negation);
                chars.next();
            }
            '~' => {
                tokens.push(Token::BitwiseComplement);
                chars.next();
            }
            '!' => {
                tokens.push(Token::LogicalNegation);
                chars.next();
            }
            '/' => {
                chars.next();
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '/' => {
                            // Skip single-line comment
                            chars.next();
                            while let Some(&ch) = chars.peek() {
                                if ch == '\n' {
                                    break;
                                }
                                chars.next();
                            }
                        }
                        '*' => {
                            // Skip multi-line comment
                            chars.next();
                            while let Some(ch) = chars.next() {
                                if ch == '*' {
                                    if let Some(&next_ch) = chars.peek() {
                                        if next_ch == '/' {
                                            chars.next();
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            // Handle division or invalid character
                            panic!("Unexpected character after '/': {:?}", next_ch);
                        }
                    }
                }
            }
            'i' => {
                if chars.clone().collect::<String>().starts_with("int") {
                    //skip 3 and push
                    for _ in 0..3 {
                        chars.next();
                    }
                    tokens.push(Token::IntKeyword);
                }else {
                    lex_identifier_or_keyword(&mut chars, &mut tokens);
                }
            },
            'r' => {
                if chars.clone().collect::<String>().starts_with("return") {
                    for _ in 0..6 {
                        chars.next();
                    }
                    tokens.push(Token::ReturnKeyword);
                }else {
                    lex_identifier_or_keyword(&mut chars, &mut tokens);
                }
            },
            c if c.is_digit(10) => {
                lex_integer_literal(&mut chars, &mut tokens);
            },
            c if c.is_alphanumeric() || c == '_' => {
                lex_identifier_or_keyword(&mut chars, &mut tokens);
            },
            ' ' | '\n' | '\r' => {
                chars.next();
            },
            _ => {
                panic!("Unexpected character: {:?}", ch);
            }
        }
    }
    return tokens;
}

fn lex_identifier_or_keyword(chars: &mut std::iter::Peekable<std::str::Chars>, tokens: &mut Vec<Token>) {
    let mut identifier = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            identifier.push(ch);
            chars.next();
        }else{
            break;
        }
    }
    tokens.push(Token::Identifier(identifier));
}

fn lex_integer_literal(chars: &mut std::iter::Peekable<std::str::Chars>, tokens: &mut Vec<Token>) {
    let mut number = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_digit(10) {
            number.push(ch);
            chars.next();
        }else{
            break;
        }
    }
    tokens.push(Token::IntegerLiteral(number));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::io::Seek;
    use std::io::SeekFrom;

    fn create_temp_file(content: &str) -> File {
        let mut file = tempfile::tempfile().expect("Could not create temp file");
        file.write_all(content.as_bytes()).expect("Could not write to temp file");
        file.seek(SeekFrom::Start(0)).expect("Could not seek to start of temp file");
        file
    }

    #[test]
    fn test_empty_file() {
        let file = create_temp_file("");
        let tokens = lex(file);
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_single_tokens() {
        let file = create_temp_file("{ } ( ) ; int return - ~ !");
        let tokens = lex(file);
        let expected = vec![
            Token::OpenBrace,
            Token::CloseBrace,
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::Semicolon,
            Token::IntKeyword,
            Token::ReturnKeyword,
            Token::Negation,
            Token::BitwiseComplement,
            Token::LogicalNegation,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_identifier_and_integer_literal() {
        let file = create_temp_file("foo 123");
        let tokens = lex(file);
        let expected = vec![
            Token::Identifier("foo".to_string()),
            Token::IntegerLiteral("123".to_string()),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_mixed_tokens() {
        let file = create_temp_file("int main() { return 42; }");
        let tokens = lex(file);
        let expected = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral("42".to_string()),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comments() {
        let file = create_temp_file("int main() { // This is a comment\n return 42; /* This is another comment */ }");
        let tokens = lex(file);
        let expected = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral("42".to_string()),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_unexpected_character() {
        let file = create_temp_file("int main() { return 42 @; }");
        let result = std::panic::catch_unwind(|| {
            lex(file);
        });
        assert!(result.is_err());
    }
}
