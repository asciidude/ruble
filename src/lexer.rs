mod tokens;

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    debug_name: String
}

impl Lexer {
    pub fn new(source: String, debug_name: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 0,
            debug_name: debug_name
        }
    }

    pub fn lex(&mut self) -> Vec<tokens::Token> {
        let mut codeblock_index = -1;
        let mut tokens: Vec<tokens::Token> = Vec::<tokens::Token>::new();

        while self.source.len() > self.position {
            match self.current_char() {
                '=' => {
                    tokens.push(tokens::Token::new(tokens::TokenType::Assign, "=".to_owned(), self.position, self.line));
                    self.position += 1;
                },
                '"' => {
                    self.position += 1;
                    let mut buffer: String = String::new();

                    while self.current_char() != '"' {
                        if self.current_char() == '\\' {
                            self.position += 1;
                        }

                        buffer.push(self.current_char());
                        self.position += 1;
                    }

                    tokens.push(tokens::Token::new(tokens::TokenType::String, buffer, self.position, self.line));
                    self.position += 1;
                },
                ';' => {
                    let mut buffer: String = String::new();
                    self.position += 1;

                    buffer.push(';');
                    while self.current_char() != '\r' && self.current_char() != '\n' {
                        buffer.push(self.current_char());
                        self.position += 1;
                    }

                    tokens.push(tokens::Token::new(tokens::TokenType::Comment, buffer, self.position, self.line));
                    self.position += 1;
                },
                '{' => {
                    self.position += 1;
                    codeblock_index += 1;
                    let mut block_lex: String = String::new();
                    let mut buffer: String = String::new(); // (?) Remove later and just use buffer

                    while self.current_char() != '}' {
                        block_lex.push(self.current_char());
                        buffer.push(self.current_char());
                        self.position += 1;
                    }

                    buffer = buffer.replace("   ", "\t");

                    Lexer::new(block_lex, format!("{}{}",String::from("CODEBLOCK "), codeblock_index.to_string())).lex(); // (?) Run in parser
                    tokens.push(tokens::Token::new(tokens::TokenType::CodeBlock, buffer, self.position, self.line));
                    self.position += 1;
                },
                '(' => {
                    self.position += 1;
                    let mut buffer: String = String::new();

                    buffer.push('(');
                    while self.current_char() != ')' {
                        buffer.push(self.current_char());
                        self.position += 1;
                    }
                    buffer.push(')');

                    tokens.push(tokens::Token::new(tokens::TokenType::FunctionArguments, buffer, self.position, self.line));
                    self.position += 1;
                },
                _ if self.current_char().is_whitespace() => {
                    if self.current_char() == '\r' && self.next_char() == '\n' {
                        self.position += 1;
                        self.line += 1;
                        break;
                    }

                    self.position += 1;
                },
                _ if self.current_char().is_numeric() => {
                    let mut buffer: String = String::new();

                    loop {
                        if self.position >= self.source.len() ||
                            self.current_char().is_whitespace() {
                            break;
                        }

                        if self.current_char() == '_' {
                            self.position += 1;
                        }

                        if self.current_char() == '.' {
                            buffer.push_str(".");
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
                        "fn" => tokens::TokenType::FunctionIdentifier,
                        "let" => tokens::TokenType::VariableDeclaration,
                        _ => tokens::TokenType::Identifier
                    };

                    tokens.push(tokens::Token::new(t_type, buffer, self.position, self.line));
                },
                _ => unimplemented!("Unimplemented at line {0} position {1}, current character is {2} and next character is {3}",
                                    self.line, self.position, self.current_char(), self.next_char())
            }
        }

        println!("---+ START LEXER: {0} +----", self.debug_name.to_owned());

        for _token in &tokens {
            println!("{:?}", _token);
        }

        println!("---+ END LEXER:   {0} +---", self.debug_name.to_owned());

        tokens
    }

    pub fn current_char(&self) -> char {
        *self.source.get(self.position).unwrap()
    }

    pub fn next_char(&self) -> char {
        *self.source.get(self.position + 1).unwrap()
    }
}
