use crate::ast::*;

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut iter = tokens.into_iter().peekable();

    expect_token(&mut iter, Token::IntKeyword)?;
    let identifier = expect_identifier(&mut iter)?;
    expect_token(&mut iter, Token::OpenParenthesis)?;

    if let Some(Token::VoidKeyword) = iter.peek() {
        iter.next(); // Consume the void keyword
    } else if let Some(Token::CloseParenthesis) = iter.peek() {
        // No parameters, continue
    } else {
        return Err("Expected 'void' or ')' after '(".to_string());
    }
    expect_token(&mut iter, Token::CloseParenthesis)?;

    expect_token(&mut iter, Token::OpenBrace)?;
    expect_token(&mut iter, Token::ReturnKeyword)?;
    let integer = expect_integer_literal(&mut iter)?;
    let constant = Exp::Const(integer);
    expect_token(&mut iter, Token::Semicolon)?;
    expect_token(&mut iter, Token::CloseBrace)?;

    if iter.next().is_some() {
        return Err("Unexpected tokens at after function delcaration".to_string());
    }
    let fn_decl = FunDecl {
        name: identifier,
        body: Statement::Return(constant),
    };

    return Ok(Program{func: fn_decl});


}
/// Pretty-print function to display the AST in a readable way.
///
/// # Arguments
///
/// * `ast` - The AST to be printed.
// pub fn pretty_print(ast: &Program) {
//     println!("FUN INT {}:", ast.func.name);
//     println!("    params: ()");
//     match &ast.func.body {
//         Statement::Return(exp) => match exp {
//             Exp::Const(value) => println!("    body:\n        RETURN Int<{}>", value),
//         },
//     }
// }

/// Helper function to check if the next token matches the expected token type.
///
/// # Arguments
///
/// * `iter` - A mutable reference to a Peekable iterator over the tokens.
/// * `expected` - The expected token type.
///
/// # Returns
///
/// If the token matches, it consumes the token and returns `Ok(())`.
/// Otherwise, it returns an `Err` with an error message.
fn expect_token(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>, expected: Token) -> Result<(), String> {
    match iter.peek() {
        Some(token) if *token == expected => {
            iter.next();
            Ok(())
        }
        Some(token) => Err(format!("Expected {:?}, found {:?}", expected, token)),
        None => Err(format!("Expected {:?}, but found end of input", expected)),
    }
}

/// Helper function to check if the next token is an identifier and returns its value.
///
/// # Arguments
///
/// * `iter` - A mutable reference to a Peekable iterator over the tokens.
///
/// # Returns
///
/// If the token is an identifier, it consumes the token and returns its value.
/// Otherwise, it returns an `Err` with an error message.
fn expect_identifier(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<String, String> {
    match iter.next() {
        Some(Token::Identifier(name)) => Ok(name),
        Some(token) => Err(format!("Expected identifier, found {:?}", token)),
        None => Err("Expected identifier, but found end of input".to_string()),
    }
}

/// Helper function to check if the next token is an integer literal and returns its value.
///
/// # Arguments
///
/// * `iter` - A mutable reference to a Peekable iterator over the tokens.
///
/// # Returns
///
/// If the token is an integer literal, it consumes the token and returns its value.
/// Otherwise, it returns an `Err` with an error message.
fn expect_integer_literal(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<i32, String> {
    match iter.next() {
        Some(Token::IntegerLiteral(value)) => {
            value.parse::<i32>().map_err(|_| "Invalid integer literal".to_string())
        }
        Some(token) => Err(format!("Expected integer literal, found {:?}", token)),
        None => Err("Expected integer literal, but found end of input".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect_token_success() {
        let tokens = vec![Token::IntKeyword];
        let mut iter = tokens.into_iter().peekable();
        assert!(expect_token(&mut iter, Token::IntKeyword).is_ok());
    }

    #[test]
    fn test_expect_token_failure() {
        let tokens = vec![Token::ReturnKeyword];
        let mut iter = tokens.into_iter().peekable();
        let result = expect_token(&mut iter, Token::IntKeyword);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected IntKeyword, found ReturnKeyword");
    }

    #[test]
    fn test_expect_identifier_success() {
        let tokens = vec![Token::Identifier("myFunc".to_string())];
        let mut iter = tokens.into_iter().peekable();
        let result = expect_identifier(&mut iter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "myFunc".to_string());
    }

    #[test]
    fn test_expect_identifier_failure() {
        let tokens = vec![Token::IntKeyword];
        let mut iter = tokens.into_iter().peekable();
        let result = expect_identifier(&mut iter);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected identifier, found IntKeyword");
    }

    #[test]
    fn test_expect_integer_literal_success() {
        let tokens = vec![Token::IntegerLiteral("42".to_string())];
        let mut iter = tokens.into_iter().peekable();
        let result = expect_integer_literal(&mut iter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_expect_integer_literal_failure() {
        let tokens = vec![Token::IntKeyword];
        let mut iter = tokens.into_iter().peekable();
        let result = expect_integer_literal(&mut iter);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected integer literal, found IntKeyword");
    }

    #[test]
    fn test_parse_valid_program() {
        let tokens = vec![
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
        let result = parse(tokens);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.func.name, "main");
        if let Statement::Return(Exp::Const(value)) = program.func.body {
            assert_eq!(value, 42);
        } else {
            panic!("Expected return statement with constant value");
        }
    }

    #[test]
    fn test_parse_invalid_program_unexpected_token() {
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntKeyword, // Invalid token here
            Token::Semicolon,
            Token::CloseBrace,
        ];
        let result = parse(tokens);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected integer literal, found IntKeyword");
    }

    #[test]
    fn test_parse_invalid_program_extra_tokens() {
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::IntegerLiteral("42".to_string()),
            Token::Semicolon,
            Token::CloseBrace,
            Token::Semicolon, // Extra token here
        ];
        let result = parse(tokens);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unexpected tokens at after function delcaration");
    }
}
