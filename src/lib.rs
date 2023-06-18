use std::cell::RefCell;
use std::rc::Rc;

type Next = Rc<RefCell<Option<Node>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub value: i32,
    pub next: Next,
}

fn create_node(value: i32) -> Node {
    Node {
        value,
        next: Rc::new(RefCell::new(None)),
    }
}


/// Create a new Rc reference to a node 
fn reference_node(node: Node) -> Rc<RefCell<Option<Node>>> {
    Rc::new(RefCell::new(Some(node)))
}

/// borrow the Rc reference and clone it with unwrap
fn borrow_clone(node: Next) -> Node {
    node.borrow().clone().unwrap()
}

impl Node {
    pub fn new(item: i32) -> Node {
        Node {
            value: item,
            next: Rc::new(RefCell::new(None)),
        }
    }

    pub fn add(&mut self, value: i32) -> Node {
        let mut node = create_node(value);

        if self.next.borrow().is_some() {
            node.next = Rc::new(RefCell::new(Some(self.next.borrow_mut().clone().unwrap())));
        }

        self.next = Rc::new(RefCell::new(Some(node.clone())));

        node
    }

    pub fn show(&self) {
        let mut current = Rc::new(RefCell::new(Some(self.clone())));

        loop {
            let node = current.borrow_mut().clone().unwrap();
            println!("{}", node.value);

            if node.next.borrow().is_none() {
                break;
            }

            current = node.next.clone();
        }
    }

    pub fn delete(&mut self, element: Node) -> bool {

        loop {
            if self.next.borrow().is_none() {
                break;
            }

            let next = borrow_clone(self.next.clone());

            if next == element {
                self.next = element.next;
                return true;
            }

            *self = self.next.take().unwrap();
            println!("{:?}", self.value);
        }

       
        false
    }

    pub fn exists(&self, element: Node) -> bool {
        let mut current = reference_node(self.clone());

        loop {
            let node = borrow_clone(current);

            if node.next.borrow().is_none() {
                break;
            }

            if node == element {
                return true;
            }

            current = node.next.clone();
        }

        false
    }

    pub fn find(&self, value: i32) -> Option<Node> {
        let mut current = reference_node(self.clone());

        let mut element: Option<Node> = None;
        loop {
            let n = borrow_clone(current);
            if n.value == value {
                element = Some(n);
                break;
            }

            if n.next.borrow().is_none() {
                break;
            }

            current = n.next.clone();
        }
        
        element

    }

    pub fn length(&self) -> u32 {
        let mut current = reference_node(self.clone());
        let mut count: u32 = 0;

        loop {
            let node = borrow_clone(current);

            count += 1;
            if node.next.borrow().is_none() {
                break;
            }

            current = node.next.clone();
        }

        count
    }
}
