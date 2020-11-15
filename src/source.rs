pub struct Source {
    text: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Source {
    pub fn new(text: &str) -> Source {
        Source {
            text: text.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.current >= self.text.len()
    }

    pub fn advance(&mut self) -> Option<char> {
        match self.is_finished() {
            true => None,
            false => {
                self.current += 1;
                Some(self.text[self.current - 1])
            }
        }
    }

    pub fn peek(&self) -> Option<char> {
        match self.is_finished() {
            true => None,
            false => Some(self.text[self.current]),
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn it_starts_in_the_correct_state() {
        let examples = vec![
            ("hello".to_string(), false),
            ("".to_string(), true),
        ];

        for (text, expected) in examples {
            let source = Source::new(&text);
            assert_eq!(source.is_finished(), expected);
        }
    }

    #[test]
    fn it_can_advance_until_finished() {
        let text = "hello".to_string();
        let mut source = Source::new(&text);
        let mut chars = vec![];

        while let Some(c) = source.advance() {
            chars.push(c);
        }

        let collected: String = chars.iter().collect();
        assert_eq!(text, collected);
    }

    #[test]
    fn it_can_support_a_whitespace_tokenizer() {
        let text = indoc! {"
            hello world this
            is some sample text.
        "};

        let mut source = Source::new(&text);
        let expected = vec![
            "hello",
            "world",
            "this",
            "is",
            "some",
            "sample",
            "text.",
        ];

        let mut tokens = vec![];
        let mut in_token = !source.peek().unwrap().is_whitespace();
        let mut buffer = vec![];

        while let Some(c) = source.advance() {
            if in_token {
                if c.is_whitespace() {
                    let token: String = buffer.iter().collect();
                    tokens.push(token);
                    buffer.clear();
                    in_token = false;
                } else {
                    buffer.push(c);
                }
            } else {
                if !c.is_whitespace() {
                    buffer.push(c);
                    in_token = true;
                }
            }
        }

        assert_eq!(tokens.len(), expected.len());

        for (t, e) in tokens.iter().zip(expected.iter()) {
            assert_eq!(t, e);
        }        
    }
}