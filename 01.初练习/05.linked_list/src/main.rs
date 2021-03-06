use std::rc::Rc;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&self, data: T) -> Self {
        LinkedList {
            head: Some(Rc::new(Node {
                data,
                next: self.head.clone(),
            })),
        }
    }
}

fn main() {
    let list_of_nums = LinkedList::new().append(1).append(2);
    println!("{:?}", list_of_nums);

    let list_of_strs = LinkedList::new().append("test1").append("test2");
    println!("{:?}", list_of_strs);
}
