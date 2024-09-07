use std::ops::Deref;

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // hello(&(*m)[..]);
               //
    let c = CustomSmartPointer { data: String::from("my stuff") };

    let d = CustomSmartPointer { data: String::from("other stuff") };

    println!("CustomSmartPointers created");

    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
