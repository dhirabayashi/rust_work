fn main() {
    for i in 1..51 {
        let one_unit = i % 10;
        let ten_unit = i / 10;

        if i % 3 == 0 || one_unit == 3 || ten_unit == 3 {
            println!("A");
        } else {
            println!("{}", i);
        }
    }
}