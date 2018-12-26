use crate::scanner::{Token, TokenKind};

enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token)
}


fn pretty_print(expr: &Expr) -> String {
    let mut output = String::new();

    match expr {
        Expr::Unary(token, expr) => { 
            output.push_str(&token.lexeme);
            output.push_str(&pretty_print(expr.as_ref()))
        }
        Expr::Literal(token) => {
            output.push_str(&token.lexeme);
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