fn main() {
    let needle = 0o52;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    // &haystack -> references to elements in the array.
    // haystack.iter() -> references to elements in array / borrows
    // haystack.iter_mut() -> allows mutation of iterable elements.
    // haystack.into_iter() -> returns elements as values / takes ownership
    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}
