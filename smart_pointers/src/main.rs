enum List {
    Cons(i32, Box<List>),
    Nil,
}
use std::rc::Rc;
enum RcList {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn add(x: &i32) -> i32 {
    x * 2
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct SmartPointer {
    data: String,
}

impl Drop for SmartPointer {
    fn drop(&mut self) {
        println!("dropping like it is hot");
    }
}



fn main() {
    println!("Hello, world!");

    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))
            )
        ))
    );
    let x = 5;
    let y = MyBox::new(x);

    let sp = SmartPointer { data: String::from("lolz")};
    use List::{Cons, Nil};
    // let graph = Cons(5, Rc::new(
    //                 Cons(10, Rc::new(Nil))));

    // let pointer1 = Cons(4, Rc::clone(&graph));
    // let pointer2 = Cons(5, Rc::clone(&graph));

    println!("create");
    assert_eq!(5, x);
    add(&y);
}
