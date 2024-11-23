use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    let b = Rc::new(RefCell::new(Node { next: Some(a.clone()) }));

    // Creating a cycle
    if let Some(ref mut next) = a.borrow_mut().next {
        *next = b.clone();
    }

    println!("a: {:?}", a);
    println!("b: {:?}", b);

    // Uncommenting the following lines will cause a memory leak
    println!("a strong count: {}", Rc::strong_count(&a));
    println!("b strong count: {}", Rc::strong_count(&b));
}