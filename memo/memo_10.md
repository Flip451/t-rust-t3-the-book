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
    - [トレイトを定義する](#トレイトを定義する)
    - [型にトレイトを実装する](#型にトレイトを実装する)
      - [トレイト実装で注意すべき制限(コヒーレンス, 孤児のルール)](#トレイト実装で注意すべき制限コヒーレンス-孤児のルール)
    - [デフォルト実装](#デフォルト実装)
    - [引数としてのトレイト(impl Trait 構文)](#引数としてのトレイトimpl-trait-構文)
    - [トレイト境界構文](#トレイト境界構文)
    - [複数のトレイト境界を + 構文で指定する](#複数のトレイト境界を--構文で指定する)
    - [where 句を使ったより明確なトレイト境界](#where-句を使ったより明確なトレイト境界)
    - [トレイトを実装している型を返す](#トレイトを実装している型を返す)
    - [トレイト境界を使用して、メソッド実装を条件分けする](#トレイト境界を使用してメソッド実装を条件分けする)
    - [ブランケット実装](#ブランケット実装)

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
- なので以下のようにまとめることができる(`<...>` の部分については、10.3 節で取り扱う)
  ```rust
  fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
      let mut largest = list[0];
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
      largest
  }

  // 以下のような実装もアリ：
  // fn largest<T: PartialOrd>(list: &[T]) -> &T {
  //     let mut largest = &list[0];
  //     for item in list.iter() {
  //         if *item > *largest {
  //             largest = item;
  //         }
  //     }
  //     largest
  // }

  fn main() {
      let number_list = vec![34, 50, 25, 100, 65];
      let result = largest(&number_list);
      println!("The largest number is {}", result);

      let char_list = vec!['y', 'm', 'a', 'q'];
      let result = largest(&char_list);
      println!("The largest char is {}", result);
  }
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
- トレイトを用いると**複数の型に共通の振る舞いを定義**できる
  - ここで、「複数の型が共通の振る舞いを持つ」とは、「それらの型全てに対して"同じ"メソッド群を呼び出せる」ことを指す
- トレイト境界を使用すると、あるジェネリックが、特定の振る舞いをもつあらゆる型になり得ることを指定できる

### トレイトを定義する
- 任意の型が、そのトレイトを保持するために必要なメソッドのシグネチャを定義する
  ```rust
  // 以下の定義をすると、コンパイラにより、Summary トレイトを保持するあらゆる型に、このシグニチャと全く同じメソッド summarize が定義されていることが強制される
  pub trait Summary {
    fn summarize(&self) -> String; // 波括弧内に実装を提供する代わりに、セミコロンを使用していることに注意
  }
  ```

### 型にトレイトを実装する
- 構文：トレイト `Hoge` を型 `Fuga` に適用する
  ```rust
  impl Hoge for Fuga {
    // トレイト Hoge を保持するために必要なメソッドの具体的な実装
  }
  ```
- 例：型 `NewsArticle` と型 `Tweet` にトレイト `Summary` を適用する<br/>
  **`lib.rs`**
  ```rust
  pub trait Summary {
    fn summarize(&self) -> String;
  }

  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }

  impl Summary for NewsArticle {
    fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
  }

  pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }

  impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
    }
  }
  ```

- 適用したトレイトのメソッドを呼び出す
  ```rust
  use chapter10::{self, Summary, Tweet};
  
  fn main() {
    let tweet = Tweet {
      username: String::from("horse_ebooks"),
      content: String::from(
        "of course, as you probably already know, people",
      ),
      reply: false,
      retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize()); // 1 new tweet: horse_ebooks: of course, as you probably already know, people
  }
  ```

#### トレイト実装で注意すべき制限(コヒーレンス, 孤児のルール)
- トレイトか対象の型が自分のクレートに固有 (local) である時のみ、型に対してトレイトを実装できる
  - 以下は可能：
    - 自クレート内で定義した型に自クレートで実装したトレイトを適用する
    - 自クレート内で定義した型に外部クレートで実装されたトレイトを適用する
    - 外部クレート内で定義された型に自クレートで実装したトレイトを適用する
  - 一方、外部のトレイトを外部の型に対して実装することはできない
    - 例：自クレート内で `Vec<T>` に対して `Display` トレイトを実装することはできない

### デフォルト実装
- トレイトの定義内で指定する必要なメソッドにデフォルトの実装を用意しておいて、
- 型にトレイトを適用する際には、そのメソッドを上書きするか、デフォルトの実装をそのまま使用するか選択する余地を生み出すことができる
  ```rust
  pub trait Summary {
    fn summarize(&self) -> String {
      String::from("(Read more...)")
    }
  }
  ```

- ある型にトレイトを適用する際に、あるメソッドのデフォルト実装を採用するには、トレイトを適用する構文の中でそのメソッドの具体的な実装の記述を省けばよい：<br/>
  **`src/lib.rs`**
  ```rust
  pub trait Summary {
    fn summarize(&self) -> String {
      String::from("(Read more...)")
    }
  }
  
  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }
  
  impl Summary for NewsArticle {}
  ```
- トレイト内のデフォルト実装を採用している型に対しても、メソッドを独自実装をしている型と同じようにメソッドを呼び出せる：<br/>
  **NewArticle を使用するファイル内**
  ```rust
  use chapter10::{self, NewsArticle, Summary};
  
  fn main() {
    let article = NewsArticle {
      headline: String::from("Penguins win the Stanley Cup Championship!"),
      location: String::from("Pittsburgh, PA, USA"),
      author: String::from("Iceburgh"),
      content: String::from(
        "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
      ),
    };

    println!("New article available! {}", article.summarize()); // New article available! (Read more...)
  }
  ```

- デフォルト実装は、自トレイト内の他のメソッド（デフォルト実装がされていないものでもよい）を呼び出すことができる：
  - （ただしデフォルト実装を、そのメソッドをオーバーライドしている実装から呼び出すことはできないので注意）
  ```rust
  pub trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
      format!("(Read more from {}...)", self.summarize_author())
    }
  }
  ```

### 引数としてのトレイト(impl Trait 構文)
- 関数 `func1` の引数 `param1` の型を「あるトレイト `Hoge` を実装した型の参照」としたいとき：
  ```rust
  // この関数の引数 param1 は、指定されたトレイト Hoge を実装しているあらゆる型を受け付ける
  // 逆に Hoge を実装していない型を持つ型を渡そうとすると、コンパイルエラーを起こす
  fn func1(param1: &impl Hoge) {
    // ... (ここでは トレイト Hoge によって実装が要請されている param1 のあらゆるメソッドを呼び出せる)
  }
  ```
- これは実は、トレイト境界構文の糖衣構文

### トレイト境界構文
- 関数に渡すジェネリック型引数（`fn func1<T> (...) {...}` でいうと `T` ）の各々にあるトレイトが実装されていることを強制する：
- 構文:
  - `T1, ..., Tn`: 関数 `func1` の定義内で有効なジェネリック型 
  - `Trait1, ..., Traitn`: `T1, ..., Tn` の各々に実装されていることを要請したいトレイト（トレイトの実装を要請しないなら、`Tk: Traitk,` の代わりに `Tk,` と書けばよい）
  - `U1, ..., Um, V`: 具体的な型と、`T1, ..., TN` で構築される
  ```rust
  fn func1<T1: Trait1, T2: Trait2, ..., Tn: Traitn> (param_1: U1, param_2: U2, ... param_m: Um) -> V {
    // 
  }
  ```
- 例：
  ```rust
  pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
  }
  ```

- 例：`Summary` を実装する 2 つのパラメータを持つような関数（`item1` と `item2` の型が（どちらも `Summary` を実装する限り）異なっても良い場合）
  ```rust
  pub fn notify(item1: &impl Summary, item2: &impl Summary) { ... }
  ```
- 例：`Summary` を実装する 2 つのパラメータを持つような関数（`item1` と `item2` の型が同じ型であることを要請したい場合）
  ```rust
  pub fn notify<T: Summary>(item1: &T, item2: &T) { ... }
  ```

### 複数のトレイト境界を + 構文で指定する
- 複数のトレイトの実装を要請する
  ```rust
  pub fn notify(item: &(impl Summary + Display)) { ... }
  ```
  ```rust
  pub fn notify<T: Summary + Display>(item: &T) { ... }
  ```

### where 句を使ったより明確なトレイト境界
- 関数のシグネチャの可読性を保つために、`where` 句を用いることができる
- たとえば、
  ```rust
  fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { ... }
  ```
  の代わりに以下のように書くこともできる：
  ```rust
  fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
  { ... }
  ```

### トレイトを実装している型を返す
- ここまでは、関数の引数にトレイトの実装を要請してきたが、
- 関数の返り値の型にトレイトの実装を要請することもできる：
  ```rust
  // 戻り値の型として impl Summary を使うことにより、具体的な型が何かを言うことなく、
  // returns_summarizable 関数は Summary トレイトを実装している何らかの型を返すのだ、と指定する
  fn returns_summarizable() -> impl Summary { ... }
  ```

- ただし、`impl Trait` は一種類の型を返す場合にのみ使える
  - 返り値の型を `impl Hoge` と指定しても、ある時は `Hoge` トレイトを実装する `A` 型を返し、ある時は `Hoge` トレイトを実装する `B` 型を返す関数を実装できるわけではない（そのような方法は１７章で扱う）
  - たとえば、以下のコードはコンパイルエラーを起こす：
    ```rust
    pub trait Summary {
      fn summarize(&self) -> String;
    }
   
    pub struct NewsArticle {
      pub headline: String,
      pub location: String,
      pub author: String,
      pub content: String,
    }
   
    impl Summary for NewsArticle {
      fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
      }
    }
   
    pub struct Tweet {
      pub username: String,
      pub content: String,
      pub reply: bool,
      pub retweet: bool,
    }
   
    impl Summary for Tweet {
      fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
      }
    }

    // NewsArticle か Tweet を返すというのは、コンパイラの impl Trait 構文の実装まわりの制約により許されていない
    fn returns_summarizable(switch: bool) -> impl Summary {
      if switch {
        NewsArticle {
          headline: String::from(
            "Penguins win the Stanley Cup Championship!",
          ),
          location: String::from("Pittsburgh, PA, USA"),
          author: String::from("Iceburgh"),
          content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
          ),
        }
      } else {
        Tweet {
          username: String::from("horse_ebooks"),
          content: String::from(
            "of course, as you probably already know, people",
          ),
          reply: false,
          retweet: false,
        }
      }
    }
    ```

### トレイト境界を使用して、メソッド実装を条件分けする
- 以下の例では、`new` メソッドは、`Pair<T>` の `T` の型が何であれ定義される
- 一方で、`cmp_display` メソッドは、`Pair<T>` の `T` が `Display` 型と `PartialOrd` 型をともに実装しているときのみ実装される
  ```rust
  use std::fmt::Display;
  
  struct Pair<T> {
    x: T,
    y: T,
  }

  impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
      Self { x, y }
    }
  }

  impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
      if self.x >= self.y {
        println!("The largest member is x = {}", self.x);
      } else {
        println!("The largest member is y = {}", self.y);
      }
    }
  }
  ```

### ブランケット実装
- あるトレイト `Hoge` を実装するすべての型にトレイト `Fuga` を実装する：
  ```rust
  impl<T: Hoge> Fuga for T {
    // Fuga を保持するために必要なメソッド群の定義
  }
  ```
    - 例：標準ライブラリには、Display トレイトを実装するあらゆる型に ToString トレイトを実装している。このブランケット実装があるので、Display トレイトを実装する任意の型に対して、ToString トレイトで定義された to_string メソッドを呼び出せる
      ```rust
      impl<T: Display> ToString for T {
        // --snip--
      }
      ```