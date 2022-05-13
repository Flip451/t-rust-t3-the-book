/* 実装中の調査内容についてのメモ
 * 先頭文字が母音かどうかを判定する方法をコーディングするうえで参考にした資料：
 * - [Rustで「文字が特定の文字集合に含まれるか」を判定するのはどれが速いか](https://keens.github.io/blog/2019/10/06/rustde_mojigatokuteinomojishuugounifukumareruka_wohanteisurunohadoregahayaika/)
 *     - 母音 "aiueo" を先にバイト列としてソートした形（[b'a', b'e', b'i', b'o', b'o']）で保持しておいて、binary_search で探索
 *     - binary_search については、[APIドキュメント](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.binary_search) を参照した
 * 
 * String を byte 配列に変換する方法については、
 *  [How do I convert a string into a vector of bytes in rust?](https://stackoverflow.com/questions/23850486/how-do-i-convert-a-string-into-a-vector-of-bytes-in-rust)
 * を参照した
 * 結論から言えば、`as_byte` メソッドを使えばよい
 * 
 */

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
