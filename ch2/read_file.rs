use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // ファイル名の指定があるかどうか調べる
    if args.len() < 2 {
        println!("入力ファイルを指定してください。");
        return;
    }
    // （0から数えて）1番目の要素を得る
    let filename = &args[1];

    let text = fs::read_to_string(filename).unwrap();
    println!("{}", text);
}