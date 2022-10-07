use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // ここの `move` は必要. なぜなら、spawned thread は v よりも長生きする可能性があるから.
    let handle = thread::spawn(move || {
        println!("Here's vector: {:?}", v);
    });

    // 例えば、ここで、`drop(v);` すると、確実に spawned thread は v よりも長生きする

    handle.join().unwrap();
}
