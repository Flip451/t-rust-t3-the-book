fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);

    println!("string1: {}, string2: {}", string1, string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence: &str = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
        pp: "文法を理解するために作成した特に意味もないフィールド",
    };
    println!("{}", i.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
struct ImportantExcerpt<'a, T> {
    part: &'a str,
    pp: T,
}

// unconstrained type parameter
// impl<'a, T, S> ImportantExcerpt<'a, (T, T)> {
//     fn hoge() {
//         println!("This is hoge!");
//     }
// }

// 所有権を奪い、その値への参照を返す関数はコンパイルエラーを起こすはず
// --> &'static を求められた
// 現時点でこれが何かわからない
// fn func1(x: String, y: String) -> &String {
//     &x
// }

// これも許されないらしい
// ダングリング参照を作らないのでよさそうに思えたが。。。。
// よくよく考えると、x を受け取ってもらえるとも限らないし当然か。。。
// fn func2(x: String, y: String) -> (&String, String) {
//     (&x, x)
// }

// fn func3(x: & str, y:&str) -> &str {
//     x
// }
