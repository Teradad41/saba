use crate::renderer::js::ast::{Node, Program};
use alloc::rc::Rc;
use core::borrow::Borrow;
use core::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub struct JsRuntime {}

impl JsRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&mut self, program: &Program) {
        for node in program.body() {
            self.evaluate(&Some(node.clone()));
        }
    }

    fn evaluate(&mut self, node: &Option<Rc<Node>>) -> Option<RuntimeValue> {
        let node = match node {
            Some(node) => node,
            None => return None,
        };

        match node.borrow() {
            Node::ExpressionStatement(expr) => return self.evaluate(&expr),
            Node::AdditiveExpression {
                operator,
                left,
                right,
            } => {
                let left_value = match self.evaluate(&left) {
                    Some(value) => value,
                    None => return None,
                };

                let right_value = match self.evaluate(&right) {
                    Some(value) => value,
                    None => return None,
                };

                if operator == &'+' {
                    Some(left_value + right_value)
                } else if operator == &'-' {
                    Some(left_value - right_value)
                } else {
                    None
                }
            }
            Node::AssignmentExpression {
                operator: _,
                left: _,
                right: _,
            } => {
                // 後ほど実装
                None
            }
            Node::MemberExpression {
                object: _,
                property: _,
            } => {
                // 後ほど実装
                None
            }
            Node::NumericLiteral(value) => Some(RuntimeValue::Number(*value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number(u64),
}

impl Add<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn add(self, rhs: RuntimeValue) -> Self::Output {
        let (RuntimeValue::Number(lhs), RuntimeValue::Number(rhs)) = (&self, &rhs);
        return RuntimeValue::Number(lhs + rhs);
    }
}

impl Sub<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn sub(self, rhs: RuntimeValue) -> Self::Output {
        let (RuntimeValue::Number(lhs), RuntimeValue::Number(rhs)) = (&self, &rhs);
        return RuntimeValue::Number(lhs - rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::js::{ast::JsParser, token::JsLexer};
    use alloc::string::ToString;

    #[test]
    fn test_num() {
        let input = "42".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(42))];

        let mut i = 0;
        for node in ast.body() {
            let result = runtime.evaluate(&Some(node.clone()));
            assert_eq!(result, expected[i]);
            i += 1;
        }
    }

    #[test]
    fn test_add_num() {
        let input = "1 + 2".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(3))];

        let mut i = 0;
        for node in ast.body() {
            let result = runtime.evaluate(&Some(node.clone()));
            assert_eq!(result, expected[i]);
            i += 1;
        }
    }

    #[test]
    fn test_sub_num() {
        let input = "2 - 1".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(1))];

        let mut i = 0;
        for node in ast.body() {
            let result = runtime.evaluate(&Some(node.clone()));
            assert_eq!(result, expected[i]);
            i += 1;
        }
    }
}
