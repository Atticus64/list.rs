
use list::Node;

#[test]
fn test_list_insert_10_items() {
    let mut list = Node::new(1);
    for _ in 0..100 {
        list.add(100);
    }
    assert_eq!(list.lenght(), 101);
}
