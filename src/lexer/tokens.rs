#[derive(Debug)]
pub enum TokenType {
    VariableDeclaration,
    Identifier,
    Comment,
    Assign,
    String,
    Number,
    CodeBlock,
    FunctionArguments,
    FunctionIdentifier
}

#[derive(Debug)]
pub struct Token {
    t_type: TokenType,
    t_literal: String,
    t_index: usize,
    t_line: usize
}

impl Token {
    pub fn new(t_type: TokenType, t_literal: String, t_index: usize, t_line: usize) -> Self {
        Self {
            t_type,
            t_literal,
            t_index,
            t_line
        }
    }
}