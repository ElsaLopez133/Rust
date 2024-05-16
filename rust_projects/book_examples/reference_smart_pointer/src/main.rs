//The Cons variants own the data they hold, so when we create the b list, 
//a is moved into b and b owns a. Then, when we try to use a again when 
//creating c, we’re not allowed to because a has been moved.
//we’ll change our definition of List to use Rc<T> in place of Box<T>
//Every time we call Rc::clone, the reference count to the data within 
//the Rc<List> will increase, and the data won’t be cleaned up unless 
//there are zero references to it.

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}