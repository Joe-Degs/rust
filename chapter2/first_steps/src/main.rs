use std::convert::TryInto;

fn first_step() {
    let a = 20;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("(a + b) + (b + c) = {}", e);
}

fn intro_numbers() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("addition {}", addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32
    ];
    println!("{:02}", forty_twos[0])
}

fn intro_numbers_bases() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {}, {}, {}", three, thirty, three_hundred);
    println!("base 2: {:b}, {:b}, {:b}", three, thirty, three_hundred);
    println!("base 8: {:o}, {:o}, {:o}", three, thirty, three_hundred);
    println!("base 16: {:x}, {:x}, {:x}", three, thirty, three_hundred);
}

fn add(i: i32, j: i32) -> i32 { i+j }

fn comparisons() {
    let a: i32 = 10;
    let b: u16 = 100;
    
    if a < (b as i32) {
        println!("Ten is less than hundred.");
    }
}

fn comparisons1() {
    let a: i32 = 10;
    let b: u16 = 100;
    if a < b.try_into().unwrap() {
        println!("Ten is less than hundred again.");
    }
}

fn main() {
    first_step();
    intro_numbers();
    intro_numbers_bases();
    comparisons();
    comparisons1();
}
