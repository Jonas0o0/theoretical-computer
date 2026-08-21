use crate::ast::{BinaryOperator, Expr, Program, Stmt};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> &Token {
        let token = self.tokens.get(self.current).unwrap_or(&Token::EOF);
        self.current += 1;
        token
    }

    fn consume(&mut self, expected: Token, error_message: &str) -> Result<&Token, String> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            Err(error_message.to_string())
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while self.peek() != &Token::EOF {
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Let => self.parse_let_statement(),
            Token::If => self.parse_if_statement(),
            Token::Loop => self.parse_loop_statement(),
            Token::While => self.parse_while_statement(),
            Token::Poke => self.parse_poke_statement(),
            Token::Identifier(_) => self.parse_assignation_or_call_function(),
            Token::Fn => self.parse_function(),
            _ => Err(format!("Instruction inattendue : {:?}", self.peek())),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.consume(Token::LBrace, "On attend une '{' au début du bloc")?;

        let mut statements = Vec::new();
        while self.peek() != &Token::RBrace && self.peek() != &Token::EOF {
            statements.push(self.parse_statement()?);
        }
        self.consume(Token::RBrace, "On attend une '}' à la fin du bloc")?;

        Ok(statements)
    }

    fn parse_let_statement(&mut self) -> Result<Stmt, String> {
        self.advance();

        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err("On attendait un nom de variable après 'let'".to_string()),
        };

        self.consume(Token::Assign, "On attend un '=' après le nom de la variable")?;

        let value = self.parse_expression()?;

        self.consume(Token::Semicolon, "Il manque un ';' à la fin de l'instruction")?;

        Ok(Stmt::Let { name, value })
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, String> {
        self.advance();
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Stmt::If { condition, body })
    }

    fn parse_loop_statement(&mut self) -> Result<Stmt, String>{
        self.advance();
        let body = self.parse_block()?;
        Ok (Stmt::Loop { body })
    }

    fn parse_while_statement(&mut self) -> Result<Stmt, String>{
        self.advance();
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body })
    }

    fn parse_poke_statement (&mut self) -> Result<Stmt, String>{
        self.advance();
        self.consume(Token::LParen, "On attend un '(' après poke")?;
        let address = self.parse_expression()?;
        self.consume(Token::Comma, "On attend un ',' entre les 2 expressions")?;
        let value = self.parse_expression()?;
        self.consume(Token::RParen, "On attend un ')' a la fin d'un poke")?;
        self.consume(Token::Semicolon, "Il manque un ';' à la fin de l'instruction")?;
        Ok(Stmt::Poke { address, value })
    }

    fn parse_assignation_or_call_function(&mut self) -> Result<Stmt, String> {
        let nom = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err("Impossible de lire l'identifiant".to_string()),
        };

        match self.peek() {
            Token::Assign => {
                self.advance();
                let value = self.parse_expression()?;
                self.consume(Token::Semicolon, "Il manque un ';' après l'assignation")?;

                Ok(Stmt::Assign { name: nom, value })
            }
            Token::LParen => {
                self.advance();
                self.consume(Token::RParen, "Il manque une ')' après l'appel de fonction")?;
                self.consume(Token::Semicolon, "Il manque un ';' après l'appel de fonction")?;

                Ok(Stmt::FnCall(nom))
            }

            _ => Err(format!("Après '{}', on attendait un '=' ou des parenthèses '()'", nom)),
        }
    }

    fn parse_function(&mut self) -> Result<Stmt, String> {
        self.advance();
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err("On attendait un nom de fonction après 'fn'".to_string()),
        };
        self.consume(Token::LParen, "Il manque une '(' après le nom de la fonction")?;
        self.consume(Token::RParen, "Il manque une ')' pour fermer les arguments")?;
        let body = self.parse_block()?;
        Ok(Stmt::InlineFn { name, body })
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        let token = self.advance().clone();

        let mut gauche = match token {
            Token::Number(n) => Expr::Number(n),

            Token::Identifier(name) => Expr::Identifier(name),

            Token::Peek => {
                self.consume(Token::LParen, "On attend un '(' après peek")?;
                let interieur = self.parse_expression()?;
                self.consume(Token::RParen, "On attend un ')' a la fin d'un peek")?;
                Expr::Peek(Box::new(interieur))
            }

            _ => return Err(format!("Expression invalide : {:?}", token)),
        };

        loop {
            let operateur = match self.peek() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Sub,
                Token::Equal => BinaryOperator::Equal,
                Token::NotEqual => BinaryOperator::NotEqual,
                Token::LessThan => BinaryOperator::LessThan,
                Token::GreaterThan => BinaryOperator::GreaterThan,
                _ => break,
            };
            self.advance();
            let droite = self.parse_expression()?;

            gauche = Expr::BinaryOp {
                left: Box::new(gauche),
                operator: operateur,
                right: Box::new(droite),
            };
        }
        Ok(gauche)
    }

}