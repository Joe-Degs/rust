fn array() {
    let one = [1, 2, 3]; // array literal infers type.
    let two: [u8; 3] = [1, 2, 3]; // array init with type def i.e ['type', length]
    let blank1 = [0; 3]; // repeated expression, type infered i.e (0) is repeated (3) times.
    let blank2: [u8; 3] = [0; 3]; // type def, repeated expression.

    // Just like in golang, the size of an array matters the type system.
    // Meaning two arrays that accept a single data type but have different sizes
    // are different.

    let arrays = [one, two, blank1, blank2];

    for a in &arrays {  // reference to array returns a slice.
        print!("{:?}: ", a);
        for n in a.iter() { //
            print!("\t{:?} + 10 = {} ", n, n+1);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t(Σ{:?}) = {}", a, sum);
    }
}

// a slice is of the type [T]. This means the size of a slice at runtime is not
// known therefore we can say its sort of dynamic array. If you take the reference
// of an array [T; n], you get a slice. A reference of a slice is also a slice.
// Bollocks!
//
//
// Vectors Vec<T> are just like golang slices. They are dynamically sized and are
// growable at runtime.

fn main() {
    array()
}
