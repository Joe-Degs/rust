#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    //drop(grains);

    println!("{:?}", grains);
    make_string()
}

fn make_string() {
    let s: String = String::from("Hello thee world!");
    let len = calc_len(&s);
    println!("The length of '{}' is {}.", &s, len)
}

fn calc_len(s: &String) -> usize {
    s.len()
}
