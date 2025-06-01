use alloc::string::String;
use alloc::vec::Vec;

// CSS のトークン（本来は 24 種類ある）
#[derive(Debug, Clone, PartialEq)]
pub enum CssToken {
    HashToken(String),
    Delim(char),
    Number(f64),
    Colon,
    SemiColon,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurly,
    CloseCurly,
    Ident(String),
    StringToken(String),
    AtKeyword(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CssTokenizer {
    pos: usize,
    input: Vec<char>,
}

impl CssTokenizer {
    pub fn new(css: String) -> Self {
        Self {
            pos: 0,
            input: css.chars().collect(),
        }
    }

    // 再びダブルクォーテーションまたはシングルクォーテーションが現れるまで入力を文字として消費する
    fn consume_string_token(&mut self) -> String {
        let mut s = String::new();

        // 開始位置を記録（クォートの文字）
        let quote_char = self.input[self.pos];

        // 開始引用符をスキップ
        self.pos += 1;

        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            if c == quote_char {
                // 終了引用符を見つけたので、その位置で停止
                break;
            } else {
                s.push(c);
                self.pos += 1;
            }
        }
        s
    }

    // 数字またはピリオドが出続けている間、数字として解釈する
    // それ以外が来たら数字を返すメソッド
    fn consume_numeric_token(&mut self) -> f64 {
        let mut num = 0f64;
        let mut floating = false;
        let mut floating_digit = 1f64;

        loop {
            if self.pos >= self.input.len() {
                return num;
            }

            let c = self.input[self.pos];

            match c {
                '0'..='9' => {
                    if floating {
                        floating_digit *= 1f64 / 10f64;
                        num += (c.to_digit(10).unwrap() as f64) * floating_digit
                    } else {
                        num = num * 10.0 + (c.to_digit(10).unwrap() as f64);
                    }
                    self.pos += 1;
                }
                '.' => {
                    floating = true;
                    self.pos += 1;
                }
                _ => break,
            }
        }
        num
    }

    // 文字、数字、ハイフンまたはアンダースコアが出続けている間、識別子として解釈する
    // それ以外が出てきたら今までの文字を返してメソッドを終了
    fn consume_ident_token(&mut self) -> String {
        let mut s = String::new();
        s.push(self.input[self.pos]);

        loop {
            self.pos += 1;
            let c = self.input[self.pos];
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => {
                    s.push(c);
                }
                _ => break,
            }
        }
        s
    }
}

impl Iterator for CssTokenizer {
    type Item = CssToken;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pos >= self.input.len() {
                return None;
            }

