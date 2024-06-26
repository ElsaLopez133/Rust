use crate::List::{Cons, Nil};
use std::ops::Deref;


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

//Defining our own smart pointer
//The MyBox type is a tuple struct with one element of type T. 
//The MyBox::new function takes one parameter of type T and returns a MyBox 
//instance that holds the value passed in.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
//Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type. 
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//The second trait important to the smart pointer pattern is Drop, which lets you customize 
//what happens when a value is about to go out of scope
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

}