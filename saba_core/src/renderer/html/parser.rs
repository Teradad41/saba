use crate::renderer::dom::node::{Element, ElementKind, Node, NodeKind, Window};
use crate::renderer::html::attribute::Attribute;
use crate::renderer::html::token::{HTMLToken, HtmlTokenizer};
use alloc::rc::Rc;
use alloc::string::String;
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

// DOM ツリーを構築するための情報を格納する構造体
#[derive(Debug, Clone)]
pub struct HtmlParser {
    window: Rc<RefCell<Window>>,
    mode: InsertionMode,
    original_insertion_mode: InsertionMode, // とある状態に遷移した時に以前の挿入モードを保存する
    stack_of_open_elements: Vec<Rc<RefCell<Node>>>, // 構文解析中にブラウザが使用するスタック
    t: HtmlTokenizer,                       // t.next() メソッドを使用する
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

    fn create_element(&self, tag: &str, attributes: Vec<Attribute>) -> Node {
        Node::new(NodeKind::Element(Element::new(tag, attributes)))
    }

    fn create_char(&self, c: char) -> Node {
        let mut s = String::new();
        s.push(c);
        Node::new(NodeKind::Text(s))
    }

    // HTML の構造を解析して要素ノードを作成し、挿入先の位置を決定する
    // TODO: よくわかってない
    fn insert_element(&mut self, tag: &str, attributes: Vec<Attribute>) {
        let window = self.window.borrow();
        // 現在開いている要素スタックの最後のノードを取得
        let current = match self.stack_of_open_elements.last() {
            Some(n) => n.clone(),
            // スタックが空の場合はルートの要素
            None => window.document(),
        };
        let node = Rc::new(RefCell::new(self.create_element(tag, attributes)));

        // 参照しているノードに子ノードがある場合（ node から見て兄弟ノードがあるかどうか）
        if current.borrow().first_child().is_some() {
            let mut last_sibling = current.borrow().first_child();
            loop {
                last_sibling = match last_sibling {
                    Some(ref node) => {
                        if node.borrow().next_sibling().is_some() {
                            node.borrow().next_sibling()
                        } else {
                            break;
                        }
                    }
                    None => unimplemented!("last_sibling should be Some"),
                }
            }

            last_sibling
                .unwrap()
                .borrow_mut()
                .set_next_sibling(Some(node.clone()));
            node.borrow_mut().set_previous_sibling(Rc::downgrade(
                &current
                    .borrow()
                    .first_child()
                    .expect("failed to get a first child"),
            ));
        } else {
            current.borrow_mut().set_first_child(Some(node.clone()));
        }

        current.borrow_mut().set_last_child(Rc::downgrade(&node));
        node.borrow_mut().set_parent(Rc::downgrade(&current));

        // オープン要素スタックに追加
        self.stack_of_open_elements.push(node);
    }

    fn insert_char(&mut self, c: char) {
        let current = match self.stack_of_open_elements.last() {
            Some(n) => n.clone(),
            None => return,
        };

        // current がテキストノードの場合 → テキストノードに文字を追加
        if let NodeKind::Text(ref mut s) = current.borrow_mut().kind {
            s.push(c);
            return;
        };

        if c == '\n' || c == ' ' {
            return;
        }

        let node = Rc::new(RefCell::new(self.create_char(c)));

        if current.borrow().first_child().is_some() {
            current
                .borrow()
                .first_child()
                .unwrap()
                .borrow_mut()
                .set_next_sibling(Some(node.clone()));
            node.borrow_mut().set_previous_sibling(Rc::downgrade(
                &current
                    .borrow()
                    .first_child()
                    .expect("failed to get a first child"),
            ));
        } else {
            current.borrow_mut().set_first_child(Some(node.clone()));
        }

        current.borrow_mut().set_last_child(Rc::downgrade(&node));
        node.borrow_mut().set_parent(Rc::downgrade(&current));

        self.stack_of_open_elements.push(node);
    }

    fn pop_current_node(&mut self, element: ElementKind) -> bool {
        let current = match self.stack_of_open_elements.last() {
            Some(n) => n,
            None => return false,
        };

        if current.borrow().get_element_kind() == Some(element) {
            self.stack_of_open_elements.pop();
            return true;
        }

        false
    }

    fn pop_until(&mut self, element_kind: ElementKind) {
        assert!(
            self.contain_in_stack(element_kind),
            "stack doesn't have an element {:?}",
            element_kind
        );

        loop {
            let current = match self.stack_of_open_elements.pop() {
                Some(n) => n,
                None => return,
            };

            if current.borrow().get_element_kind() == Some(element_kind) {
                return;
            }
        }
    }

    fn contain_in_stack(&mut self, element_kind: ElementKind) -> bool {
        for i in 0..self.stack_of_open_elements.len() {
            if self.stack_of_open_elements[i].borrow().get_element_kind() == Some(element_kind) {
                return true;
            }
        }

        false
    }

    pub fn construct_tree(&mut self) -> Rc<RefCell<Window>> {
        let mut token = self.t.next();

        while token.is_some() {
            match self.mode {
                InsertionMode::Initial => {
                    // 文字トークンは無視する
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
                            // 開始タグトークンで html タグだった場合は、DOM ツリーに HTML 要素を追加
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
                    // それ以外の場合でも自動的に HTML 要素を追加
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
                    // head がなくても追加
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
                    // body タグがなくても追加
                    self.insert_element("body", Vec::new());
                    self.mode = InsertionMode::InBody;
                    continue;
                }
                InsertionMode::InBody => match token {
                    Some(HTMLToken::StartTag {
                        ref tag,
                        self_closing: _,
                        ref attributes,
                    }) => match tag.as_str() {
                        "p" => {
                            self.insert_element(tag, attributes.to_vec());
                            token = self.t.next();
                            continue;
                        }
                        _ => token = self.t.next(),
                    },
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
                // style タグと script タグが開始した後
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
                        // 終了タグ以外は文字をテキストノードとして DOM に追加する
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
                        // 文字トークンの時は無視
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
                        // DOM ツリーをリターン
                        Some(HTMLToken::Eof) | None => {
                            return self.window.clone();
                        }
                        _ => {}
                    }
                    // パースの失敗
                    self.mode = InsertionMode::InBody;
                }
            }
        }

        // 構築し終えたら、ルートノードを持つ Window オブジェクトを返す
        self.window.clone()
    }
}
