fn main() {
    for x in 1..10 {
        for y in 1..10 {
            print!("{:3},", x * y)
        }
        println!();
    }
}