use crate::scanner::{Token, TokenKind};

enum Expr {
    Literal(Token),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>)
}


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0
        }
    }

    fn isAtEnd(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn advance(&mut self) -> Token {
        if !self.isAtEnd() {
            self.current = self.current + 1;
        }

        self.previous()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        // handle the ( ...)* part of the rule for association
        let matchEquality = || {
            match self.peek().kind {
                TokenKind::BangEqual | TokenKind::EqualEqual =>  {
                    self.advance();
                    true
                }
                _ => false
            }
        };

        while matchEquality() {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        
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
        _ => output.push_str("Unknown Expression")
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