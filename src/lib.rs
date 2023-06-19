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

    pub fn from(slice: [i32;5]) -> Node {

        let mut n = Node::new(slice[0]);

        let len = slice.len();
        for (i, item) in slice.iter().enumerate().take(len)  {
            if i == 0 {
                continue;
            }
            n.add(*item);
        }

        n
    }

    pub fn from_vec(vec: Vec<i32>) -> Node {
        if vec.is_empty() {
            return Node {
                value: 0,
                next: None,
            }

        }

        let mut n = Node::new(*vec.first().unwrap());

        for i in 1..vec.len() {
            n.add(*vec.get(i).unwrap());
        }

        n
    }

    pub fn add(&mut self, value: i32) -> Node {
        let mut node = create_node(value);

        if self.next.is_some() {
            node.next = reference_node(self.next.clone().unwrap().borrow().clone());
        }

        self.next = reference_node(node.clone());

        node
    }

    pub fn add_node(&mut self, new_node: Node) {

        let mut current = reference_node(self.clone());
        while let Some(node) = current.clone() {

            if node.borrow().next.is_none() {
                break;
            }

            current = node.borrow_mut().next.clone();
        }

        current.clone().unwrap().borrow_mut().next = new_node.next;
        current.unwrap().borrow_mut().value = new_node.value;
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

    pub fn find(&self, value: i32) -> Option<Node> {
        let mut current = reference_node(self.clone());
    
        let mut element: Option<Node> = None;
        loop {
            let v = current.unwrap();
            let n = v.borrow();
            if n.value == value {
                element = Some(n.clone());
                break;
            }
    
            if n.next.is_none() {
                break;
            }
    
            current = n.next.clone();
        }
       
        element
    
    }

    pub fn lenght(&self) -> i32 {
        let mut current = reference_node(self.clone());

        let mut count = 0;
        while let Some(node) = current {

            current = node.borrow().next.clone();
            count += 1;
        }

        count
    }

    pub fn items(&self) -> Vec<i32> {
        let mut current = reference_node(self.clone());
        let mut items = Vec::new();

        while let Some(node) = current {

            current = node.borrow().next.clone();
            items.push(node.borrow().value);
        }

        items
    }


}
