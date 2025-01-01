struct Node<T> {
    value: T,
    link: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            link: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.link;
            node.value
        })
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.link;
        }
        count
    }

    fn print(&self)
    where
        T: std::fmt::Display,
    {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.link;
        }
        println!("End");
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    //push value from front
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    //list node
    println!("list contents:");
    list.print();

    //check lenght
    println!("Length: {}", list.len());

    //delete from front
    println!("Popped: {:?}", list.pop_front());
    println!("Popped: {:?}", list.pop_front());

    // Print the list again
    println!("List after popping:");
    list.print();

    // Check if empty
    println!("Is empty: {}", list.is_empty());
}
