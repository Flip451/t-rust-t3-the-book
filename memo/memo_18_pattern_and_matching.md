# １８章：パターンとマッチング

## 目次

- [１８章：パターンとマッチング](#１８章パターンとマッチング)
  - [目次](#目次)
  - [18.0 概要](#180-概要)
  - [18.1 パターンが使用されることのある箇所全部](#181-パターンが使用されることのある箇所全部)
  - [18.2 論駁可能性: パターンが合致しないかどうか](#182-論駁可能性-パターンが合致しないかどうか)
  - [18.3 パターン記法](#183-パターン記法)
    - [`match` 中でリテラルをパターンとして使う](#match-中でリテラルをパターンとして使う)
    - [`match` 中で名前付き変数をパターンとして使う](#match-中で名前付き変数をパターンとして使う)
    - [or を表す `|`](#or-を表す-)
    - [範囲を表す `..=`](#範囲を表す-)
    - [分配して値を分解する](#分配して値を分解する)
      - [構造体](#構造体)
      - [`enum`](#enum)
      - [参照を分配する](#参照を分配する)
      - [構造体とタプルを分配する](#構造体とタプルを分配する)
    - [`_` をつけてパターンの値を無視する](#_-をつけてパターンの値を無視する)
      - [`_` で値全体を無視](#_-で値全体を無視)
      - [ネストされた `_` で値の一部を無視する](#ネストされた-_-で値の一部を無視する)
      - [`_` から始まる名前で未使用の変数を無視する](#_-から始まる名前で未使用の変数を無視する)
      - [`..` で残りの部分を無視する](#-で残りの部分を無視する)
    - [`ref`, `ref mut` でパターンに参照を生成する](#ref-ref-mut-でパターンに参照を生成する)
    - [マッチガードで追加の条件式](#マッチガードで追加の条件式)
    - [`@` 束縛](#-束縛)

## 18.0 概要

- パターンには二種類ある：
  - 論駁不可能：渡される可能性のあるあらゆる値に合致するパターン
  - 論駁可能：なんらかの可能性のある値に対して合致しないことがあるパターン

- パターンが使用されることのある箇所の一覧（〇では論駁可能なパターンを、×では論駁不可能なパターンを使う）：
   1. `match` アーム
      - 基本的に論駁可能なパターンを使うが、最後のアームでだけは論駁不可能なパターンを使ってよい
   2. 条件分岐 `if let` 式（〇）
   3. `while let` 条件ループ（〇）
   4. `for` ループ（×）
   5. `let` 文（×）
   6. 関数の引数（×）

## 18.1 パターンが使用されることのある箇所全部

1. `match` アーム

   - 2.に示す `if let` よりも網羅性がある

   ```rust
   match VALUE {
      // パターン => 式,
      PATTERN => EXPRESSION,
      PATTERN => EXPRESSION,
      PATTERN => EXPRESSION,
   }
   ```

2. 条件分岐 `if let` 式

   ```rust
   if let PATTERN = EXPRESSION {
       // ...
   }
   ```

   - `if let` には、パターンに合致しないときに走るコードを記す `else` を用意できる
     - `if let`, `else if`, `else if let` 式を混ぜて使える
   - 網羅性はないがその分柔軟に使える
   - 例：

     ```rust
     fn main() {
         let favorite_color: Option<&str> = None;
         let is_tuesday = false;
         let age: Result<u8, _> = "34".parse();
 
         if let Some(color) = favorite_color {
             println!("Using your favorite color, {color}, as the background");
         } else if is_tuesday {
             println!("Tuesday is green day!");
         } else if let Ok(age) = age {
             if age > 30 {
                 println!("Using purple as the background color");
             } else {
                 println!("Using orange as the background color");
             }
         } else {
             println!("Using blue as the background color");
         }
     }
     ```

3. `while let` 条件ループ

   - パターンが合致するならループを実行する

   ```rust
   while let PATTERN = EXPRESSION {
       // ...
   }
   ```

4. `for` ループ

   ```rust
   for PATTERN in EXPRESSION {
       // ...
   }
   ```

5. `let` 文

   ```rust
   let PATTERN = EXPRESSION;
   ```

6. 関数の引数
   - 以下の `x` の部分はパターン：

   ```rust
   fn hoge(x: i32) {
      // ...
   }
   ```

   - 例：

     ```rust
     fn print_coordinates(&(x, y): &(i32, i32)) {
         println!("Current location: ({}, {})", x, y);
     }
 
     fn main() {
         let point = (3, 5);
         print_coordinates(&point);
     }
     ```

## 18.2 論駁可能性: パターンが合致しないかどうか

- パターンには 2 つの形態がある: 論駁可能なものと論駁不可能なもの
  - 論駁不可能：渡される可能性のあるあらゆる値に合致するパターン
    - 例：`let x = 5;` の `x`
  - 論駁可能：なんらかの可能性のある値に対して合致しないことがあるパターン
    - 例：`if let Some(x) = a_value {...}` の `Some(x)`

- 論駁不可能なパターンのみを受け付ける：
  - 関数の引数、`let` 文、`for` ループ
  - 例：`let Some(x) = some_option_value;` は `Some(x)` が論駁可能なパターンなのでコンパイルエラーを起こす

- 論駁可能なパターンのみを受け付ける：
  - `if let`, `while let`
  - 例：以下のコードはコンパイルエラーを起こす（`x` は論駁不可能なので）：

    ```rust
    if let x = 5 {
        println!("{}", x);
    };
    ```

- `match` 式については、
  - 基本的に、論駁可能なパターンを使う
  - ただし、最後のアームでのみ論駁不可能なパターンを使用することが許される

## 18.3 パターン記法

すべての合法なパターン記法を示す

### `match` 中でリテラルをパターンとして使う

- 例：

  ```rust
  match x {
      1 => println!("one"),
      2 => println!("two"),
      3 => println!("three"),
      _ => println!("anything"),
  }
  ```

### `match` 中で名前付き変数をパターンとして使う

- `match` は新しいスコープを開始するので、`match` 式内のパターンの一部として宣言された変数は、`match` 構文外の同名変数を覆い隠すので注意
- たとえば、以下のコードでは
  - 最初に `Matched, y = 5`
  - 次に `at the end: x = Some(5), y = 10` と表示される

  ```rust
  let x = Some(5);
  let y = 10;  // ...(*1)

  match x {
      Some(50) => println!("Got 50"),
      Some(y) => println!("Matched, y = {y}"),  // ここで使用されている `y` は (*1) で定義されている `y` を覆い隠す
      _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);  //　ここで参照されている `y` は (*1) で定義されたもの
  ```

### or を表す `|`

- 例：

  ```rust
  match x {
      1 | 2 => println!("one or two"),
      3 => println!("three"),
      _ => println!("anything"),
  }
  ```

### 範囲を表す `..=`

- 数値か `char` 値のみ対応
- 例：数値

  ```rust
  match x {
      1..=5 => println!("one through five"),  // x が 1, 2, 3, 4, 5 のいずれかならマッチ
      _ => println!("something else"),
  }
  ```

- 例：`char` 値

  ```rust
  match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
  }
  ```

### 分配して値を分解する

#### 構造体

- 例：

  ```rust
  struct Point {
      x: i32,
      y: i32,
  }

  fn main() {
      let p = Point { x: 0, y: 7 };

      let Point { x: a, y: b } = p;
      assert_eq!(0, a);
      assert_eq!(7, b);
  }
  ```

- 省略記法を使った例：

  ```rust
  struct Point {
      x: i32,
      y: i32,
  }

  fn main() {
      let p = Point { x: 0, y: 7 };

      let Point { x, y } = p;
      assert_eq!(0, x);
      assert_eq!(7, y);
  }
  ```

- 構造体の一部をリテラルパターンに置き換えて、構造体の一部のみを名前付き変数に収める例：

  ```rust
  struct Point {
      x: i32,
      y: i32,
  }

  fn main() {
      let p = Point { x: 0, y: 7 };

      match p {
          Point { x, y: 0 } => println!("On the x axis at {}", x),
          Point { x: 0, y } => println!("On the y axis at {}", y),
          Point { x, y } => println!("On neither axis: ({}, {})", x, y),
      }
  }
  ```

#### `enum`

- 例：

  ```rust
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(i32, i32, i32),
  }

  fn main() {
      let msg = Message::ChangeColor(0, 160, 255);

      match msg {
          Message::Quit => {
              println!("The Quit variant has no data to destructure.")
          }
          Message::Move { x, y } => {
              println!(
                  "Move in the x direction {} and in the y direction {}",
                  x, y
              );
          }
          Message::Write(text) => println!("Text message: {}", text),
          Message::ChangeColor(r, g, b) => println!(
              "Change the color to red {}, green {}, and blue {}",
              r, g, b
          ),
      }
  }
  ```

#### 参照を分配する

- パターンの中で `&` を使用することで、参照の中の値を保持する変数を得られる：
- 特にイテレータがあるクロージャで役立つ
- 例：

  ```rust
  struct Point {
      x: i32,
      y: i32,
  }
  
  let points = vec![
      Point { x: 0, y: 0 },
      Point { x: 1, y: 5 },
      Point { x: 10, y: -3 },
  ];

  let sum_of_squares: i32 = points
      .iter()  // `iter` メソッドは `points` 内の各 `Point` への不変参照 `&Point` を返す
      .map(|&Point { x, y }| x *　x + y *　y)  // ここの `&` を外してしまうと、型不一致エラーが発生する
      .sum();  // 0^2 + 0^2 + 1^2 + 5^2 +10^2 + (-3)^2 = 135
  ```

#### 構造体とタプルを分配する

- 以下のような複雑なパターンも可能：

  ```rust
  struct Point {
    x: i32,
    y: i32,
  }
  
  let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
  ```

### `_` をつけてパターンの値を無視する

- **`_`** を使って**値全体を無視**したり、
- 他のパターンの内部で **`_` から始まる変数名**を使ってその値を無視することができる

#### `_` で値全体を無視

- `_` はどんな値にも一致するけれども、値を束縛しないワイルドカードパターン
- 例：

  ```rust
  fn foo(_: i32, y: i32) {
      println!("This code only uses the y parameter: {}", y);
  }

  fn main() {
      foo(3, 4);
  }
  ```

#### ネストされた `_` で値の一部を無視する

- 他のパターンの内部で `_` を使用して、値の一部だけを無視することもできる
- 例：以下のコードでは、ユーザは既存の設定を上書きできないけれども、設定を解除したり、現在設定がされていなければ設定に値を与えられる

  ```rust
  fn main() {
      let mut setting_value = Some(5);
      let new_setting_value = Some(10);

      match (setting_value, new_setting_value) {
          (Some(_), Some(_)) => {
              println!("Can't overwrite an existing customized value");
          }
          _ => {
              setting_value = new_setting_value;
          }
      }

      println!("setting is {:?}", setting_value);
  }
  ```

- 例：複数の箇所で `_` を使用して特定の値を無視する

  ```rust
  fn main() {
      let numbers = (2, 4, 8, 16, 32);

      match numbers {
          (first, _, third, _, fifth) => {
              println!("Some numbers: {first}, {third}, {fifth}")
          }
      }
  }
  ```

#### `_` から始まる名前で未使用の変数を無視する

- 通常コンパイラは、未使用の変数を見つけるとワーニングを出すが、`_` から始まる変数名のものに関しては見過ごされる
  - 例：以下のコードでは未使用の変数 `x`, `_y` が生成されているが、警告は `x` に関してのもののみ出力される：

    ```rust
    fn main() {
        let x = 5;
        let _y = 10;
    }
    ```

- `_` という変数と `_` から始まる変数とでは、値が束縛されうるかという点で異なることに注意
  - 例：以下の二つのコードのうち、前者はコンパイルエラーを起こすが後者はコンパイルが通る：

    ```rust
    fn main() {
        let s = Some(String::from("Hello!"));

        if let Some(_s) = s {  // `_s` に対しては所有権の移動が起こる
            println!("found a string");
        }

        println!("{:?}", s);  // `s` はムーブ済みなのでここではアクセスできない
    }
    ```

    ```rust
    fn main() {
        let s = Some(String::from("Hello!"));

        if let Some(_) = s {  // `_` に対しては値が束縛されないので、`s` のデータの所有権はムーブされない
            println!("found a string");
        }

        println!("{:?}", s);  // ここでも `s` にアクセスできる
    }
    ```

#### `..` で残りの部分を無視する

- `..` で多くの部分からなる値の一部だけを取り出すことができる
- 例：構造体での使用例

  ```rust
  fn main() {
      struct Point {
          x: i32,
          y: i32,
          z: i32,
      }

      let origin = Point { x: 0, y: 0, z: 0 };

      match origin {
          Point { x, .. } => println!("x is {}", x),  // Point 構造体の y, z のフィールドは無視する
      }
  }
  ```

- 例：タプルでの使用例

  ```rust
  fn main() {
      let numbers = (2, 4, 8, 16, 32);

      match numbers {
          (first, .., last) => {  // タプルの最初の要素と最後の要素のみを取り出す
              println!("Some numbers: {first}, {last}");
          }
      }
  }
  ```

### `ref`, `ref mut` でパターンに参照を生成する

- パターンの外部で、値を借用したかったら `&` を使えばいいが、
- パターンの内部では、参照にしたい変数の前に `ref` をつけることで値を借用する
- `ref` を使用することで、値の所有権をパターン中の変数にムーブさせる代わりに、参照を生成することができる
  - 例：例えば以下のコードは所有権の移動の問題でコンパイルできない

    ```rust
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(name) => {println!("Found a name", name)}.  // ここで `robot_name` の一部の所有権が `name` に移動する
        None => (),
    }

    println!("robot_name is: {:?}", robot_name); // ここでは robot_name にアクセスできないためコンパイルエラーを起こす
    ```

  - これを解消するには以下のように書き換えればよい：

    ```rust
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref name) => {println!("Found a name", name)}.  // `name` は参照なので所有権を奪わない
        None => (),
    }

    println!("robot_name is: {:?}", robot_name); // なので、ここでも robot_name にアクセスできる
    ```

- また、`ref mut` を用いればパターン中で可変参照を取ることができる
  - 例：

    ```rust
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"), // name を可変参照として取得して、*で参照外しして、その `name` の値を更新している
        None => (),
    }

    println!("robot_name is: {:?}", robot_name); // 値が更新されているので 「robot_name is: Some("Another name") 」と出力される
    ```

### マッチガードで追加の条件式

- **マッチガード**：`match` アーム中のパターンの直後に `if` 条件式を追記することで、そのアームが選択されるのに必要な条件を追加できる
  - 例：

    ```rust
    fn main() {
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }
    }
    ```

- 通常 `match` アームのパターン内で、`match` 式の外の変数を参照しようとしても、新しい変数が作成されるだけでうまくいかない
- しかし、マッチガードを使うことで、`match` 式の外部の変数とパターンで取得した変数の比較などが可能になる
  - 例：

    ```rust
    fn main() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            // Some(y) => println!("Matched, y = {y}"),
            // のように書くと、このアームだけで有効な局所的なスコープ内で新しい変数 `y` が定義されるだけなので注意
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);
    }
    ```

- マッチガードと or 演算子の `|` を組合わせるとき、マッチガードの条件式は `|` で並列されたすべてのパターンに適用されることに注意
  - たとえば、`4 | 5 | 6 if y` は `(4 | 5 | 6) if y` のような挙動をする
  - 例：以下のコードは `no` と出力する

    ```rust
    fn main() {
        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"),  // x の値が 4, 5, 6 のいずれかに等しく、かつ y が true の場合だけにアームがマッチする
            _ => println!("no"),
        }
    }
    ```

### `@` 束縛

- `at` 演算子 `@` により、**パターン中で変数を生成するのと同時に、その値が（`@` 後に記述した）パターンに一致するかを検証できる**
- 例：

  ```rust
  fn main() {
      enum Message {
          Hello { id: i32 },
      }

      let msg = Message::Hello { id: 5 };

      match msg {
          Message::Hello {
              id: id_variable @ 3..=7,  // `id` の値が 3 以上 7 以下であることを検証しつつ、その値を `id_variable` に収められる
          } => println!("Found an id in range: {}", id_variable),
          Message::Hello { id: 10..=12 } => {  // この書き方では、`id` が 10 以上 12 以下であることしか確かめられない（`id` という変数に値は保存されない）
              println!("Found an id in another range")
          }
          Message::Hello { id } => println!("Found some other id: {}", id),
      }
  }
  ```
