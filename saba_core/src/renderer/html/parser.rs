use crate::renderer::dom::node::{ElementKind, Node, Window};
use crate::renderer::html::token::{HTMLToken, HtmlTokenizer};
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    AfterHead,
    InBody,
    Text,
    AfterBody,
    AfterAfterBody,
}

#[derive(Debug, Clone)]
pub struct HtmlParser {
    window: Rc<RefCell<Window>>,
    mode: InsertionMode,
    original_insertion_mode: InsertionMode,
    stack_of_open_elements: Vec<Rc<RefCell<Node>>>,
    t: HtmlTokenizer,
}

impl HtmlParser {
    pub fn new(t: HtmlTokenizer) -> Self {
        Self {
            window: Rc::new(RefCell::new(Window::new())),
            mode: InsertionMode::Initial,
            original_insertion_mode: InsertionMode::Initial,
            stack_of_open_elements: Vec::new(),
            t,
        }
    }

    pub fn construct_tree(&mut self) -> Rc<RefCell<Window>> {
        let mut token = self.t.next();

        while token.is_some() {
            match self.mode {
                InsertionMode::Initial => {
                    if let Some(HTMLToken::Char(_)) = token {
                        token = self.t.next();
                        continue;
                    }

                    self.mode = InsertionMode::BeforeHtml;
                    continue;
                }
                InsertionMode::BeforeHtml => {
                    match token {
                        Some(HTMLToken::Char(c)) => {
                            if c == ' ' || c == '\n' {
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HTMLToken::StartTag {
                            ref tag,
                            self_closing: _,
                            ref attributes,
                        }) => {
                            if tag == "html" {
                                self.insert_element(tag, attributes.to_vec());
                                self.mode = InsertionMode::BeforeHead;
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HTMLToken::Eof) | None => {
                            return self.window.clone();
                        }
                        _ => {}
                    }
                    self.insert_element("html", Vec::new());
                    self.mode = InsertionMode::BeforeHead;
                    continue;
                }
                InsertionMode::BeforeHead => {
                    match token {
                        Some(HTMLToken::Char(c)) => {
                            if c == ' ' || c == '\n' {
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HTMLToken::StartTag {
                            ref tag,
                            self_closing: _,
                            ref attributes,
                        }) => {
                            if tag == "head" {
                                self.insert_element(tag, attributes.to_vec());
                                self.mode = InsertionMode::InHead;
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HTMLToken::Eof) | None => {
                            return self.window.clone();
                        }
                        _ => {}
                    }
                    self.insert_element("head", Vec::new());
                    self.mode = InsertionMode::InHead;
                    continue;
                }
                InsertionMode::InHead => {
                    match token {
                        Some(HTMLToken::Char(c)) => {
                            if c == ' ' || c == '\n' {
                                self.insert_char(c);
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HTMLToken::StartTag {
                            ref tag,
                            self_closing: _,
                            ref attributes,
                        }) => {
                            if tag == "style" || tag == "script" {
                                self.insert_element(tag, attributes.to_vec());
                                self.original_insertion_mode = self.mode;
                                self.mode = InsertionMode::Text;
                                token = self.t.next();
                                continue;
                            }

                            if tag == "body" {
                                self.pop_until(ElementKind::Head);
                                self.mode = InsertionMode::AfterHead;
                                continue;
                            }

                            if let Ok(_element_kind) = ElementKind::from_str(tag) {
                                self.pop_until(ElementKind::Head);
                                self.mode = InsertionMode::AfterHead;
                                continue;
                            }
                        }
                        Some(HTMLToken::EndTag { ref tag }) => {
                            if tag == "head" {
                                self.mode = InsertionMode::AfterHead;
                                token = self.t.next();
                                self.pop_until(ElementKind::Head);
                                continue;
                            }
                        }
                        Some(HTMLToken::Eof) | None => {
                            return self.window.clone();
                        }
                    }
                    token = self.t.next();
                    continue;
                }
                InsertionMode::AfterHead => {
                    match token {
                        Some(HTMLToken::Char(c)) => {
                            if c == ' ' || c == '\n' {
                                self.insert_char(c);
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HTMLToken::StartTag {
                            ref tag,
                            self_closing: _,
                            ref attributes,
                        }) => {
                            if tag == "body" {
                                self.insert_element(tag, attributes.to_vec());
                                token = self.t.next();
                                self.mode = InsertionMode::InBody;
                                continue;
                            }
                        }
                        Some(HTMLToken::Eof) | None => {
                            return self.window.clone();
                        }
                        _ => {}
                    }
                    self.insert_element("body", Vec::new());
                    self.mode = InsertionMode::InBody;
                    continue;
                }
                InsertionMode::InBody => match token {
                    Some(HTMLToken::EndTag { ref tag }) => match tag.as_str() {
                        "body" => {
                            self.mode = InsertionMode::AfterBody;
                            token = self.t.next();
                            if !self.contain_in_stack(ElementKind::Body) {
                                continue;
                            }
                            self.pop_until(ElementKind::Body);
                            continue;
                        }
                        "html" => {
                            if self.pop_current_node(ElementKind::Body) {
                                self.mode = InsertionMode::AfterBody;
                                assert!(self.pop_current_node(ElementKind::Html));
                            } else {
                                token = self.t.next();
                            }
                            continue;
                        }
                        _ => {
                            token = self.t.next();
                        }
                    },
                    Some(HTMLToken::Eof) | None => {
                        return self.window.clone();
                    }
                    _ => {}
                },
                InsertionMode::Text => {
                    match token {
                        Some(HTMLToken::Eof) | None => {
                            return self.window.clone();
                        }
                        Some(HTMLToken::EndTag { ref tag }) => {
                            if tag == "style" {
                                self.pop_until(ElementKind::Style);
                                self.mode = self.original_insertion_mode;
                                token = self.t.next();
                                continue;
                            }
                            if tag == "script" {
                                self.pop_until(ElementKind::Script);
                                self.mode = self.original_insertion_mode;
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HTMLToken::Char(c)) => {
                            self.insert_char(c);
                            token = self.t.next();
                            continue;
                        }
                        _ => {}
                    }

                    self.mode = self.original_insertion_mode;
                }
                InsertionMode::AfterBody => {
                    match token {
                        Some(HTMLToken::Char(_c)) => {
                            token = self.t.next();
                            continue;
                        }
                        Some(HTMLToken::EndTag { ref tag }) => {
                            if tag == "html" {
                                self.mode = InsertionMode::AfterAfterBody;
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HTMLToken::Eof) | None => {
                            return self.window.clone();
                        }
                        _ => {}
                    }

                    self.mode = InsertionMode::InBody;
                }
                InsertionMode::AfterAfterBody => {
                    match token {
                        Some(HTMLToken::Char(_c)) => {
                            token = self.t.next();
                            continue;
                        }
                        Some(HTMLToken::Eof) | None => {
                            return self.window.clone();
                        }
                        _ => {}
                    }

                    self.mode = InsertionMode::InBody;
                }
            }
        }

        self.window.clone()
    }
}