use crate::scanner::{Token, TokenKind};

#[derive(PartialEq, Debug, Clone)]
pub enum ParseError {
    MissingParenthesis,
    UnexpectedToken,
}

// The tokens are owned. Probably not the best idea.
#[derive(PartialEq, Debug)]
pub enum Expr {
    Literal(Token),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

/// Grammar we want to parse:
///
///    expression     → equality ;
///    equality       → comparison ( ( "!=" | "==" ) comparison )* ;
///    comparison     → addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
///    addition       → multiplication ( ( "-" | "+" ) multiplication )* ;
///    multiplication → unary ( ( "/" | "*" ) unary )* ;
///    unary          → ( "!" | "-" ) unary
///                   | primary ;
///    primary        → NUMBER | STRING | "false" | "true" | "nil"
///                   | "(" expression ")" ;
///
/// Each rule is mapped to the corresponding function.
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    // Utilities

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }

        self.previous()
    }

    fn consume(&mut self, kind: TokenKind) -> Option<Token> {
        if self.peek().kind == kind {
            Some(self.advance())
        } else {
            None
        }
    }

    fn match_any_of(&mut self, kinds: &[TokenKind]) -> bool {
        if kinds.contains(&self.peek().kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    // GRAMMAR DEF

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        // handle the ( ...)* part of the rule for association
        while self.match_any_of(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.addition()?;

        // handle the ( ...)* part of the rule for association
        while self.match_any_of(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.addition()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.multiplication()?;

        // handle the ( ...)* part of the rule for association
        while self.match_any_of(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.multiplication()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        // handle the ( ...)* part of the rule for association
        while self.match_any_of(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().kind {
            TokenKind::Bang | TokenKind::Minus => {
                self.advance();
                let operator = self.previous();
                let right = self.unary()?;
                Ok(Expr::Unary(operator, Box::from(right)))
            }
            _ => self.primary(),
        }
    }

    ///    primary        → NUMBER | STRING | "false" | "true" | "nil"
    ///                   | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().kind {
            TokenKind::False | TokenKind::True | TokenKind::Nil => {
                self.advance();
                Ok(Expr::Literal(self.previous()))
            }
            TokenKind::Number(_) | TokenKind::String(_) => {
                self.advance();
                Ok(Expr::Literal(self.previous()))
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                if self.consume(TokenKind::RightParen).is_none() {
                    Err(ParseError::MissingParenthesis)
                } else {
                    Ok(Expr::Grouping(Box::from(expr)))
                }
            }
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    // Error Handling

    /// After an error is signaled, we skip tokens until we reach a token that
    /// could be a delimiter. The goal is to try to get back on our feet and
    /// continue parsing.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::SemiColon {
                return;
            }

            match self.peek().kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}

pub fn ast_dump(expr: &Expr) -> String {
    let mut output = String::new();

    match expr {
        Expr::Literal(token) => {
            output.push_str("(");
            output.push_str(&token.lexeme);
            output.push_str(")");
        }
        Expr::Unary(token, expr) => {
            output.push_str("(");
            output.push_str(&token.lexeme);
            output.push_str(&ast_dump(expr.as_ref()));
            output.push_str(")");
        }
        Expr::Binary(left, token, right) => {
            output.push_str("(");
            output.push_str(&ast_dump(left.as_ref()));
            output.push_str(&token.lexeme);
            output.push_str(&ast_dump(right.as_ref()));
            output.push_str(")");
        }
        Expr::Grouping(expr) => {
            output.push_str("(");
            output.push_str(&ast_dump(expr.as_ref()));
            output.push_str(")");
        }
    };

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;

    #[test]
    fn print_literal() {
        let number_literal = Token::new(TokenKind::Number(42.0), "42".to_owned(), 1);
        let result = ast_dump(&Expr::Literal(number_literal));
        assert_eq!("(42)", &result);
    }

    #[test]
    fn print_unary() {
        let minus_token = Token::new(TokenKind::Minus, "-".to_owned(), 1);
        let literal_token = Token::new(TokenKind::Number(42.0), "42".to_owned(), 1);
        let expr = Expr::Unary(minus_token, Box::from(Expr::Literal(literal_token)));

        let result = ast_dump(&expr);
        assert_eq!("(-(42))", &result);
    }

    #[test]
    fn test_parse() {
        let mut scanner = Scanner::new("3 + 4".to_owned());
        scanner.scan_tokens();
        let mut parser = Parser::new(scanner.tokens);

        let expected = Expr::Binary(
            Box::new(Expr::Literal(Token::new(
                TokenKind::Number(3.0),
                "3".to_owned(),
                1,
            ))),
            Token::new(TokenKind::Plus, "+".to_owned(), 1),
            Box::new(Expr::Literal(Token::new(
                TokenKind::Number(4.0),
                "4".to_owned(),
                1,
            ))),
        );

        assert_eq!(expected, parser.parse().unwrap());
    }

    #[test]
    fn invalid_unary_parse() {
        let invalid_unary = String::from("-");
        let mut scanner = Scanner::new(invalid_unary);
        scanner.scan_tokens();
        let mut parser = Parser::new(scanner.tokens);
        assert_eq!(Err(ParseError::UnexpectedToken), parser.parse());
    }

    #[test]
    fn invalid_binary_parse() {
        let invalid_binary = String::from("3 +");
        let mut scanner = Scanner::new(invalid_binary);
        scanner.scan_tokens();
        let mut parser = Parser::new(scanner.tokens);
        assert_eq!(Err(ParseError::UnexpectedToken), parser.parse());
    }

        #[test]
    fn missing_closing_parenthesis() {
        let missing_parenthesis = String::from("(42");
        let mut scanner = Scanner::new(missing_parenthesis);
        scanner.scan_tokens();

        for token in &scanner.tokens {
            println!("Token: {:?}", token);
        }

        let mut parser = Parser::new(scanner.tokens);
        assert_eq!(Err(ParseError::MissingParenthesis), parser.parse());
    }
}
