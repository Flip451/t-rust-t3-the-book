use std::collections::HashMap;

fn main() {
    // 整数のリストが与えられ、ベクタを使って mean(平均値)、median(ソートされた時に真ん中に来る値)、mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう) を返してください。
    let list = vec![
        1, 3, 8, 11, 1023, -5, 9, 7, 11, 10, 1, 2, 3, 4, 5, 6, 7, 0, 46, 9, 8, 7, 3, 2, 7
    ];
    let list = merge_sort(list);

    println!("{:?}", list);
    println!("mean: {}, median: {}, mode: {}", mean(&list), median(&list), mode(&list));
}

fn mean(list: &Vec<i32>) -> f64 {
    let len = list.len();

    let mut sum = 0;
    for &num in list.iter() {
        sum += num;
    }

    let mean = (sum as f64) / (len as f64);

    mean
}

fn median(list: &Vec<i32>) -> f64 {
    let mut median: f64 = 0.0;
    let len = list.len();
    if (len % 2 == 0) {
        median = ((list[len / 2 - 1] + list[len / 2]) as f64) / 2.0;
    } else {
        median = list[len / 2] as f64;
    }

    median
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, u8> = HashMap::new();

    for &num in list.iter() {
        let mut count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut mode: i32 = 0;
    let mut max_count = 0;
    for (num, count) in map {
        if count >= max_count {
            mode = num;
            max_count = count;
        }
    }
    mode
}

fn merge_sort(list: Vec<i32>) -> Vec<i32>{
    let len = list.len();
    match len {
        2 => {return {
            if list[0] <= list[1] {
                list
            } else {
                let mut list = list;
                list.into_iter().rev().collect()
            }
        }},
        1 => {return list},
        _ => (),
    }
    
    let (list_a, list_b) = slpit_vector(list);

    // merge_sort された二つの list を merge する
    merge(merge_sort(list_a), merge_sort(list_b))
}

fn merge(list_a: Vec<i32>, list_b: Vec<i32>) -> Vec<i32> {
    let mut list_out = Vec::new();
    // ふたつのベクタを反転しておく（pop, push で小さいほうの値にアクセスするため）
    let mut list_a: Vec<i32> = list_a.into_iter().rev().collect();
    let mut list_b: Vec<i32> = list_b.into_iter().rev().collect();

    loop {
        let a = match list_a.pop() {
            Some(num) => num,
            None => {
                list_out.append(&mut list_b.into_iter().rev().collect());
                break;
            }
        };

        let b = match list_b.pop() {
            Some(num) => num,
            None => {
                list_a.push(a);
                list_out.append(&mut list_a.into_iter().rev().collect());
                break;
            }
        };

        if a < b {
            list_out.push(a);
            list_b.push(b);
        } else {
            list_out.push(b);
            list_a.push(a);
        }
    }

    list_out
}

// ベクタを二つの部分（前半部と後半部）に分け、それぞれに所有権を与える
fn slpit_vector(v: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut v = v;
    let len = v.len() / 2;

    let mut v_latter: Vec<i32> = Vec::new();
    for _ in 0..len {
        let e = v.pop().unwrap();
        v_latter.push(e);
    }
    
    let v_first = v;

    (v_first, v_latter.into_iter().rev().collect())
}