use std::time::{Instant, Duration};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1,0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("{}", count);
    
    find_needle_haystack();
}

fn find_needle_haystack() {
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}, {}", item, result)
        }
    }
}
