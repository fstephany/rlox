#[derive(Debug, PartialEq)]
enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String(String),
    Number(f32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    fn new(kind: TokenKind, lexeme: String, line: usize) -> Token {
        Token {
            kind,
            lexeme,
            line,
        }
    }
}

#[derive(Debug)]
struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub had_errors: bool,
    // start of the current lexeme
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            tokens: Vec::new(),
            had_errors: false,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenKind::Eof, "".to_owned(), self.line));
    }

    fn error(&mut self, line: usize, message: String) {
        self.had_errors = true;
        println!("Error at line {}: {}", line, message);
    }

    fn add_token(&mut self, kind: TokenKind) {
        // Beware that we are slicing bytes here. Not actual characters.
        let text_slice = &self.source[self.start..self.current];
        let token = Token::new(kind, text_slice.to_owned(), self.line);

        self.tokens.push(token);
    }

    fn scan_token(&mut self) {
        let c = match self.advance() {
            Some(c) => c,
            None => return
        };

        match c {
            // Single char tokens
            '(' => self.add_token(TokenKind::LeftParen,),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            ';' => self.add_token(TokenKind::SemiColon),
            '*' => self.add_token(TokenKind::Star),

            // Single or two char(s) tokens
            '!' => {
                let token = if self.advance_if_matches('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.add_token(token)
            },
            '=' => {
                let token = if self.advance_if_matches('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token(token)
            },
            '<' => {
                let token = if self.advance_if_matches('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.add_token(token)
            },
            '>' => {
                let token = if self.advance_if_matches('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.add_token(token)
            },

            // '/' can be a commented line.
            '/' => {
                if self.advance_if_matches('/') {
                    // consume the comment without doing anything with it.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash);
                }
            },

            // Eats whitespace
            ' ' | '\r' | '\t' => { /* Do Nothing */},

            '\n' => self.line = self.line + 1,

            // string literals
            '"' => self.string_literal(),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.number_literal(),

            // Nothing we know
            default => self.error(self.line, "Unexpected character".to_owned()),
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number_literal(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // consume '.'
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let literal_value = &self.source[self.start .. self.current];
        let double_value = literal_value.parse::<f32>().unwrap();
        
        self.add_token(TokenKind::Number(double_value));
    }

    fn string_literal(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string".to_owned());
            return
        } 
        
        // closing quote
        self.advance();

        // +1/-1 because we don't want the quote
        let literal_value = &self.source[self.start +1 .. self.current -1];
        self.add_token(TokenKind::String(literal_value.to_owned()));
    }

    /// Get the next char without consuming it.
    fn peek(&self) -> char {
        return self.source.chars().nth(self.current).unwrap_or('\0');
    }

    fn peek_next(&self) -> char {
        return self.source.chars().nth(self.current + 1).unwrap_or('\0');
    }

    /// consumes the next char if it matches the expected one.
    fn advance_if_matches(&mut self, expected: char) -> bool {
        match self.source.chars().nth(self.current) {
            Some(c) => {
                if c == expected {
                    self.current = self.current + 1;
                    return true;
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current = self.current + 1;
        self.source.chars().nth(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let scanner = Scanner::new("".to_owned());
        assert!(!scanner.had_errors);
    }

    #[test]
    fn empty_source() {
        let mut scanner = Scanner::new("".to_owned());
        scanner.scan_tokens();

        assert!(!scanner.had_errors);
        assert_eq!(1, scanner.tokens.len());
        assert_eq!(TokenKind::Eof, scanner.tokens[0].kind);
    }

    #[test]
    fn single_char_tokens() {
        let mut scanner = Scanner::new("{}()+".to_owned());
        scanner.scan_tokens();
        assert!(!scanner.had_errors);
        assert_eq!(6, scanner.tokens.len())
    }

    #[test]
    fn multi_line() {
        let source = r#"{}
        >
        <=
        +"#;

        let mut scanner = Scanner::new(source.to_owned());
        scanner.scan_tokens();
        assert!(!scanner.had_errors);
        assert_eq!(6, scanner.tokens.len())
    }

    #[test]
    fn end_of_stream_lookahead() {
        let mut scanner = Scanner::new("<".to_owned());
        scanner.scan_tokens();
        assert!(!scanner.had_errors);

        let scanned_token = &scanner.tokens[0];
        assert_eq!(TokenKind::Less, scanned_token.kind);
    }


    #[test]
    fn comments() {
        let source = r#"
        // This is a comment
        < // another comment
        {// a third comment
        "#;

        let mut scanner = Scanner::new(source.to_owned());
        scanner.scan_tokens();
        assert!(!scanner.had_errors);

        assert_eq!(3, scanner.tokens.len());
    }

    #[test]
    fn string_literal() {
        let source = r#"
        "blop"
        "#;

        let mut scanner = Scanner::new(source.to_owned());
        scanner.scan_tokens();
        assert!(!scanner.had_errors);
        assert_eq!(&TokenKind::String("blop".to_owned()), &scanner.tokens[0].kind);
    }

    #[test]
    fn multi_line_string_literal() {
        let source = String::from(r#"
        "blop
        blip"
        "#);

        let literal = String::from(r#"blop
        blip"#);

        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        assert!(!scanner.had_errors);
        assert_eq!(&TokenKind::String(literal), &scanner.tokens[0].kind);
    }

    #[test]
    fn numbers() {
        let source = String::from("7 42 3.14");
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        assert!(!scanner.had_errors);

        assert_eq!(&TokenKind::Number(7.0), &scanner.tokens[0].kind);
        assert_eq!(&TokenKind::Number(42.0), &scanner.tokens[1].kind);
        assert_eq!(&TokenKind::Number(3.14), &scanner.tokens[2].kind);
    }
}