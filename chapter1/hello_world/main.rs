fn main() {
    println!("Hello world!");
    printer();

    let ewe = "some random string";
    let random = "unrandom string";

    let regions = [ewe, random];
    for region in regions.iter() {
        println!("{}", region);
    }
}

fn printer() {
    println!("{} {}", "Hello", "World!");
}
