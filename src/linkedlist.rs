use std::rc::Rc;
use std::rc::Weak;
use std::cell::Cell;

struct Node<T> {
    value: T,
    next: Option<Rc<Cell<Node<T>>>>,
    prev: Weak<Cell<Node<T>>>,
}

pub struct LinkedList<T> {
    left: Option<Rc<Cell<Node<T>>>>,
    right: Weak<Cell<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            left: None,
            right: Weak::new(),
        }
    }

    pub fn push_right(&mut self, item: T) {
        match self.right.upgrade() {
            Some(node_ref) => {
                let new_node = Node {
                    value: item,
                    next: None,
                    prev: Rc::downgrade(&node_ref),
                };

                let r1 = Rc::new(Cell::new(new_node));
                (*(*node_ref).get_mut()).next = Some(r1.clone());
                self.right = Rc::downgrade(&r1);
            },
            None => {
                let new_node = Node {
                    value: item,
                    next: None,
                    prev: Weak::new(),
                };

                let r1 = Rc::new(Cell::new(new_node));
                self.left = Some(r1);
                self.right = Rc::downgrade(&r1);
            },
        }
    }

    pub fn push_left(&mut self, item: T) {
        match self.left.take() {
            Some(node_ref) => {
                let new_node = Node {
                    value: item,
                    next: Some(node_ref.clone()),
                    prev: Weak::new(),
                };

                let r1 = Rc::new(Cell::new(new_node));
                self.left.replace(r1);
            },
            None => {
                let new_node = Node {
                    value: item,
                    next: None,
                    prev: Weak::new(),
                };

                let r1 = Rc::new(Cell::new(new_node));
                self.left.replace(r1);
            },
        }
    }

    pub fn pop_right(&mut self) -> Option<T> {
        self.right.upgrade().map(|node_ref| {
        })
    }

    pub fn pop_left(&mut self) -> Option<T> {
    }
}
