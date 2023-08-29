use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));  // `Arc::new` で複数の所有者の存在を許す（`Rc::new` でも似たことができるがマルチスレッドに対応していない）
    let mut handles = vec![];

    for i in 0..10 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);  // thread に `move` する前に複製
        let handle = thread::spawn(move || {  // ここで `move` キーワードをつけているので `counter` の所有権はクロージャに移動する
            if i % 2 == 0 {
                thread::sleep(Duration::from_millis(1));  // ためしに２の倍数番目のスレッドを遅延させてみる（その間、奇数番目のスレッドが先行して実行される）
            }
            let mut num: MutexGuard<i32> = counter.lock().unwrap();  // 複製した `Arc<Mutex<i32>>`（`couter`）に対し `unlock` メソッドを呼び出す
            *num += i;
            println!("loop {}: num is {}", i, num);
        });  // ここ（クロージャの終り）で複製された `Arc<Mutex<i32>>`（`couter`）はドロップされ、Mutex はアンロックされる
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.lock().unwrap());
}