            let c = self.input[self.pos];
            let token = match c {
                '(' => {
                    self.pos += 1;
                    CssToken::OpenParenthesis
                }
                ')' => {
                    self.pos += 1;
                    CssToken::CloseParenthesis
                }
                ',' => {
                    self.pos += 1;
                    CssToken::Delim(',')
                }
                '.' => {
                    self.pos += 1;
                    CssToken::Delim('.')
                }
                ':' => {
                    self.pos += 1;
                    CssToken::Colon
                }
                ';' => {
                    self.pos += 1;
                    CssToken::SemiColon
                }
                '{' => {
                    self.pos += 1;
                    CssToken::OpenCurly
                }
                '}' => {
                    self.pos += 1;
                    CssToken::CloseCurly
                }
                ' ' | '\n' => {
                    self.pos += 1;
                    continue;
                }
                '"' | '\'' => {
                    let value = self.consume_string_token();
                    // consume_string_token 内部ですでに終了引用符の位置まで移動しているので、
                    // 終了引用符をスキップする
                    self.pos += 1;
                    CssToken::StringToken(value)
                }
                '0'..='9' => CssToken::Number(self.consume_numeric_token()),
                // 常に #ID のセレクタとして扱う
                '#' => {
                    let value = self.consume_ident_token();
                    CssToken::HashToken(value)
                }
                '-' => CssToken::Ident(self.consume_ident_token()),
                // 次の3文字が識別子として有効な場合、 at-keyword-token を生成する
                '@' => {
                    if self.pos + 3 < self.input.len()
                        && self.input[self.pos + 1].is_ascii_alphabetic()
                        && self.input[self.pos + 2].is_ascii_alphabetic()
                        && self.input[self.pos + 3].is_ascii_alphabetic()
                    {
                        // skip '@'
                        self.pos += 1;
                        let t = CssToken::AtKeyword(self.consume_string_token());
                        t
                    } else {
                        self.pos += 1;
                        CssToken::Delim('@')
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => CssToken::Ident(self.consume_ident_token()),
                _ => {
                    unimplemented!("char {} is not supported yet", c);
                }
            };

            return Some(token);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_empty() {
        let style = "".to_string();
        let mut t = CssTokenizer::new(style);
        assert!(t.next().is_none());
    }

    #[test]
    fn test_one_rule() {
        let style = "p { color: red; }".to_string();
        let mut t = CssTokenizer::new(style);
        let expected = [
            CssToken::Ident("p".to_string()),
            CssToken::OpenCurly,
            CssToken::Ident("color".to_string()),
            CssToken::Colon,
            CssToken::Ident("red".to_string()),
            CssToken::SemiColon,
            CssToken::CloseCurly,
        ];
        for e in expected {
            assert_eq!(Some(e.clone()), t.next());
        }
        assert!(t.next().is_none());
    }

    #[test]
    fn test_id_selector() {
        let style = "#id { color: red; }".to_string();
        let mut t = CssTokenizer::new(style);
        let expected = [
            CssToken::HashToken("#id".to_string()),
            CssToken::OpenCurly,
            CssToken::Ident("color".to_string()),
            CssToken::Colon,
            CssToken::Ident("red".to_string()),
            CssToken::SemiColon,
            CssToken::CloseCurly,
        ];
        for e in expected {
            assert_eq!(Some(e.clone()), t.next());
        }
        assert!(t.next().is_none());
    }

    #[test]
    fn test_class_selector() {
        let style = ".class { color: red; }".to_string();
        let mut t = CssTokenizer::new(style);
        let expected = [
            CssToken::Delim('.'),
            CssToken::Ident("class".to_string()),
            CssToken::OpenCurly,
            CssToken::Ident("color".to_string()),
            CssToken::Colon,
            CssToken::Ident("red".to_string()),
            CssToken::SemiColon,
            CssToken::CloseCurly,
        ];
        for e in expected {
            assert_eq!(Some(e.clone()), t.next());
        }
        assert!(t.next().is_none());
    }

    #[test]
    fn test_multiple_rules() {
        let style = "p { content: \"Hey\"; } h1 { font-size: 40; color: blue; }".to_string();
        let mut t = CssTokenizer::new(style);
        let expected = [
            CssToken::Ident("p".to_string()),
            CssToken::OpenCurly,
            CssToken::Ident("content".to_string()),
            CssToken::Colon,
            CssToken::StringToken("Hey".to_string()),
            CssToken::SemiColon,
            CssToken::CloseCurly,
            CssToken::Ident("h1".to_string()),
            CssToken::OpenCurly,
            CssToken::Ident("font-size".to_string()),
            CssToken::Colon,
            CssToken::Number(40.0),
            CssToken::SemiColon,
            CssToken::Ident("color".to_string()),
            CssToken::Colon,
            CssToken::Ident("blue".to_string()),
            CssToken::SemiColon,
            CssToken::CloseCurly,
        ];

        // 各トークンを個別に検証し、どこで失敗したかを明確にする
        for (i, expected_token) in expected.iter().enumerate() {
            let actual_token = t.next();
            assert_eq!(
                Some(expected_token.clone()),
                actual_token,
                "Token mismatch at position {}: expected {:?}, got {:?}",
                i,
                expected_token,
                actual_token
            );
        }
        assert!(t.next().is_none(), "Expected no more tokens");
    }
}
