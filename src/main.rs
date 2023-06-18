use list::Node;

fn main() {
    let mut list = Node::new(1);

    let n = list.add(4);

    list.add(8);


    list.add(3);

    list.add(2);

    list.show();

    println!("--------------------");
    println!("to delete: {}", n.value);
    list.delete(n);
    list.show();

}
