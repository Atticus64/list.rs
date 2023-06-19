use list::Node;

fn main() {
    // let mut list = Node::from_vec(vec![1, 2, 3, 1]);
    let mut list = Node::from([1, 2, 3, 1, 6]);

    list.show();

    list.add(4);

    list.add(8);

    list.add(3);

    let mut n = Node::new(777);
    n.add(56);  
    n.add(999);  

    list.add_node(n);

    list.add(90);
    list.add(2);

    list.show();
    println!("--------------------");
    let find = list.find(777);
    let e = find.unwrap();

    println!("del: {:?}", e.value);
    list.show();
    println!("len: {:?}", list.lenght());
    println!("items: {:?}", list.items());

}
