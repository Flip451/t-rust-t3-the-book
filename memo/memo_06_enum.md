# ６章
## 目次
- [６章](#６章)
  - [目次](#目次)
  - [6.0 概要](#60-概要)
    - [Enum の特徴](#enum-の特徴)
  - [6.1 Enum](#61-enum)
    - [Option](#option)
  - [6.2 match フロー制御演算子](#62-match-フロー制御演算子)
  - [6.3 if let で簡潔なフロー制御](#63-if-let-で簡潔なフロー制御)

## 6.0 概要
- enum
  - Option
  - Result
- match
- if let

### Enum の特徴
- 構造体と同様に独自の型として機能する
- その定義の中で取りうる値をすべて列挙する
- 列挙子のインスタンスは列挙された値の中のいずれか一つの値をとる

## 6.1 Enum
- 定義
  ```rust
  enum IpAddrKind{
    V4,
    V6,
  }
  ```

- インスタンス化
  ```rust
  // four と six はともに IpAddrKind 型の値
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;
  ```

- 各列挙子にデータを直接添付する
- 列挙子 `IpAddrKind` の取りうる値について、各々の列挙子ごとにその型を定めることができる
  ```rust
  enum IpAddrKind {
     V4(u8, u8, u8, u8),
     V6(String),
  }

  let home = IpAddr::V4(127, 0, 0, 1);

  let loopback = IpAddr::V6(String::from("::1"));
  ```

  - 参考：標準ライブラリにおける `IpAddrKind` の定義
    ```rust
    struct Ipv4Addr {
      // 省略
    }

    struct Ipv6Addr {
      // 省略
    }

    enum IpAddr {
      V4(Ipv4Addr),
      V6(Ipv6Addr),
    }
    ```

- メソッド定義
  ```rust
  enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  impl Message {
    fn call(&self) {
      // メソッド定義
    }
  }

  let m = Message::Write("hello");
  m.call();
  ```

### Option
- `Option<T>` は、値が存在するか不在かという概念をコード化する enum
- 標準ライブラリの中以下のように定義されている

```rust
enum Option<T> {
  Some(T),
  None,
}
```

- `Option<T>` は有益すぎて、初期化処理にさえ含まれている
  - つまり明示的にスコープに導入する必要がない
- また、`Some` と `None` を `Option::` の接頭辞なしに直接使える
- `None` をインスタンス化するときは、型注釈が必要なので注意（`None` 値を見ただけでは、`Some` 列挙子が保持する型をコンパイラが推論できないから）
  ```rust
  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;
  ```

- 以下のコードはコンパイルエラーを起こす（`Option<T>` と `T` は異なる型なので）
  ```rust
  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  let sum = x + y;    // i8 と Option<i8> は異なる型を持つのでそのままでは加算できない
  ```


## 6.2 match フロー制御演算子
- 値は match の各パターンを通り抜け、値が「適合する」最初のパターンで、コードブロックに落ち、実行中に使用される
- match は式
  ```rust
  #[derive(Debug)]
  enum UsState {
    Alabama,
    Alaska,
    // ... などなど
  }

  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
  }

  fn value_in_cents(coin: Coin) -> u32 {
    match coin {
      Coin::Penny => {
        println!("Lucky penny!");
        1
      },
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
      },
    }
  }
  ```

- `Option<T>` と組み合わせる
  ```rust
  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      None => None,
      Some(num) => Some(num + 1),
    }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  ```

- match は包括的である
  - 全てのあらゆる可能性を網羅し尽くさなければ、コードは有効にならない
  - たとえば、以下のコードはコンパイルエラーを起こす：
    ```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
      match x {
        Some(num) => Some(num + 1),
      }
    }
    ```

- プレースホルダー `_` はどんな値にもマッチする
  ```rust
  let some_u8_value = 0u8;

  match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
  }
  ```

## 6.3 if let で簡潔なフロー制御
- `if let` は値が一つのパターンにマッチした時にコードを走らせ、それ以外の場合は何もしない `match` への糖衣構文
  - たとえば、以下のコードは `if let` 記法で簡略化できる
    ```rust
    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    ```
  - 簡略化するとこんな感じ：
    ```rust
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("three");
    }
    ```

- `if let` ＋ `else` の構文は、値が一つのパターンにマッチした時にあるコードを走らせ、それ以外の場合は他のコードを実行する `match` への糖衣構文
  - たとえば、以下のコードは `if let` + `else` で簡略化できる：
    ```rust
    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("Not three"),
    }
    ```
  - 簡略化するとこんな感じ：
    ```rust
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("Not three");
    }
    ```
