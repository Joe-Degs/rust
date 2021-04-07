use std::rc::Rc;
use std::sync::{Arc, Mutex};

// different ways to create variables in rust
fn main() {
    let a = 10; // on the stack
    let b = Box::new(20); // on the heap{ boxing vars send them to the heap }
    let c = Rc::new(Box::new(30)); // on the heap{ wraped in a reference counter }
    let d = Arc::new(Mutex::new(40)); // " { in a mutual exclusion lock wraped in an atomic reference counter }

    println!("a: {:?} b: {:?} c: {:?} d: {:?}", a, b, c, d);
}

// rust gives you safety and control at the same time
// you are allowed to do lowlevel things without too much stress
// but also are allowed to follow all the strict rules in the compiler.
// this is neat, save for the fact that, all these rules are hard computer
// sciency things that the average programmer might struggle to understand.
// In short rust is the complicated hippy guy of programming languages.
