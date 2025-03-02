use std::rc::Rc;
use std::cell::RefCell;

pub struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value: value,
            next: None,
        }
    }
}

pub struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn append(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        if let Some(_head) = &self.head {
            let mut current = self.head.clone();

            while let Some(next) = &current.clone().expect("error").borrow().next {
                current = Some(next.clone());
            }
            current.expect("error").borrow_mut().next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
    }

    pub fn print(&self) {
        let mut current = self.head.clone();

        while let Some(node) = current {
            println!("{} -> ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!("null");
    }
}