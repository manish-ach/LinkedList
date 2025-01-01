// generic over type T and contains a value and a link to the next node
struct Node<T> {
    value: T,
    link: Option<Box<Node<T>>>, // Optional link to next node (None if end of list)
                                //heap allocated using Box
}
//similar to Node, maintaining refence to head
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

//implementation block for LinkedList methods
impl<T> LinkedList<T> {
    // create a new empty list
    fn new() -> Self {
        LinkedList { head: None }
    }

    // adds a node at the front
    fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,                  //stores the value
            link: self.head.take(), //takes ownership of current head & make it next head
        });
        self.head = Some(new_node); //make the new node head
    }

    // removes and return from front of the list
    fn pop_front(&mut self) -> Option<T> {
        //take ownership of head node
        self.head.take().map(|node| {
            self.head = node.link; //update head to point next node
            node.value //return current value
        })
    }

    //checks if empty
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

    //print the content of list
    //only works if the type T implements Display trait i.e. println! able
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
