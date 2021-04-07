use std::time::{Duration};

// a function with lifetimes. I dont even know what a lifetime is yet.
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'a i32) -> i32 { *i + *j }

// a generic function.
// fn add<T>(i: T, j: T) -> T { i + j }
// this does not work because type T represents all types and not
// all types can work with operator '+'.

// For the generic type to be able to work with operator +, you
// have to explicitly define the operator on it or something using
// some traits or something I dont know about yet, so cheer!
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T { i + j }

fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);
    println!("{}", res);
    println!("{}", add(1,2));
    //println!("{}", add("Hello ", "World!"));
    // i thought this should work!
    println!("{:?}", add(Duration::new(5, 0), Duration::new(10, 0)));
}
