use std::fmt;
/// Enum representing the different types of tokens that the lexer can recognize.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    IntKeyword,
    ReturnKeyword,
    VoidKeyword,
    Identifier(String),
    IntegerLiteral(String),
    Negation,
    BitwiseComplement,
    LogicalNegation,
    Decrement,
}

// AST nodes
#[derive(Debug)]
pub struct Program {
    pub func: FunDecl,
}
#[derive(Debug)]
pub struct FunDecl {
    pub name: String,
    pub body: Statement,
}
#[derive(Debug)]
pub enum Statement {
    Return(Exp),
}
#[derive(Debug)]
pub enum Exp {
    Const(i32),
    //UnOp(Token, Box<Exp>),
}
// ---Define the structure for the Assembly AST----
#[derive(Debug)]
pub struct AsmProgram {
    pub function: AsmFunction,
}
#[derive(Debug)]
pub struct AsmFunction {
    pub name: String,
    pub instructions: Vec<AsmInstruction>,
}
#[derive(Debug)]
pub enum AsmInstruction {
    Mov(AsmOperand, AsmOperand),
    Ret,
}
#[derive(Debug)]
pub enum AsmOperand {
    Imm(i32),
    Register,
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::OpenBrace => write!(f, "Open brace"),
            Token::CloseBrace => write!(f, "Close brace"),
            Token::OpenParenthesis => write!(f, "Open parenthesis"),
            Token::CloseParenthesis => write!(f, "Close parenthesis"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::IntKeyword => write!(f, "Int keyword"),
            Token::ReturnKeyword => write!(f, "Return keyword"),
            Token::VoidKeyword => write!(f, "Void keyword"),
            Token::Identifier(val) => write!(f, "Identifier \"{}\"", val),
            Token::IntegerLiteral(val) => write!(f, "Constant \"{}\"", val),
            Token::Negation => write!(f, "Negation"),
            Token::BitwiseComplement => write!(f, "Bitwise complement"),
            Token::LogicalNegation => write!(f, "Logcial negation"),
            Token::Decrement => write!(f, "Decrement operator"),
        }
    }
}

