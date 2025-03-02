use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Display;

pub struct BinaryNode<T> {
    value: T,
    left: Option<Rc<RefCell<BinaryNode<T>>>>,
    right: Option<Rc<RefCell<BinaryNode<T>>>>,
}

impl<T> BinaryNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct BinaryTree<T> {
    root: Option<Rc<RefCell<BinaryNode<T>>>>,
}

impl<T: Ord + Display> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(BinaryNode::new(value)));

        if let Some(root) = &self.root {
            Self::insert_node(root.clone(), new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    fn insert_node(node: Rc<RefCell<BinaryNode<T>>>, new_node: Rc<RefCell<BinaryNode<T>>>) {
        let mut node_borrow = node.borrow_mut();

        if new_node.borrow().value < node_borrow.value {
            if let Some(left) = &node_borrow.left {
                Self::insert_node(left.clone(), new_node);
            } else {
                node_borrow.left = Some(new_node);
            }
        } else {
            if let Some(right) = &node_borrow.right {
                Self::insert_node(right.clone(), new_node);
            } else {
                node_borrow.right = Some(new_node);
            }
        }
    }

    pub fn print_in_order(&self) {
        Self::print_node(&self.root);
        println!();
    }

    fn print_node(node: &Option<Rc<RefCell<BinaryNode<T>>>>) {
        if let Some(n) = node {
            let n = n.borrow();
            Self::print_node(&n.left);
            println!("{}", n.value);
            Self::print_node(&n.right);
        }
    }
}