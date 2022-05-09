fn main() {
    let s: String = String::from("hello");
    let slice: &str = &s[1..4];     // ell
    let slice: &str = &slice[1..];  // ll
    let slice: &str = &slice[..1];  // l
    
    println!("slice: {}", slice);
    
    
    let s: String = String::from("hello");
    let s: &String = &s;
    let s: &&String = &s;
    let s: &&&String = &s;
    let slice: &str = &s[1..4];     // ell

    println!("slice: {}", slice);

    let a:[i32; 10] = [1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..9];     // [2, 3, 4, 5, 1, 2, 3, 4]
    let slice: &[i32] = &slice[..7];  // [2, 3, 4, 5, 1, 2, 3]
    let slice: &[i32] = &slice[3..];  // [5, 1, 2, 3]
    println!("slice: {:?}", slice);
}
