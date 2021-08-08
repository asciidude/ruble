mod tokens;

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 0
        }
    }

    pub fn lex(&mut self) {
        let mut tokens: Vec<tokens::Token> = Vec::<tokens::Token>::new();
        
        while self.source.len() > self.position {
            match self.current_char() {
                '=' => {
                    tokens.push(tokens::Token::new(tokens::TokenType::Assign, "=".to_owned(), self.position, self.line));
                    self.position += 1;
                },
                '\'' | '"' => {
                    self.position += 1;
                    let mut buffer: String = String::new();
                    while self.current_char() != '\'' && self.current_char() != '"' {
                        if self.current_char() == '\\' {
                            self.position += 1;
                        }

                        buffer.push(self.current_char());
                        self.position += 1;
                    }

                    tokens.push(tokens::Token::new(tokens::TokenType::String, buffer, self.position, self.line));
                    self.position += 1;
                },
                '\n' => {
                    self.position += 1;
                    self.line += 1;
                },
                _ if self.current_char().is_numeric() => {
                    let mut buffer: String = String::new();

                    loop {
                        if self.position >= self.source.len() {
                            break;
                        }

                        if self.current_char() == '_' {
                            self.position += 1;
                        }

                        buffer.push(self.current_char());
                        self.position += 1;
                    }

                    tokens.push(tokens::Token::new(tokens::TokenType::Number, buffer, self.position, self.line));
                },
                _ if self.current_char().is_alphabetic() => {
                    let mut buffer: String = String::new();
                    while self.current_char().is_alphabetic() {
                        buffer.push(self.current_char());
                        self.position += 1;
                    }

                    let t_type: tokens::TokenType = match buffer.as_str() {
                        "let" => tokens::TokenType::VariableDeclaration,
                        _ => tokens::TokenType::Identifier
                    };

                    tokens.push(tokens::Token::new(t_type, buffer, self.position, self.line));
                },
                _ => self.position += 1 /*unimplemented!("Unimplemented at position {0}", self.position)*/
            }
        }
    }

    pub fn current_char(&self) -> char {
        *self.source.get(self.position).unwrap()
    }
}
