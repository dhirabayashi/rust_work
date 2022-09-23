fn main() {
    for y in 1926..2027 {
        print!("西暦{}年 = ", y);

        if 1926 <= y && y <= 1988 {
            if y == 1926 {
                println!("昭和元年");
            } else {
                println!("昭和{}年", y - 1925);
            }
        } else if 1989 <= y && y <= 2018 {
            if y == 1989 {
                println!("平成元年");
            } else {
                println!("平成{}年", y - 1988);
            }
        } else if 2019 <= y {
            if y == 2019 {
                println!("令和元年");
            } else {
                println!("令和{}年", y - 2018);
            }
        }
    }
}