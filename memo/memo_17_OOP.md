# １７章：Rust のオブジェクト指向プログラミング機能

## 目次

- [１７章：Rust のオブジェクト指向プログラミング機能](#１７章rust-のオブジェクト指向プログラミング機能)
  - [目次](#目次)
  - [17.0 概要](#170-概要)
  - [17.1 オブジェクト指向プログラミングとは](#171-オブジェクト指向プログラミングとは)
  - [17.2 トレイトオブジェクトで異なる型の値を許容する](#172-トレイトオブジェクトで異なる型の値を許容する)
    - [トレイトオブジェクトは、ダイナミックディスパッチを行う](#トレイトオブジェクトはダイナミックディスパッチを行う)
    - [トレイトオブジェクトには、オブジェクト安全性が必要](#トレイトオブジェクトにはオブジェクト安全性が必要)
  - [17.3 オブジェクト指向デザインパターンを実装する（実装例：ステートデザインパターン）](#173-オブジェクト指向デザインパターンを実装する実装例ステートデザインパターン)
    - [要件](#要件)
    - [`Post` の実装](#post-の実装)
    - [`add_text` メソッドの実装](#add_text-メソッドの実装)
    - [草稿の記事の内容は空であることを保証する](#草稿の記事の内容は空であることを保証する)
    - [記事の査読を要求すると、状態が変化する](#記事の査読を要求すると状態が変化する)
    - [`content` の振る舞いを変化させる `approve` メソッドを追加する](#content-の振る舞いを変化させる-approve-メソッドを追加する)

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

## 17.3 オブジェクト指向デザインパターンを実装する（実装例：ステートデザインパターン）

### 要件

- ここでは、ブログ記事に関する例を取り上げることでステートデザインパターンの実例を示す

- ブログの最終的な機能は以下のような感じ：

> 1. ブログ記事は、空の草稿から始まる。
> 2. 草稿ができたら、査読が要求される。
> 3. 記事が承認されたら、公開される。
> 4. 公開されたブログ記事だけが表示する内容を返すので、未承認の記事は、誤って公開されない。
>
> - それ以外の記事に対する変更は、効果を持つべきではない
>   - 例えば、査読を要求する前にブログ記事の草稿を承認しようとしたら、記事は、非公開の草稿のままになるべき

- コードにすると以下のような感じ（現時点では `blog` クレートは未実装なのでコンパイル不能）：

  ```rust
  use blog::Post;

  fn main() {
      let mut post = Post::new();

      post.add_text("I ate a salad for lunch today");
      assert_eq!("", post.content());

      post.request_review();
      assert_eq!("", post.content());

      post.approve();
      assert_eq!("I ate a salad for lunch today", post.content());
  }
  ```

- `Post` 型は内部でステートを持つ：
  - 「草稿」、「査読待ち」、「公開中」のいずれかの状態を持つ
  - 状態間の遷移は `Post` 型内部で管理される

### `Post` の実装

- `Post` 構造体と `new` メソッドを定義する

  ```rust
  pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
  }

  impl Post {
    pub fn new() -> Post {
      Post {
        state: Some(Box::new(Draft {})),
        content : String::new(),
      }
    }
  }

  trait State {}

  struct Draft {}

  impl State for Draft {}
  ```

### `add_text` メソッドの実装

  ```rust
  // --snip--

  impl Post {
    // --snip--

    pub fn add_text(&mut self, text: &str) {  // このメソッドは記事のステートによらずに定義される（実装も `state` フィールドと関わりなし）
      self.content.push_str(text);
    }
  }

  // --snip--
  ```

### 草稿の記事の内容は空であることを保証する

- `add_text` を呼び出して記事に内容を追加した後でさえ、記事はまだ草稿状態なので、それでも `content` メソッドには空の文字列スライスを返してほしい
- --> 一旦、常に空の文字列スライスを返すように実装する

  ```rust
  // --snip--

  impl Post {
    // --snip--

    pub fn content(&self) -> &str {
      ""
    }
  }

  // --snip--
  ```

### 記事の査読を要求すると、状態が変化する

- `request_review` メソッドを呼び出すと `state` が `Draft` から `PendingReview` に変わるようにする
- 状態が何であれ、同じメソッド `request_review` で状態ごとに定義された期待通りの動作をするように実装

  ```rust
  pub struct Post {
      state: Option<Box<dyn State>>,
      content: String,
  }

  impl Post {
      // --snip--

      pub fn request_review(&mut self) {
          // `Option<T>` の `take` メソッドについては https://doc.rust-lang.org/std/option/enum.Option.html#method.take を参照せよ
          // `take` で `Post` 構造体の `state` に `None` を残し `state` にあったデータを `s` にムーブする
          // 一旦 `self.state` に `None` を渡すことで、状態遷移後に `Post` が古い状態を使用してしまう可能性を排除できる
          // `s` は `Box<dyn State>`
          if let Some(s) = self.state.take() {
              self.state = Some(s.request_review());
          }
      }
  }

  trait State {
      // 状態を `PendingReview` に更新するメソッド
      // 現在の `State` を消費して新しい `State` を返す
      // `State` はトレイトなので、ここでは `Box<Self>` および `Box<dyn State>` を用いる
      // 受け取った引数を消費したいので、引数は `&self` ではなく `self: Box<Self>`
      fn request_review(self: Box<Self>) -> Box<dyn State>;
  }

  struct Draft {}

  impl State for Draft {
      // 状態が `Draft` なら何もせず `Box<PendingReview>` を返す
      fn request_review(self: Box<Self>) -> Box<dyn State> {
          Box::new(PendingReview {})
      }
  }

  struct PendingReview {}

  impl State for PendingReview {
      // 状態が `PendingReview` ならそのまま `self` を返す（変更なし）
      fn request_review(self: Box<Self>) -> Box<dyn State> {
          self
      }
  }
  ```

### `content` の振る舞いを変化させる `approve` メソッドを追加する

- `approve` も `request_review` と同様に状態を変化させる

  ```rust
  pub struct Post {
      state: Option<Box<dyn State>>,
      content: String,
  }

  impl Post {
      // --snip--

      pub fn approve(&mut self) {
          if let Some(s) = self.state.take() {
              self.state = Some(s.approve());
          }
      }
  }

  trait State {
      // --snip--

      fn approve(self: Box<Self>) -> Box<dyn State>;
  }

  // --snip--

  struct PendingReview {}

  impl State for PendingReview {
      // --snip--

      fn approve(self: Box<Self>) -> Box<dyn State> {
          Box::new(Published {})
      }
  }

  struct Published {}

  impl State for Published {
      fn request_review(self: Box<Self>) -> Box<dyn State> {
          self
      }

      fn approve(self: Box<Self>) -> Box<dyn State> {
          self
      }
  }
  ```

- `Published` 状態では、`content` としてまともな内容を返してほしい（他の状態では空の文字列 `""` を返せばよい）

  ```rust
  pub struct Post {
      state: Option<Box<dyn State>>,
      content: String,
  }

  impl Post {
      // --snip--

      pub fn content(&self) -> &str {
          // state の参照を `as_ref` で `Option<&Box<dyn State>>` に変換する
          //   as_ref については https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref を参照すること
          // これを `unwrap` で `&Box<dyn State>` にする（`state` が `None` ではありえないことが他のメソッドの定義からわかるので `unwrap` して問題ない）
          // `&Box<dyn State>` に `content` メソッドを呼び出すと、参照外し型強制が働くので `State` トレイトに実装された `content` メソッドが呼び出される
          // `as_ref` メソッドにより所有権関連のエラーが解消されているらしい。（ここは理解が浅いので要確認）
          // --> 実際 `as_ref` を除くとエラーが発生することは確認ずみ
          // > 「as_ref メソッドを呼び出します。値の所有権ではなく、Option 内部の値への参照が欲しいから」とのこと。現時点ではよくわからん。
          self.state.as_ref().unwrap().content(&self)
      }

      // --snip--
  }

  trait State {
      // State を参照して、`post.content` を返すか、空の文字列 "" を返すかどうか決める
      // デフォルト実装を追加しておくことで Draft と PendingReview 構造体での content の実装を省略する
      // ライフタイム注釈に注意：`post` のライフタイムを返り値に共有する
      fn content<'a>(&self, _post: &'a Post) -> &'a str {
          ""
      }

      --snip--
  }

  // --snip--
  struct Published {}

  impl State for Published {
      fn content<'a>(&self, post: &'a Post) -> &'a str {
          &post.content
      }

      // --snip--
  }
  ```
