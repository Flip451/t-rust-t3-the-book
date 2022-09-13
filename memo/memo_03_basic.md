# ３章
## 目次
- [３章](#３章)
  - [目次](#目次)
  - [3.1 変数と可変性](#31-変数と可変性)
    - [定義](#定義)
      - [変数](#変数)
      - [定数](#定数)
    - [シャドーイング](#シャドーイング)
  - [3.2 データ型](#32-データ型)
    - [スカラー型](#スカラー型)
      - [整数型](#整数型)
      - [浮動小数点型](#浮動小数点型)
      - [bool](#bool)
      - [char](#char)
    - [複合型](#複合型)
      - [タプル](#タプル)
      - [配列型](#配列型)
  - [3.3 関数](#33-関数)
  - [コメント](#コメント)
  - [フロー制御](#フロー制御)
    - [if 式](#if-式)
    - [loop](#loop)
    - [while](#while)
    - [for](#for)

## 3.1 変数と可変性
### 定義
#### 変数
- 変数は標準で不変
  ```rust
  let imutable_var = 3;
  let mut mutable_var = 5;
  ```
#### 定数
- 定数はどんなスコープでも定義できる
- 型注釈が必要
- 定数式にしかセットできない
  - 関数の呼び出し結果や実行時に評価される値にはセットできない
- すべて大文字、アンダースコアで単語を区切る
  ```rust
  const MAX_POINT = 100_000;
  ```

### シャドーイング
- 定義済みの変数を `let` で定義しなおすと元の値を覆い隠すことができる
  ```rust
  fn main() {
      let x = 5;
      let x = x + 1;
      let x = x * 2;
      println!("x is {}", x); // x is 12
  }
  ```

- シャドーイングで変数の型を変えることもできる
  ```rust
  let spaces = "   ";
  let spaces = spaces.len();
  ```

- シャドーイングはできても immutable な変数の値を変えることはできないことに注意
  - たとえば以下のコードはコンパイルエラーを吐く
    ```rust
    let spaces = "   ";
    spaces = spaces.len();
    ```

## 3.2 データ型
- Rust は静的型付き言語
  - コンパイル時にすべての変数の型が分かっている必要がある
- 型推論で複数の型が推測されるときには型注釈を省略できない

### スカラー型
#### 整数型
- |  大きさ  |  符号付き  | 符号なし |
  | ---- | ---- | --- |
  |  8 bit  |  i8  | u8  |
  |  16 bit  |  i16  | u16 |
  |  32 bit  |  i32  | u32 |
  |  64 bit  |  i64  | u64 |
  |  arch  |  isize  | usize |
  - 基準型は `u32`, `i32`

- 大きさを n bit とすると表現範囲は
  - in: [-2^(n-1), 2^(n-1) -1]
  - un: [0, 2^n - 1]

- isize, usize はプログラムが動作しているコンピュータの種類で大きさが変わる
  - 64(32) ビットアーキテクチャなら64(32) bit

- 整数リテラルは以下のどの形式でも表せる
  - | 数値リテラル | 例 |
    | --- | --- |
    | 10進数 | 98_222 |
    | 16進数 | 0xff |
    | 8進数 | 0o77 |
    | 2進数 | 0b1111_0000 |
    | バイト | b'A' |
- バイトリテラル以外であれば以下を使える
  - 型接尾辞（例：`57u8`）
  - 見た目の区切り記号 `_` （例：`1_000`）

#### 浮動小数点型
- `f32`(単精度) or `f64`(倍精度)
  - 基準型は `f64`

#### bool
- `true` or `false`
  ```rust
  let t: bool = true;
  ```

#### char
- シングルクオートで囲った一文字をリテラルとする
  ```rust
  fn main() {
    let c = 'z';
    let z = 'ℤ';
    let zzz = '💤';
  }
  ```

### 複合型
- タプル or 配列

#### タプル
- 定義
  ```rust
  let t: (i32, f64, u8) = (500, 6.4, 1);
  ```

- パターンマッチングによる分解
  ```rust
  fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("y is {}", y);
  }
  ```

- 各要素へのアクセス
  ```rust
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;
  ```

#### 配列型
- 固定長
- スタック領域にメモリ領域が確保される
  ```rust
  let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
  let first = months[0];
  let second = months[1];
  ```
- 配列の範囲外の要素にアクセスしようとするとコンパイルは通るが、実行するとパニックを起こす

## 3.3 関数
- `fn` で宣言
- 命名規則は、スネークケース
- コンパイラは、関数がどこで定義されているかは気にしない（どこで定義されていても定義されてさえいれば呼び出せる）
- 各仮引数(parameter)の型は宣言しなければならない
  ```rust
  fn main() {
    println!("Hello, world!");
    another_function(5, 6); // 5, 6 は実引数(argument)
  }

  fn another_function(x: i32, y: i32) { // x, y は仮引数(parameter)
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
  }
  ```

- 関数本体は、文が並び、最後に式を置くか文を置くという形で形成される
  - 文：なんらかの動作をして値を返さない命令
    - 式の末尾に`;`をつけると文になる
  - 式：値を返す命令, 文の一部になりえる
    - 式は終端に`;`を含まない
    - `6`, `String::from("Hello.")`, `{...}` はいずれも式
    - たとえば以下は`4`を返す式
      ```rust
       {
         let x = 3;
         x + 1
       }
       ```

- 戻り値を持つ関数
  - 最後の式を返す
  ```rust
  fin plus_one() -> i32 {
    println!("plus one.");
    x + 1
  }
  ```

## コメント
- `//` で始まり、改行で終わる

## フロー制御
### if 式
- Rust では if は式（if式）
- 条件式は、bool 型でなければならない
  ```rust
  let condition = true;
  let number = if condition {
    5
  } else {
    6
  };
  println!("number is {}", number); // number is 5
  ```
- if の各アームの結果になる可能性がある値は、同じ型でなければならない
  - たとえば以下のコードはコンパイルエラーを吐く
    ```rust
    fn main() {
      let condition = true;
      let number = if condition {
        5
      } else {
        "six"
      };

      println!("The value of number is: {}", number);
    }
    ```

### loop
- `loop {...}` はブロック内のコードを無限に繰り返す
- ループ内に `break` キーワードを配置することで、プログラムに実行を終了すべきタイミングを教えることができる
```rust
fn main() {
  loop {
    println!("again!");
}
```
### while
```rust
let number = 10;
while number != 0 {
  println!("{}!", number);
  number = number - 1;
}
println!("LIFTOFF!!!");
```

### for
- for ループを使ってコレクションの各アイテムに対してコードを実行できる
  ```rust
  fn main() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
      println!("the value is: {}", element);
    }
  }
  ```
- Range 型 を使って一定の回数、同じコードを実行する
  ```rust
  fn main() {
    for number in (1..4).rev() {
      println!("{}!", number);
    }
    
    println!("LIFTOFF!!!");
  }
  ```
