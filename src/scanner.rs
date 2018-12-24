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
    String,
    Number,

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
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    fn new(kind: TokenKind, lexeme: String, literal: Option<String>, line: usize) -> Token {
        Token {
            kind,
            lexeme,
            literal,
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
            .push(Token::new(TokenKind::Eof, "".to_owned(), None, self.line));
    }

    fn error(&mut self, line: usize, message: String) {
        self.had_errors = true;
        println!("Error at line {}: {}", line, message);
    }

    fn scan_token(&mut self) {
        let c = match self.advance() {
            Some(c) => c,
            None => return
        };

        match c {
            // Single char tokens
            '(' => self.add_token(TokenKind::LeftParen),
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
                let token = if self.next_matches('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.add_token(token)
            },
            '=' => {
                let token = if self.next_matches('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token(token)
            },
            '<' => {
                let token = if self.next_matches('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.add_token(token)
            },
            '>' => {
                let token = if self.next_matches('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.add_token(token)
            },

            // '/' can be a commented line.
            '/' => {
                if self.next_matches('/') {
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

            // Nothing we know
            default => self.error(self.line, "Unexpected character".to_owned()),
        }
    }

    /// Get the next char without consuming it.
    fn peek(&self) -> char {
        return self.source.chars().nth(self.current).unwrap_or('\0');
    }

    /// consumes the next char if it matches the expected one.
    fn next_matches(&mut self, expected: char) -> bool {
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

    fn add_token(&mut self, kind: TokenKind) {
        // Beware that we are slicing bytes here. Not actual characters.
        let text_slice = &self.source[self.start..self.current];
        let token = Token::new(kind, text_slice.to_owned(), None, self.line);

        self.tokens.push(token);
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
}