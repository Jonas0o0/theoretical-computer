#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Mots-clés
    Let,
    Fn,
    Loop,
    While,
    If,
    Peek,
    Poke,

    // Identifiants et Valeurs
    Identifier(String),
    Number(u8),

    // Opérateurs et Ponctuation
    Assign,      // =
    Equal,       // ==
    NotEqual,    // !=
    LessThan,    // <
    GreaterThan, // >
    Plus,        // +
    Minus,       // -
    LBrace,      // {
    RBrace,      // }
    LParen,      // (
    RParen,      // )
    Comma,       // ,
    Semicolon,   // ;

    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let current = self.input[self.position];

            if current.is_whitespace() {
                self.position += 1;
                continue;
            }

            if current.is_ascii_digit() {
                let num = self.read_number();
                tokens.push(Token::Number(num));
                continue;
            }

            if current.is_ascii_alphabetic() {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "let" => tokens.push(Token::Let),
                    "fn" => tokens.push(Token::Fn),
                    "loop" => tokens.push(Token::Loop),
                    "while" => tokens.push(Token::While),
                    "if" => tokens.push(Token::If),
                    "peek" => tokens.push(Token::Peek),
                    "poke" => tokens.push(Token::Poke),
                    _ => tokens.push(Token::Identifier(ident)),
                }
                continue;
            }

            match current {
                '=' => {
                    if self.peek_next() == '=' {
                        self.position += 1;
                        tokens.push(Token::Equal);
                    } else {
                        tokens.push(Token::Assign);
                    }
                }
                '!' => {
                    if self.peek_next() == '=' {
                        self.position += 1;
                        tokens.push(Token::NotEqual);
                    } else {
                        panic!("Caractère non reconnu : {}", current);
                    }
                }
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '{' => tokens.push(Token::LBrace),
                '}' => tokens.push(Token::RBrace),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                ',' => tokens.push(Token::Comma),
                ';' => tokens.push(Token::Semicolon),
                '<' => tokens.push(Token::LessThan),
                '>' => tokens.push(Token::GreaterThan),
                _ => {
                    panic!("Caractère non reconnu : {}", current);
                }
            }

            self.position += 1;
        }

        tokens
    }

    fn read_number(&mut self) -> u8 {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }
        let num_str: String = self.input[start..self.position].iter().collect();
        num_str.parse().unwrap_or(0)
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_alphanumeric() {
            self.position += 1;
        }
        self.input[start..self.position].iter().collect()
    }

    fn peek_next(&self) -> char {
        if self.position + 1 < self.input.len() {
            self.input[self.position + 1]
        } else {
            '\0'
        }
    }
}