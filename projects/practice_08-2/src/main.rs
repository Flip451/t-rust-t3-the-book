use std::io;

fn main() {
    println!("Please input your guess.");
    let mut word = String::new();

    io::stdin()
        .read_line(&mut word) 
        .expect("Failed to read line");

    let word: String = match word.trim().parse() {
        Ok(word) => word,
        Err(_) => {
            println!("Please enter a word!");
            return
        }
    };

    let first = word.as_bytes()[0];

    let vowels = [b'a', b'e', b'i', b'o', b'o'];
    
    let result:String = match vowels.binary_search(&first) {
        Ok(_) => {
            // String::from(&word[1..]).push_str(&word[0..1])
            format!("{}-hay", &word[..])
        },
        Err(_) => {
            format!("{}-{}ay", &word[1..], &word[0..1])
        },
    };
    println!("{}", result);
}
