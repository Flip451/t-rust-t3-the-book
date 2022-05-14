# １０章：ジェネリック型、トレイト、ライフタイム
## 目次
- [１０章：ジェネリック型、トレイト、ライフタイム](#１０章ジェネリック型トレイトライフタイム)
  - [目次](#目次)
  - [10.0 概要](#100-概要)
  - [10.2 ジェネリックなデータ型](#102-ジェネリックなデータ型)
    - [関数定義](#関数定義)
      - [関数定義にジェネリック型を導入する例：](#関数定義にジェネリック型を導入する例)
    - [構造体定義](#構造体定義)
    - [enum 定義](#enum-定義)
    - [メソッド定義](#メソッド定義)
  - [10.3 トレイト: 共通の振る舞いを定義する](#103-トレイト-共通の振る舞いを定義する)

## 10.0 概要
- ジェネリック型
- トレイト
- ライフタイム

## 10.2 ジェネリックなデータ型

### 関数定義
- 構文：関数 `hoge` でジェネリック型をつかう
  - `T1, T2, ..., Tn`: 関数 `hoge` の定義中で使いたいジェネリック型の一覧
  - `U1, U2, ..., Um`: 関数 `hoge` の仮引数の型の一覧（各々の `U*` は定義済みの具体的な型と、`T1, T2, ..., Tn` で構築される）
  - `V`: 関数 `hoge` の返り値の型. （定義済みの具体的な型と、`T1, T2, ..., Tn` で構築される）
  ```rust
  fn hoge<T1, T2, ..., Tn> (parameter1: U1, parameter2: U2, ..., parameterm: Um) -> V {
    // parameter1, ..., parameterm を使った処理
    // 返り値の型は V である必要がある
  }
  ```

#### 関数定義にジェネリック型を導入する例：
- たとえば、`i32` の配列の最大値を求める関数 `largest_i32(list: &[i32]) -> i32` と `char` の配列の最大値を求める関数 `largest_char(list: &[char]) -> char ` を考える
  ```rust
  fn largest_i32(list: &[i32]) -> i32 {
      let mut largest = list[0];
      
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }

      largest
  }

  fn largest_char(list: &[char]) -> char {
      let mut largest = list[0];
      
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }

      largest
  }

  fn main() {
      let number_list = vec![34, 50, 25, 100, 65];
      
      let result = largest_i32(&number_list);
      println!("The largest number is {}", result);
      assert_eq!(result, 100);
      
      let char_list = vec!['y', 'm', 'a', 'q'];
      
      let result = largest_char(&char_list);
      println!("The largest char is {}", result);
      assert_eq!(result, 'y');
  }
  ```

- この時両者の関数は全く同じ実装をしている
- なので以下のようにまとめることができる
  ```rust
  // TODO:
  ```

### 構造体定義
- 例１：
  ```rust
  struct Point<T> {
    x: T,
    y: T,
  }

  fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
  }
  ```
- 例２：
  ```rust
  struct Point<T, U> {
    x: T,
    y: U,
  }
  
  fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
  }
  ```

### enum 定義
- 例：
  ```rust
  enum Result<T, E> {
    Ok(T),
    Err(E),
  }
  ```

### メソッド定義
- 構文：型 `Hoge<T1, T2, ..., Tn>` に対してメソッドを定義したいとき
  - `A1, A2, ..., Am`: `Hoge` に渡したいジェネリック型のシンボルの一覧
  - `B1, B2, .., Bn`: `Hoge`型にわたす型引数の一覧。`B*` は各々 `A1, A2, ..., Am` と具体的な型から構築される型。`A1, A2, ..., Am` のすべてが現れる必要がある（？要検証？）
  - m = 0 であれば、`impl` の直後の `<...>` は省略する
  ```rust
  impl<A1, A2, ..., Am> Hoge<B1, B2, .., Bn> {
    // ...（この領域では `A1, A2, ..., Am` が、さも定義済みの具体的な型かのように扱われる）
    // メソッド定義は関数定義と同じように行う
      // たとえば、以下の定義では
        // - T1, T2, ..., Tj は A1 ~ Am, B1 ~ Bn とは完全に独立
        // - T1, T2, ..., Tj は hoge を定義する文の中でだけ有効
        // - U1 ~ Uk, V の各々は、A1 ~ Am と、T1 ~ Tj、および、定義済みの具体的な型で構築される型
    fn hoge<T1, T2, ..., Tj> (parameter1: U1, parameter2: U2, ..., parameterm: Uk) -> V {
      // parameter1, ..., parameterm を使った処理
      // 返り値の型は V である必要がある
    }
  }
  ```
- 例１：ジェネリックな型を持つ Point<T> インスタンスにメソッドを実装する
  ```rust
  struct Point<T> {
    x: T,
    y: T,
  }

  impl<T> Point<T> {
    fn x(&self) -> &T {
      &self.x
    }
  }

  fn main() {
    let p = Point { x: 5, y: 10 };
    
    println!("p.x = {}", p.x());
  }
  ```
- 例２：`Point<f32`> だけにメソッドを実装する
  - `Point<f32>` には `distance_from_origin` というメソッドが存在するが、
  - `T` が `f32` ではない `Point<T>` の他のインスタンスにはこのメソッドが定義されない
  ```rust
  struct Point<T> {
    x: T,
  }

  impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
  }
  ```

- 例３：メソッド定義内で、ほかのジェネリック型引数を持つ型を使用する
  - 一部のジェネリックな引数は `impl<ここ>` で宣言され、
  - 他の一部はメソッド定義 `fn メソッド名<ここ>` で宣言される
  ```rust
  struct Point<T, U> {
    x: T,
    y: U,
  }

  impl<T, U> Point<T, U> {
    // self（Point型）の x 値（型 T）と、引数に渡した other（Point型）の y 値（型 W）から新しいインスタンス(型 Point<T, W>)を生成
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
      Point {
        x: self.x,
        y: other.y,
      }
    }
  }

  fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};
    
    let p3 = p1.mixup(p2);
    
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
  }
  ```

## 10.3 トレイト: 共通の振る舞いを定義する
- 