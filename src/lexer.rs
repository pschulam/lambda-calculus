use crate::source::Source;

pub struct Lexer {
    source: Source,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Lambda,
    Dot,
    Lparen,
    Rparen,
    Equals,
    Identifier(String),
    Eof,
}

impl Lexer {
    pub fn new(source: Source) -> Lexer {
        Lexer {
            source: source,
        }
    }

    pub fn get_token(&mut self) -> Token {
        loop {
            match self.scan() {
                None => continue,
                Some(token) => return token,
            }
        }
    }

    pub fn scan(&mut self) -> Option<Token> {
        self.source.reset();

        match self.source.advance() {
            None => None,
            Some(c) => {
                match c {
                    '\\' => return Some(Token::Lambda),
                    '.'  => return Some(Token::Dot),
                    '('  => return Some(Token::Lparen),
                    ')'  => return Some(Token::Rparen),
                    '='  => return Some(Token::Equals), 
                    ' '  => return None,
                    '\r' => return None,
                    '\n' => return None,
                    '\t' => return None,
                    _    => return self.identifier(),
                }
            },
        }
    }

    fn identifier(&mut self) -> Option<Token> {
        while let Some(true) = self.source.peek().map(|c| c.is_ascii_alphanumeric()) {
            self.source.advance();
        }
        Some(Token::Identifier(self.source.extract().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_lex_individual_tokens() {
        let examples = vec![
            (r"\", Token::Lambda),
            (r".", Token::Dot),
            (r"(", Token::Lparen),
            (r")", Token::Rparen),
            (r"=", Token::Equals),
            (r"def", Token::Identifier("def".to_string())),
            (r"x", Token::Identifier("x".to_string())),
            (r"xx", Token::Identifier("xx".to_string())),
            (r"   \", Token::Lambda),
        ];

        for (text, expected) in examples {
            let source = Source::new(&text);
            let mut lexer = Lexer::new(source);
            let token = lexer.get_token();
            assert_eq!(token, expected);
        }
    }
}