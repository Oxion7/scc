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
