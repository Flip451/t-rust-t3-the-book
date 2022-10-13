# １７章：Rust のオブジェクト指向プログラミング機能

## 目次

- [１７章：Rust のオブジェクト指向プログラミング機能](#１７章rust-のオブジェクト指向プログラミング機能)
  - [目次](#目次)
  - [17.0 概要](#170-概要)
  - [17.1 オブジェクト指向プログラミングとは](#171-オブジェクト指向プログラミングとは)
  - [17.2 トレイトオブジェクトで異なる型の値を許容する](#172-トレイトオブジェクトで異なる型の値を許容する)
    - [トレイトオブジェクトは、ダイナミックディスパッチを行う](#トレイトオブジェクトはダイナミックディスパッチを行う)
    - [トレイトオブジェクトには、オブジェクト安全性が必要](#トレイトオブジェクトにはオブジェクト安全性が必要)

## 17.0 概要

- aaa

## 17.1 オブジェクト指向プログラミングとは

- オブジェクトからなる
  - これはデータとメソッドを持つ
    - メソッド：データを処理するプロシージャのこと
  - Rust では `struct` と `enum` と `impl` で提供される
- カプセル化：オブジェクトの実装詳細は、そのオブジェクトを使用するコードにはアクセスできない
  - Rust では `pub` キーワードでコントロールされる
- 継承
  - Rust では代わりに `trait` を用いる
- 多相性（polymorphism）：複数のオブジェクトが特定の特徴を共有しているなら、実行時にお互いに代用できること
  - Rust でジェネリクスとトレイト境界を使用

## 17.2 トレイトオブジェクトで異なる型の値を許容する

- トレイトオブジェクト：指定したトレイトを実装する型のインスタンスと、実行時にその型のトレイトメソッドを検索するためのテーブルの両方を指す
- トレイトオブジェクトを生成するには、
  1. `&` 参照や `Box<T>` のような何らかのポインタを指定して
  2. `dyn` キーワードで関連するトレイトを指定する
  - トレイトオブジェクトがポインタを使用しなければならない理由は１９章で...
- 例： `Box<dyn Draw>`：`dyn Draw` は「`Draw` を実装する任意の型」という意味だと思っておけばよい

  ```rust
  pub trait Draw {
      fn draw(&self);
  }

  pub struct Screen {
      pub components: Vec<Box<dyn Draw>>,  // Draw トレイトを実装する任意の型のインスタンスのベクターを表す
  }

  impl Screen {
      pub fn run(&self) {
          for component in self.components.iter() {
              component.draw();  // component は Box<dyn Draw> であるので `draw` メソッドを有する
          }
      }
  }

  // Draw トレイトを実装した Button 構造体の実装例
  pub struct Button {
      pub width: u32,
      pub height: u32,
      pub label: String,
  }

  impl Draw for Button {
      fn draw(&self) {
          // code to actually draw a button
      }
  }
  ```

  - これは以下のようにトレイト境界を用いても書き換え可能だが、動作が異なる：
    - 以下の定義だと、`components` には、全コンポーネントの型が一致しているベクトルしか許容されない：

    ```rust
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    ```
  
  - このライブラリ `gui` のユーザが、幅、高さ、オプションのフィールドを持つセレクトボックス構造体を実装することに決めたら、以下のようにセレクトボックス型にも `Draw` トレイトを実装する：

    ```rust
    use gui::Draw;

    // SelectBox の定義
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    // SelectBox に Draw トレイトを実装する
    impl Draw for SelectBox {
        fn draw(&self) {
            // code to actually draw a select box
        }
    }

    // gui ライブラリから `Button` と `Screen` を導入
    use gui::{Button, Screen};

    fn main() {
        // `SelectBox` と `Button` はいずれも `Draw` トレイトを実装しているので、`Box<T>` で包めば `Screen` の `components` ベクタに含むことができる
        // このとき当然 `Screen` に定義された `run` メソッドは正常に動作する
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {  // トレイトオブジェクトにするために `Box::new` を使用することに注意
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {  // トレイトオブジェクトにするために `Box::new` を使用することに注意
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };

        screen.run();  // 各コンポーネントの `draw` メソッドが呼び出される
    }
    ```

### トレイトオブジェクトは、ダイナミックディスパッチを行う

- トレイトオブジェクトを使うと、ダイナミックディスパッチを行うことになるので注意！

  - **ダイナミックディスパッチ**：コンパイル時にコンパイラがどのメソッドを呼び出しているのか**わからない**
    - コンパイラは、どのメソッドを呼び出すか実行時に弾き出す（そのためのコードを生成する）

  - **スタティックディスパッチ**：コンパイル時にコンパイラがどのメソッドを呼び出しているか**わかる**
    - 例：ジェネリクスに対してトレイト境界を使用する際には、単相化（[原文](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics)を参照せよ）によってスタティックディスパッチが行われる

- 実行時にどの構造体に定義されたメソッドを呼ぶかを検索する必要があるので、実行時コストがある

### トレイトオブジェクトには、オブジェクト安全性が必要

- トレイトオブジェクトには、オブジェクト安全なトレイトしか作成できない
- トレイトは、トレイト内で定義されているメソッド全てに以下の特性があれば、オブジェクト安全になる：
  1. 戻り値の型が Self でない
  2. ジェネリックな型引数がない

- 例：標準ライブラリの `Clone` トレイトはメソッドがオブジェクト安全でない

  ```rust
  pub trait Clone {
      fn clone(&self) -> Self;
  }
  ```

- 例：以下のようなトレイトオブジェクトは許されない：

  ```rust
  pub struct Screen {
      pub components: Vec<Box<dyn Clone>>,
  }
  ```

  ```txt
  error[E0038]: the trait `Clone` cannot be made into an object
   --> src/main.rs:2:29
    |
  2 |     pub components: Vec<Box<dyn Clone>>,
    |                             ^^^^^^^^^ `Clone` cannot be made into an object
    |
    = note: the trait cannot be made into an object because it requires `Self: Sized`
    = note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

  For more information about this error, try `rustc --explain E0038`.
  error: could not compile `example_17_01` due to previous error
  ```
