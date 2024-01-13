fn main() {
    for (index, number) in collatz::iterate(100).unwrap().enumerate() {
        println!("{index}: {number}");
    }
}
