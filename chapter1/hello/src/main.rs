fn greet_world() {
    let hello = "Hello, world!";
    let southern_germany = "Grüß Gott!";
    let chinese = "ハロー・ワール";

    let greetings = [hello, southern_germany, chinese];
    
    for greet in greetings.iter() {
        println!("{}", &greet);
    }
}

fn main() {
    greet_world();
}
