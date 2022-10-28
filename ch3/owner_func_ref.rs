fn main() {
    let g1 = String::from("過ちを見過ごす人は美しい");
    show_message(&g1); // 参照を渡す
    println!("{}", g1);  // ok 所有権は移動していない
}

fn show_message(message: &String) {
    println!("{}", message);
}
