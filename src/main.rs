use list::Node;

fn main() {
    let mut list = Node::new(1);

    list.add(4);

    list.add(8);

    let n = list.add(3);

    let new_node = Node::new(90);

    list.addNode(new_node);

    list.add(2);

    list.show();

    println!("--------------------");
    let last = list.pop();
    let delete = list.delete(&n);
    list.show();
    println!("last: {:?}", last);
    println!("delete: {:?}", delete);

}
