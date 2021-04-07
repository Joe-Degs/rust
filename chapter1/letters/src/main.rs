fn main() {
    let mut letters = vec!["a", "b", "c"];

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone());
    }
    
    if check_eq(10, 20) {
        println!("true");
    } else {
        println!("false")
    }
}


fn check_eq(num1: i64, num2: i64) -> bool { num1 == num2 }
