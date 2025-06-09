use alloc::{
    string::{String, ToString},
    vec::Vec,
};

static RESERVED_WORDS: [&str; 1] = ["var"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Punctuator(char),
    Number(u64),
    Identifier(String),
    Keyword(String),
    StringLiteral(String),
}

pub struct JsLexer {
    pos: usize,
    input: Vec<char>,
}

impl JsLexer {
    pub fn new(js: String) -> Self {
        Self {
            pos: 0,
            input: js.chars().collect(),
        }
    }

    fn consume_number(&mut self) -> u64 {
        let mut num = 0;

        loop {
            if self.pos >= self.input.len() {
                return num;
            }

            let c = self.input[self.pos];

            match c {
                '0'..='9' => {
                    num = num * 10 + (c.to_digit(10).unwrap() as u64);
                    self.pos += 1;
                }
                _ => break,
            }
        }

        return num;
    }

    fn contains(&self, keyword: &str) -> bool {
        for i in 0..keyword.len() {
            if keyword
                .chars()
                .nth(i)
                .expect("failed to access to i-th char")
                != self.input[self.pos + i]
            {
                return false;
            }
        }

        true
    }

    fn consume_identifier(&mut self) -> String {
        let mut result = String::new();

        loop {
            if self.pos >= self.input.len() {
                return result;
            }

            if self.input[self.pos].is_ascii_alphanumeric() || self.input[self.pos] == '$' {
                result.push(self.input[self.pos]);
                self.pos += 1;
            } else {
                return result;
            }
        }
    }

    fn consume_string(&mut self) -> String {
        let mut result = String::new();
        self.pos += 1;

        loop {
            if self.pos >= self.input.len() {
                return result;
            }

            if self.input[self.pos] == '"' {
                self.pos += 1;
                return result;
            }

            result.push(self.input[self.pos]);
            self.pos += 1;
        }
    }

    fn check_reserved_word(&self) -> Option<String> {
        for word in RESERVED_WORDS {
            if self.contains(word) {
                return Some(word.to_string());
            }
        }

        None
    }
}

impl Iterator for JsLexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        // ホワイトスペースまたは改行文字が続く限り、次の位置に進める
        while self.input[self.pos] == ' ' || self.input[self.pos] == '\n' {
            self.pos += 1;

            if self.pos >= self.input.len() {
                return None;
            }
        }

        if let Some(keyword) = self.check_reserved_word() {
            self.pos += keyword.len();
            let token = Some(Token::Keyword(keyword));
            return token;
        }

        let c = self.input[self.pos];
        let token = match c {
            'a'..='z' | 'A'..='Z' | '_' | '$' => Token::Identifier(self.consume_identifier()),
            '+' | '-' | ';' | '=' | '.' | ',' | '(' | ')' | '{' | '}' => {
                let t = Token::Punctuator(c);
                self.pos += 1;
                t
            }
            '0'..='9' => Token::Number(self.consume_number()),
            '"' => Token::StringLiteral(self.consume_string()),
            _ => unimplemented!("char {:?} is not implemented yet", c),
        };

        Some(token)
    }
}

// JsLexer のテスト
#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_empty() {
        let input = "".to_string();
        let mut lexer = JsLexer::new(input).peekable();
        assert!(lexer.peek().is_none());
    }

    #[test]
    fn test_num() {
        let input = "42".to_string();
        let mut lexer = JsLexer::new(input).peekable();

        let expected = [Token::Number(42)].to_vec();
        let mut i = 0;
        while lexer.peek().is_some() {
            assert_eq!(Some(expected[i].clone()), lexer.next());
            i += 1;
        }
        assert!(lexer.peek().is_none());
    }

    #[test]
    fn test_add_num() {
        let input = "1 + 2".to_string();
        let mut lexer = JsLexer::new(input).peekable();

        let expected = [Token::Number(1), Token::Punctuator('+'), Token::Number(2)].to_vec();
        let mut i = 0;
        while lexer.peek().is_some() {
            assert_eq!(Some(expected[i].clone()), lexer.next());
            i += 1;
        }
        assert!(lexer.peek().is_none());
    }

    #[test]
    fn test_assign_variable() {
        let input = "var foo=\"bar\";".to_string();
        let mut lexer = JsLexer::new(input).peekable();
        let expected = [
            Token::Keyword("var".to_string()),
            Token::Identifier("foo".to_string()),
            Token::Punctuator('='),
            Token::StringLiteral("bar".to_string()),
            Token::Punctuator(';'),
        ]
        .to_vec();

        let mut i = 0;
        while lexer.peek().is_some() {
            assert_eq!(Some(expected[i].clone()), lexer.next());
            i += 1;
        }
        assert!(lexer.peek().is_none());
    }

    #[test]
    fn test_add_variable_and_num() {
        let input = "var foo=42; var result=foo+1;".to_string();
        let mut lexer = JsLexer::new(input).peekable();
        let expected = [
            Token::Keyword("var".to_string()),
            Token::Identifier("foo".to_string()),
            Token::Punctuator('='),
            Token::Number(42),
            Token::Punctuator(';'),
            Token::Keyword("var".to_string()),
            Token::Identifier("result".to_string()),
            Token::Punctuator('='),
            Token::Identifier("foo".to_string()),
            Token::Punctuator('+'),
            Token::Number(1),
            Token::Punctuator(';'),
        ]
        .to_vec();

        let mut i = 0;
        while lexer.peek().is_some() {
            assert_eq!(Some(expected[i].clone()), lexer.next());
            i += 1;
        }
        assert!(lexer.peek().is_none());
    }
}
