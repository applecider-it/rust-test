use std::io::{self, Write};

fn main() {
    println!("💻 Rust CLI ツールへようこそ！");
    print!("あなたのお名前は？: ");
    io::stdout().flush().unwrap(); // 画面に文字を即座に表示

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("こんにちは、{}さん！WSLでRustのCLIが動いています！", name.trim());
}