use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::thread_rng(); // gen_range メソッドの第一引数は &mut self なので mutable として定義する必要がある
    let secret_number: u32 = rng.gen_range(1..101);
    // println!("Secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin() //  stdin 関数は、std::io::Stdin オブジェクトを返し、この型は、ターミナルの標準入力へのハンドルを表す
            .read_line(&mut guess) // ユーザから入力を受け付け
            .expect("Failed to read line"); // read_line メソッドは io::Result 型を返す. io::Result` オブジェクトが `Err` 値の場合、`expect` メソッドはプログラムをクラッシュさせ、引数として渡されたメッセージを表示します.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
