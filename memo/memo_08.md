# ８章
## 目次
- [８章](#８章)
  - [目次](#目次)
  - [8.0 概要](#80-概要)
  - [8.1 ベクタ `Vec<T>`](#81-ベクタ-vect)
    - [初期化](#初期化)
    - [更新](#更新)
    - [所有権との関係](#所有権との関係)
    - [ベクタの値を走査する](#ベクタの値を走査する)
    - [Enum を使って複数の型を保持する](#enum-を使って複数の型を保持する)
  - [8.2 文字列で UTF-8 でエンコードされたテキストを保持する](#82-文字列で-utf-8-でエンコードされたテキストを保持する)
    - [初期化](#初期化-1)
    - [更新](#更新-1)
    - [文字列の一部にアクセスする方法...](#文字列の一部にアクセスする方法)
      - [Rust における文字列](#rust-における文字列)
      - [文字列をスライスする](#文字列をスライスする)
    - [文字列を走査するメソッド群](#文字列を走査するメソッド群)
  - [8.3 キーとそれに紐づいた値をハッシュマップに格納する](#83-キーとそれに紐づいた値をハッシュマップに格納する)
    - [定義](#定義)
    - [所有権との関係](#所有権との関係-1)
    - [ハッシュマップの値にアクセスする](#ハッシュマップの値にアクセスする)
    - [ハッシュマップを更新する](#ハッシュマップを更新する)
      - [値を上書きする(`insert`)](#値を上書きするinsert)
      - [キーに値がなかった時のみ値を挿入する(`entry` + `or_insert`)](#キーに値がなかった時のみ値を挿入するentry--or_insert)
      - [古い値に基づいて値を更新する(HashMap の使用例)](#古い値に基づいて値を更新するhashmap-の使用例)
    - [ハッシュ関数](#ハッシュ関数)

## 8.0 概要
- コレクションが指すデータはヒープに確保される
- データ量はコンパイル時にわかる必要はない
- 伸縮可能
- **ベクタ型**は、可変長の値を並べて保持
- **文字列**は、文字のコレクション
- **ハッシュマップ**は、値を特定のキーと紐付けさせてくれる. **マップ**の特定の実装

## 8.1 ベクタ `Vec<T>`
- ベクタには、同じ型の値しか保持できない
### 初期化
- 新しい空のベクタを作る
  ```rust
  let v: Vec<i32> = Vec::new();
  ```

- `vec!` マクロで初期化する
  ```rust
  let v = vec![1, 2, 3];
  ```

### 更新
- `push` メソッドを使用してベクタ型に値を追加する
  ```rust
  let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);
  ```

### 所有権との関係
- ベクタをドロップすれば、要素（所有者と結びついたヒープデータの全体）もドロップする
  ```rust
  {
    let v = vec![1, 2, 3, 4];
    // vで作業をする
  } // <- vはここでスコープを抜け解放される
  ```

- ベクタに保持された値を参照する
  - 添字記法によるアクセスと、`get` メソッドによりアクセスの２通りがありうる
    - `&` と `[]` を使用して参照を得るか
    - 番号を引数として `get` メソッドに渡して、`Option<&T>` を得る
  ```rust
  let v = vec![1, 2, 3, 4, 5];
  
  let third: &i32 = &v[2];
  let third: Option<&i32> = v.get(2);
  ```

- ベクタに含まれない範囲外の値を使用しようとしたときの挙動：
  - `&hoge[]` --> 実行時にパニックを起こす
  - `get` メソッド --> パニックすることなく `None` を返す
  ```rust
  let v = vec![1, 2, 3, 4, 5];

  let does_not_exist = &v[100];    // パニックを起こす
  let does_not_exist = v.get(100); // None を返す
  ```

- `&hoge[]` は `hoge` への不変参照なので、`&hoge[]` を使用したスコープ内で `hoge` の可変参照を取るようなメソッドを実行しようとするとコンパイルエラーを起こす
  - たとえば以下のコードでは、可変参照と不変参照が同一スコープに共存できないことからコンパイルエラーを起こす
  ```rust
  let mut v = vec![1, 2, 3, 4, 5];
  
  let first = &v[0];  // v の不変参照
  
  v.push(6);          // push(&mut self) メソッドは v の可変参照を引数に取る

  println!("The first element is: {}", first);
  ```

### ベクタの値を走査する
- ベクタの各要素に対する不変参照を得て、それらを出力する
  ```rust
  let v = vec![100, 32, 57];

  for i in &v {
    println!("{}", i);
  }
  ```

- 全要素に変更を加える目的で、可変なベクタの各要素への可変な参照を走査する
  ```rust
  let mut v = vec![100, 32, 57];
  
  for i in &mut v {
    *i += 50;    // 各要素に 50 を足す
  }
  ```

### Enum を使って複数の型を保持する
- ベクタに異なる型の要素を保持する必要が出たら、enum を定義して使用することができる
  ```rust
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
  ```

## 8.2 文字列で UTF-8 でエンコードされたテキストを保持する
- 文字列は UTF-8 エンコードされている
### 初期化
- 空の新規文字列を生成する
  ```rust
  let mut s = String::new();
  ```

- `Display` トレイトを実装する型の変数に対して、`to_string` メソッドを呼び出す
  ```rust
  let data = "initial contents";
  
  let s = data.to_string();
  
  // the method also works on a literal directly:
  let s = "initial contents".to_string();
  ```

- `String::from` 関数を使って、文字列リテラルから `String` を生成する
  ```rust
  let s = String::from("initial contents");
  ```

### 更新
- `push_str` メソッドで文字列スライスを追記する
  ```rust
  // この 3 行の後、s は foobar を含む
  let mut s = String::from("foo");
  let s2 = "bar";
  s.push_str(s2);  // 引数の型は &str なので所有権の移動は起こらない
  ```

- `push` メソッドで、1 文字(`char`)を引数として取り、`String` に追加
  ```rust
  let mut s = String::from("lo");
  s.push('l');
  ```

- `+` 演算子で連結
  - ２つの `String` を連結する
  - `+` 演算子は `fn add(self, s: &str) -> String {` のようなシグネチャを持つ
  ```rust
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2;    // ここで後者のオペランドには Stringへの参照 `&String` を用いていることに注意
                        // + 演算子のシグネチャ的にはこの &String 型は、仮引数の型と合致しないが
                        // コンパイラが &String 引数を &str に型強制してくれるためうまく動作する
                        // (つまり、 &s2 を &s2[..] に強制してくれる（参照外し型強制）)
                        // String::from("Hello, ") の所有権は s1 から s3 に移る
                        // と同時に、String::from("Hello, ") のヒープデータの直後に &s2 で借用してきた内容が付け足される
  ```

- `format!`マクロで連結
  - このマクロは引数の所有権を奪わない
  ```rust
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  
  let s = format!("{}-{}-{}", s1, s2, s3);
  ```

### 文字列の一部にアクセスする方法...
- 文字列に添え字アクセスしたい！！
  - **Rust の文字列は、添え字アクセスをサポートしていない**
  - 添え字記法で `String` の一部にアクセスしようとすると、エラーが発生する
  - たとえば以下のコードはコンパイルエラーを起こす
  ```rust
  let s1 = String::from("hello");
  let h = s1[0];
  ```

#### Rust における文字列
- `String` は `Vec<u8>` のラッパ
- その実態は、Unicode の UTF-8 バイト列をヒープデータとして持つ `Vec<u8>` にすぎない
  - （UTF-8 については、[Wikipedia](https://ja.wikipedia.org/wiki/UTF-8)、[怖くないユニコードの話](https://www.youtube.com/watch?v=uXk6eOCYz_4&t=3s) が詳しい）
- たとえば、以下のコードを考える
  ```rust
  let len = String::from("Hola").len(); // 4
  let len = String::from("Здравствуйте").len(); // 24
  ```
  - このコードの１行目で登場する文字列は、1 byte 文字のみからなるので `len` = `4` となり、直観と一致する
  - しかし、２行目で登場する文字列は、2 byte 文字から構成されている
  - そのため、文字列は 12 文字からなるにもかかわらず `len` = `24` となる
  - この `24` という数字は、`"Здравствуйте"` という文字列が、ヒープ領域上で消費するバイト数に他ならない
- このことから明らかなように、Stirng 型文字列 `hoge` の n 文字目にアクセスしたいときに `hoge[n]` としても、それは `hoge` に対応して存在するヒープデータの n byte 目を参照しようとするだけで、n 文字目を取得することにはならない
  - （そればかりか、ヒープデータの n byte 目はそれ単体で文字を表すかどうかすら怪しい）
  - たとえば、`String::from("Здравствуйте")` の１文字目 "З" はキリル文字であり、UTF-8 では 2 byte (0xD0 0xB7) で表現される。よって、 `String::from("Здравствуйте")` の 0 byte 目の値を取得するとそれは 0xD0 (= 208) であって、それ単体で文字としての意味を持たない

#### 文字列をスライスする
- 文字列スライスで文字列の一部を切り出すことができる
- しかし、文字としての区切りの悪い取得の仕方を行おうとするとパニックを起こすので注意が必要
  ```rust
  let hello = "Здравствуйте";

  let s = &hello[0..4]; // s ~~ "Зд" 

  let s = &hello[0..1]; // パニックを起こす
  ```

### 文字列を走査するメソッド群
- 個々の Unicode スカラー値に対して処理を行う必要があったら、最適な方法は `chars` メソッドを使用するといい
  ```rust
  for c in "नमस्ते".chars() {
      println!("{}", c);
  }
  ```
  ```sh
  न
  म
  स
  ्
  त
  े
  ```

- bytes メソッドは、各バイトをそのまま返す
  ```rust
  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
  ```
  ```sh
  224
  164
  168
  224
  164
  174
  :
  :
  164
  224
  165
  135
  ```

## 8.3 キーとそれに紐づいた値をハッシュマップに格納する
- 型 `HashMap<K, V>` は、`K` 型のキーと `V` 型の値の対応関係を保持
- 最初に標準ライブラリのコレクション部分から `HashMap` を `use` する必要がある
- ハッシュマップはデータをヒープに保持
- キーは全て同じ型でなけばならず、値も全て同じ型でなければならない

### 定義
- 空のハッシュマップを new で作り、要素を insert で追加
  ```rust
  use std::collections::HashMap;

  fn main() {
      let mut scores = HashMap::new();

      scores.insert(String::from("Blue"), 10);
      scores.insert(String::from("Yellow"), 50);

      println!("{:?}", scores);
  }
  ```
  ```sh
  {"Yellow": 50, "Blue": 10}
  ```

- タプルのベクタに対して `collect` メソッドを使用する
  - `collect` メソッドは iterable なものなら何でも受け取り、関連するコレクションに変換することができる([ref](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect))
  - 変換先は様々な型がありうるので、型注釈が必要となることが多い
  - 下の例では、`zip` メソッドを使ってタプルのベクタを作り上げ
  - それを `collect` メソッドでをハッシュマップに変換している
  - この例では `HashMap<_, _>` という型注釈が必要になる（この注釈で十分に型が特定できる）
    ```rust
    use std::collection::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
    ```
    ```sh
    {"Blue": 10, "Yellow": 50}
    ```

### 所有権との関係
- i32 のような Copy トレイトを実装する型について、値はハッシュマップにコピーされる
- 一方で、`String` のような所有権のある値なら、値はムーブされて、所有権はハッシュマップに移る
- もちろん、値への参照をハッシュマップに挿入しても、値はハッシュマップにムーブされない
  ```rust
  use std::collection::HashMap;

  let field_name = String::from("Favorite color");
  let field_value = String::from("Red");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);  // この時点で String::from で生成して、もともと field_name, field_value が持っていた
                                        // ヒープデータの所有権がに map に移動する
  ```
- 参照が指している値は、最低でもハッシュマップが有効な間は、有効でなければならない
- この問題はライフタイムで解決される

### ハッシュマップの値にアクセスする
- `get` メソッドに提供することで、ハッシュマップから値を取り出すことができる
- `HashMap<K, V>.get` は `Option<&V>` を返す
  - `get` メソッドの引数に渡したキーに対応する値がハッシュマップになかったら、`get` は `None` を返す
  ```rust
  use std::collection::HashMap;

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score:Option<&i32> = scores.get(&team_name);

  println!("{:?}", score);
  ```
  ```sh
  Some(10)
  ```

- for ループでハッシュマップのキーと値のペアを走査する
  - 出力順は決定的でないので注意
  ```rust
  use std::collections::HashMap;
  
  let mut scores = HashMap::new();
  
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }
  ```
  ```sh
  Blue: 10
  Yellow: 50
  ```

### ハッシュマップを更新する
#### 値を上書きする(`insert`)
- `insert` メソッドは値を上書きする
  ```rust
  use std::collections::HashMap;
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);

  println!("{:?}",scores);
  ```
  ```sh
  {"Blue": 25}
  ```
#### キーに値がなかった時のみ値を挿入する(`entry` + `or_insert`)
- `entry` メソッドを使って、特定のキーに値があるか確認できる
- `entry` メソッドは `Entry` という enum を返す
- `Entry` 型には、`or_insert` メソッドが定義されている
- `or_insert` メソッドは 
  - `Entry` 値が「指定したキーに値が存在することを表すもの」であれば、そのキーに対する値への可変参照を返し
  - `Entry` 値が「指定したキーに値が存在しないことを表すもの」であれば、引数をこのキーの新しい値として挿入し、新しい値への可変参照を返す
  ```rust
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);
  ```
  ```sh
  {"Blue": 10, "Yellow": 50}
  ```

#### 古い値に基づいて値を更新する(HashMap の使用例)
```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
  let count = map.entry(word).or_insert(0);
  *count += 1;
}

println!("{:?}", map);
```

### ハッシュ関数
- 使用するハッシュ関数は切り替え可能

