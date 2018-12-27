use crate::scanner::{Token, TokenKind};

#[derive(Debug, Clone)]
enum ParseError {
    MissingParenthesis,
    UnexpectedToken,
}

// The tokens are owned. Probably not the best idea.
enum Expr {
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

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        // handle the ( ...)* part of the rule for association
        while self.match_any_of(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();

        // handle the ( ...)* part of the rule for association
        while self.match_any_of(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.addition();
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();

        // handle the ( ...)* part of the rule for association
        while self.match_any_of(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.multiplication();
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();

        // handle the ( ...)* part of the rule for association
        while self.match_any_of(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        match self.peek().kind {
            TokenKind::Bang | TokenKind::Minus => {
                self.advance();
                let operator = self.previous();
                let right = self.unary();
                Expr::Unary(operator, Box::from(right))
            }
            _ => self.primary().unwrap(), // FIXME: Propagate the error !
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
                let expr = self.expression();
                if self.consume(TokenKind::RightParen).is_none() {
                    Err(ParseError::MissingParenthesis)
                } else {
                    Ok(Expr::Grouping(Box::from(expr)))
                }
            }
            _ => Err(ParseError::UnexpectedToken),
        }
    }
}

fn pretty_print(expr: &Expr) -> String {
    let mut output = String::new();

    match expr {
        Expr::Literal(token) => {
            output.push_str(&token.lexeme);
        }
        Expr::Unary(token, expr) => {
            output.push_str(&token.lexeme);
            output.push_str(&pretty_print(expr.as_ref()));
        }
        Expr::Binary(left, token, right) => {
            output.push_str(&pretty_print(left.as_ref()));
            output.push_str(&token.lexeme);
            output.push_str(&pretty_print(right.as_ref()));
        }
        Expr::Grouping(expr) => {
            output.push_str(&pretty_print(expr.as_ref()));
        }
        _ => output.push_str("Unknown Expression"),
    };

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_literal() {
        let number_literal = Token::new(TokenKind::Number(42.0), "42".to_owned(), 1);
        let result = pretty_print(&Expr::Literal(number_literal));
        assert_eq!("42", &result);
    }

    #[test]
    fn print_unary() {
        let minus_token = Token::new(TokenKind::Minus, "-".to_owned(), 1);
        let literal_token = Token::new(TokenKind::Number(42.0), "42".to_owned(), 1);
        let expr = Expr::Unary(minus_token, Box::from(Expr::Literal(literal_token)));

        let result = pretty_print(&expr);
        assert_eq!("-42", &result);
    }
}
