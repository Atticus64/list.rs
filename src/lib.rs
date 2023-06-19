use std::cell::RefCell;
use std::rc::Rc;

type Next = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub value: i32,
    pub next: Next,
}

/// Create a new node with the value 
fn create_node(value: i32) -> Node {
    Node {
        value,
        next: None,
    }
}


/// Create a new Rc reference to a node 
fn reference_node(node: Node) -> Option<Rc<RefCell<Node>>> {
    Some(Rc::new(RefCell::new(node)))
}

impl Node {
    pub fn new(item: i32) -> Node {
        Node {
            value: item,
            next: None,
        }
    }

    pub fn add(&mut self, value: i32) -> Node {
        let mut node = create_node(value);

        if self.next.is_some() {
            node.next = reference_node(self.next.clone().unwrap().borrow().clone());
        }

        self.next = reference_node(node.clone());

        node
    }

    pub fn addNode(&mut self, mut new_node: Node) {
        if self.next.is_some() {
            new_node.next = reference_node(self.next.clone().unwrap().borrow().clone());
        }

        self.next = reference_node(new_node);
    }

    pub fn print_node(&self) {
        print!("{}->", self.value);
    }

    pub fn show(&self) {
        let mut current = reference_node(self.clone());

        while let Some(node) = current {
            let n = node.borrow();
            n.print_node();

            current = node.borrow().next.clone();
        }
        println!("null");
    }

    pub fn pop(&mut self) -> Option<Node> {
        let mut current = reference_node(self.clone());
        while let Some(node) = current {
            let temp = node.borrow().next.clone();
            let next = temp.unwrap().borrow().next.clone();

            if next.is_none() {
                let data = node.borrow().next.clone().unwrap();
                node.borrow_mut().next = None;
                return Some(data.borrow().clone());
            }

            current = node.borrow().next.clone();
        }

        None
    }

    pub fn delete(&mut self, node_to_delete: &Node) -> Option<Node> {
        let mut current = reference_node(self.clone());
        while let Some(node) = current {
            let next = node.borrow().next.clone().unwrap().borrow().clone();
            let replace = next.next.clone();

            if &next == node_to_delete {
                let data = node.borrow().next.clone().unwrap();
                node.borrow_mut().next = replace;
                return Some(data.borrow().clone());
            }

            current = node.borrow().next.clone();
        }

        None
    }

    // pub fn exists(&self, element: Node) -> bool {
    //     let mut current = reference_node(self.clone());
    //
    //     loop {
    //         let node = borrow_clone(current);
    //
    //         if node.next.borrow().is_none() {
    //             break;
    //         }
    //
    //         if node == element {
    //             return true;
    //         }
    //
    //         current = node.next.clone();
    //     }
    //
    //     false
    // }
    //
    // pub fn find(&self, value: i32) -> Option<Node> {
    //     let mut current = reference_node(self.clone());
    //
    //     let mut element: Option<Node> = None;
    //     loop {
    //         let n = borrow_clone(current);
    //         if n.value == value {
    //             element = Some(n);
    //             break;
    //         }
    //
    //         if n.next.borrow().is_none() {
    //             break;
    //         }
    //
    //         current = n.next.clone();
    //     }
    //    
    //     element
    //
    // }

    // pub fn length(&self) -> u32 {
    //     let mut current = reference_node(self.clone());
    //     let mut count: u32 = 0;
    //
    //     loop {
    //         let node = borrow_clone(current);
    //
    //         count += 1;
    //         if node.next.borrow().is_none() {
    //             break;
    //         }
    //
    //         current = node.next.clone();
    //     }
    //
    //     count
    // }
}
