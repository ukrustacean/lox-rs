use crate::token::{Token, TokenKind};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    stream: std::iter::Peekable<std::str::Chars<'a>>,

    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let stream = source.chars().peekable();

        Self {
            source,
            tokens: vec![],
            stream,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while let Some(c) = self.next() {
            self.start = self.current - 1;

            use TokenKind::*;
            match c {
                '(' => self.add_token(LeftParen),
                ')' => self.add_token(RightParen),
                '{' => self.add_token(LeftBrace),
                '}' => self.add_token(RightBrace),
                ',' => self.add_token(Comma),
                '.' => self.add_token(Dot),
                '-' => self.add_token(Minus),
                '+' => self.add_token(Plus),
                ';' => self.add_token(Semicolon),
                '*' => self.add_token(Star),
                '!' => {
                    let tok = if self.expect_next('=') {
                        BangEqual
                    } else {
                        Bang
                    };
                    self.add_token(tok);
                }
                '=' => {
                    let tok = if self.expect_next('=') {
                        EqualEqual
                    } else {
                        Equal
                    };
                    self.add_token(tok);
                }
                '<' => {
                    let tok = if self.expect_next('=') {
                        LessEqual
                    } else {
                        Less
                    };
                    self.add_token(tok);
                }
                '>' => {
                    let tok = if self.expect_next('=') {
                        GreaterEqual
                    } else {
                        Greater
                    };
                    self.add_token(tok);
                }
                '/' => {
                    if self.expect_next('/') {
                        while let Some(c) = self.peek() {
                            if c == '\n' {
                                break;
                            }
                            self.next();
                        }
                    } else if self.expect_next('*') {
                        self.scan_multiline_comment()
                    } else {
                        self.add_token(Slash);
                    }
                }
                '"' => self.scan_string(),
                '0'..='9' => self.scan_number(),
                c if is_alpha(c) => self.scan_identifier(),
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,
                _ => panic!("Unexpected character at line {}", self.line),
            }
        }

        self.tokens
    }

    fn next(&mut self) -> Option<char> {
        self.current += 1;
        self.stream.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.stream.peek().copied()
    }

    fn is_eof(&mut self) -> bool {
        self.peek().is_none()
    }

    fn add_token(&mut self, token: TokenKind) {
        self.tokens.push(Token::new(token, self.line))
    }

    fn expect_next(&mut self, a: char) -> bool {
        if let Some(b) = self.peek() {
            if a == b {
                self.next();
                return true;
            }
        }

        false
    }

    fn scan_string(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '"' => break,
                '\n' => {
                    self.line += 1;
                    self.next();
                }
                _ => {
                    self.next();
                }
            }
        }

        if self.is_eof() {
            panic!("Unterminated string at line {}", self.line)
        }

        let value = self.source[(self.start + 1)..self.current].to_owned();
        self.add_token(TokenKind::String(value));
        self.next();
    }

    fn scan_number(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    self.next();
                }
                _ => break,
            }
        }

        if let Some('.') = self.peek() {
            let mut temp_stream = self.stream.clone();
            temp_stream.next();
            if let Some('0'..='9') = temp_stream.peek() {
                self.next();

                while let Some(c) = self.peek() {
                    match c {
                        '0'..='9' => {
                            self.next();
                        }
                        _ => break,
                    }
                }
            }
        }

        let value = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenKind::Number(value));
    }

    fn scan_identifier(&mut self) {
        while let Some(c) = self.peek() {
            if is_alpha_numeric(c) {
                self.next();
            } else {
                break;
            }
        }

        let value = &self.source[self.start..self.current];

        use TokenKind::*;
        let tok = match value {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fun" => Fun,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            s => Identifier(s.to_owned()),
        };

        self.add_token(tok);
    }

    fn scan_multiline_comment(&mut self) {
        let mut nesting = 0;

        while let Some(c) = self.next() {
            match c {
                '/' => {
                    if self.expect_next('*') {
                        nesting += 1;
                    }
                }

                '*' => {
                    if self.expect_next('/') {
                        if nesting == 0 {
                            break;
                        } else {
                            nesting -= 1
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

fn is_alpha(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    }
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}
