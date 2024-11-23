use alloc::rc::Rc;
use alloc::rc::Weak;
use alloc::string::String;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Window {
    document: Rc<RefCell<Node>>,
}

impl Window {
    pub fn new() -> Self {
        let window = Self {
            document: Rc::new(RefCell::new(Node::new(NodeKind::Document))),
        };

        window
            .document
            .borrow_mut()
            .set_window(Rc::downgrade(&Rc::new(RefCell::new(window.clone()))));
        window
    }

	pub fn document(&self) -> Rc<RefCell<Node>> {
		self.document.clone()
	}
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Document,
    Element(Element),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    window: Weak<RefCell<Window>>,
    parent: Weak<RefCell<Node>>,
    first_child: Option<Rc<RefCell<Node>>>,
    last_child: Weak<RefCell<Node>>,
    previous_sibling: Weak<RefCell<Node>>,
    next_sibling: Weak<RefCell<Node>>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            window: Weak::new(),
            parent: Weak::new(),
            first_child: None,
            last_child: Weak::new(),
            previous_sibling: Weak::new(),
            next_sibling: None,
        }
    }

    pub fn set_parent(&mut self, parent: Weak<RefCell<Node>>) {
        self.parent = parent;
    }

    pub fn parent(&self) -> Weak<RefCell<Node>> {
        self.parent.clone()
    }

    pub fn set_first_child(&mut self, first_child: Rc<RefCell<Node>>) {
        self.first_child = Some(first_child);
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.first_child.as_ref().cloned()
    }
}
