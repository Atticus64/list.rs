use list::Node;

fn main() {
    let mut node = Node::new(1);

    let second = node.add(2);
    node.add(3);
    let _n = node.delete(&second); // delete 2 and returns it
    node.show(); // 1->3->null

}
