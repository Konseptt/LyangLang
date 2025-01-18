use crate::error::NepalError;
use crate::token::Token;

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

    pub fn tokenize(&mut self) -> Result<Vec<Token>, NepalError> {
        let mut tokens = Vec::new();
        while self.position < self.input.len() {
            self.skip_whitespace();
            if self.position >= self.input.len() {
                break;
            }

            // Skip comments
            if self.position + 1 < self.input.len() 
                && self.input[self.position] == '/' 
                && self.input[self.position + 1] == '/' {
                self.skip_until_newline();
                continue;
            }

            // Try to match keywords from longest to shortest to avoid partial matches
            match self.peek_char() {
                'b' if self.match_keyword("bhane") => tokens.push(Token::Bhane),
                'b' if self.match_keyword("babaal") => tokens.push(Token::Babaal),
                'b' if self.match_keyword("bol mug") => tokens.push(Token::BolMug),
                'b' if self.match_keyword("bhan") => tokens.push(Token::Bhan),
                'o' if self.match_keyword("oi mug") => tokens.push(Token::OiMug),
                'm' if self.match_keyword("mug") => tokens.push(Token::Mug),
                'j' if self.match_keyword("jod") => tokens.push(Token::Jod),
                'g' if self.match_keyword("ghata") => tokens.push(Token::Ghata),
                'l' if self.match_keyword("lai") => tokens.push(Token::Lai),
                'l' if self.match_keyword("laamo") => tokens.push(Token::Laamo),
                'y' if self.match_keyword("yedi") => tokens.push(Token::Yedi),
                's' if self.match_keyword("sakiyo") => tokens.push(Token::Sakiyo),
                'a' if self.match_keyword("aile") => tokens.push(Token::Aile),
                'f' if self.match_keyword("feri") => tokens.push(Token::Feri),
                '=' => {
                    self.position += 1;
                    if self.position < self.input.len() && self.input[self.position] == '=' {
                        self.position += 1;
                        tokens.push(Token::IsEquals);
                    } else {
                        tokens.push(Token::Equals);
                    }
                }
                '!' => {
                    self.position += 1;
                    if self.position < self.input.len() && self.input[self.position] == '=' {
                        self.position += 1;
                        tokens.push(Token::NotEquals);
                    } else {
                        return Err(NepalError::LexError("Expected '=' after '!'"));
                    }
                }
                '+' => {
                    self.position += 1;
                    tokens.push(Token::Plus);
                }
                ',' => {
                    self.position += 1;
                    tokens.push(Token::Comma);
                }
                '"' => {
                    tokens.push(Token::String(self.read_string()?));
                }
                c if c.is_alphabetic() => {
                    tokens.push(Token::Identifier(self.read_identifier()));
                }
                c if c.is_numeric() => {
                    tokens.push(Token::Number(self.read_number()?));
                }
                '\n' | '\r' => {
                    self.position += 1;
                    continue;
                }
                _ => return Err(NepalError::LexError("Invalid character found")),
            }
        }
        Ok(tokens)
    }

    fn peek_char(&self) -> char {
        self.input[self.position]
    }

    fn match_keyword(&mut self, keyword: &str) -> bool {
        let chars: Vec<char> = keyword.chars().collect();
        if self.input[self.position..].starts_with(&chars) {
            self.position += keyword.len();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_alphabetic() {
            self.position += 1;
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> Result<i32, NepalError> {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_numeric() {
            self.position += 1;
        }
        let num_str: String = self.input[start..self.position].iter().collect();
        num_str.parse().map_err(|_| NepalError::LexError("Invalid number"))
    }

    fn read_string(&mut self) -> Result<String, NepalError> {
        self.position += 1; // Skip opening quote
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position] != '"' {
            self.position += 1;
        }
        if self.position >= self.input.len() {
            return Err(NepalError::LexError("Unterminated string"));
        }
        let result = self.input[start..self.position].iter().collect();
        self.position += 1; // Skip closing quote
        Ok(result)
    }

    fn skip_until_newline(&mut self) {
        while self.position < self.input.len() && !matches!(self.input[self.position], '\n' | '\r') {
            self.position += 1;
        }
    }
}
