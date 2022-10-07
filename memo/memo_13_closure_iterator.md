# 13章：関数型言語の機能: イテレータとクロージャ

## 目次

- [13章：関数型言語の機能: イテレータとクロージャ](#13章関数型言語の機能-イテレータとクロージャ)
  - [目次](#目次)
  - [13.0 概要](#130-概要)
  - [13.1 クロージャ: 環境をキャプチャできる匿名関数](#131-クロージャ-環境をキャプチャできる匿名関数)
    - [クロージャの定義と変数への保存](#クロージャの定義と変数への保存)
    - [クロージャの型推論と注釈](#クロージャの型推論と注釈)
    - [ジェネリック引数と `Fn` トレイトを使用してクロージャを保存する（メモ化 または 遅延評価）](#ジェネリック引数と-fn-トレイトを使用してクロージャを保存するメモ化-または-遅延評価)
    - [クロージャで環境をキャプチャする](#クロージャで環境をキャプチャする)
      - [`move` キーワードで所有権を奪う](#move-キーワードで所有権を奪う)
  - [13.2 イテレータ](#132-イテレータ)
    - [`Iterator` トレイトと `next` メソッド](#iterator-トレイトと-next-メソッド)
      - [イテレータの生成](#イテレータの生成)
      - [イテレータを消費するメソッド](#イテレータを消費するメソッド)
  - [13.3 他のイテレータを生成するメソッド](#133-他のイテレータを生成するメソッド)
    - [イテレータアダプタ](#イテレータアダプタ)
    - [`Iterator` トレイトで独自のイテレータを作成する](#iterator-トレイトで独自のイテレータを作成する)
  - [13.4 第 12 章の入出力プロジェクトを改善する](#134-第-12-章の入出力プロジェクトを改善する)
    - [`src/main.rs` の修正（返却されるイテレータを直接使う）](#srcmainrs-の修正返却されるイテレータを直接使う)
    - [`src/lib.rs` の修正](#srclibrs-の修正)
      - [`Config::new` の修正（添え字の代わりに `Iterator` トレイトのメソッドを使用する）](#confignew-の修正添え字の代わりに-iterator-トレイトのメソッドを使用する)
      - [`search` 関数の修正（イテレータアダプタでコードをより明確にする）](#search-関数の修正イテレータアダプタでコードをより明確にする)
  - [13.5 パフォーマンス比較: ループ VS イテレータ](#135-パフォーマンス比較-ループ-vs-イテレータ)

## 13.0 概要

- クロージャ、変数に保存できる関数に似た文法要素
  - 環境をキャプチャできる
    - クロージャは、3 つの方法で環境から値をキャプチャ可能（それぞれの方法に応じて型が存在する）：
      - 所有権を奪う --> `FnOnce`
      - 可変で借用する --> `FnMut`
      - 不変で借用する --> `Fn`
  - 定義時に引数の型や戻り値の型を注釈する必要はない（型推論可能）
- イテレータ、一連の要素を処理する方法
  - 基本的な使い方は、
    1. コレクションから以下のメソッドで生成：
        - `iter` メソッド: **不変参照**へのイテレータを生成
        - `into_iter`: 所有権を奪い、**所有された値を返す**イテレータを生成
        - `iter_mut`: **可変参照**へのイテレータを生成
    2. イテレータアダプタで別の種類のイテレータに変換
    3. 消費アダプタで消費（これをしないとイテレータは何もしない i.e. イテレータは "lazy" ）

## 13.1 クロージャ: 環境をキャプチャできる匿名関数

- 変数に保存したり、引数として他の関数に渡すことのできる匿名関数
- 関数と異なり、呼び出されたスコープの値をクロージャは、キャプチャすることができる

### クロージャの定義と変数への保存

- 定義と変数への保存

  ```rust
  let foo = |param1, param2, ... | {
    // 処理
  };

  c1(bar, foobar, ...)
  ```

  - 例：

    ```rust
    let expensive_closure = |num| {
      println!("calculating slowly...");
      thread::sleep(Duration::from_secs(2));
      num
    };
    ```

### クロージャの型推論と注釈

- クロージャでは、fn 関数のように**引数の型や戻り値の型を注釈する必要はない**
- --> 狭い文脈でのみ利用されるから型推論が可能だから

- 型注釈をつけるなら以下のようにする：

  ```rust
  let foo = |param1: 型名, param2: 型名, ... | -> 返り値の型 {
    // 処理
  };
  ```

- クロージャの型注釈と関数定義の比較：

  ```rust
  fn add_one_v1    (x: u32) -> u32 { x + 1 }
  let add_one_v2 = |x: u32| -> u32 { x + 1 };
  let add_one_v3 = |x|             { x + 1 };
  let add_one_v4 = |x|               x + 1 ;
  ```

- クロージャ定義には、引数それぞれと戻り値に対して推論される具体的な型が一つある
- --> 同一のクロージャに異なる型の引数を渡すとコンパイルエラーを吐く（以下のコードはコンパイルエラーを起こす）
  
  ```rust
  let example_closure = |x| x;
  
  let s = example_closure(String::from("hello"));
  let n = example_closure(5);
  ```

### ジェネリック引数と `Fn` トレイトを使用してクロージャを保存する（メモ化 または 遅延評価）

- **メモ化 (memoization)** または、**遅延評価 (lazy evaluation)**
- クロージャやクロージャの呼び出し結果の値を保持する構造体を作る
- この構造体を宣言する際にはクロージャの型を指定する必要があるが、この型には `Fn`, `FnMut`, `FnOnce` のいずれかを使う
  - すなわち、全てのクロージャは、以下のいずれかのトレイトを実装している: `Fn` 、`FnMut` または、`FnOnce`
- 例：

  ```rust
  struct Cacher<T>
      where T: Fn(u32) -> u32
  {
      calculation: T,
      value: Option<u32>,
  }

  impl<T> Cacher<T>
      where T: Fn(u32) -> u32
  {
      fn new(calculation: T) -> Cacher<T> {
          Cacher {
            calculation,
            value: None,
          }
      }

      fn value(&mut self, arg: u32) -> u32 {
          match self.value {
              Some(v) => v,
              None => {
                  let v = self.calculation(arg);
                  self.value = Some(v);
                  v
              },
          }
      }
  }
  ```
  
  - 利用例：

    ```rust
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);

    assert_eq!(v1, 1);
    ```

### クロージャで環境をキャプチャする

- クロージャは関数とは異なり、**環境をキャプチャし、自分が定義されたスコープの変数にアクセスできる**
  - 例：以下の例では `equal_to_x` が 引数でもない変数 `x` の値 `4` を読みだしている

    ```rust
    fn main() {
      let x = 4;
      let equal_to_x = |z| z == x;
      let y = 4;

      assert!(equal_to_x(y));
    }
    ```

- 同じことを関数では行うことができない：
  - 以下の例はコンパイルエラーエラーを吐く：

    ```rust
    fn main() {
      let x = 4;
      fn equal_to_x(z: i32) -> bool { z == x }
      let y = 4;

      assert!(equal_to_x(y));
    }
    ```

- クロージャは、3 つの方法で環境から値をキャプチャ可能：
  - 所有権を奪う --> `FnOnce`
  - 可変で借用する --> `FnMut`
  - 不変で借用する --> `Fn`

#### `move` キーワードで所有権を奪う

- 環境でクロージャが使用している値の所有権を奪うことをクロージャに強制したいなら、引数リストの前に move キーワードを使用できる
- 例：このコードはコンパイルできない

  ```rust
  fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    
    // こ こ で は 、 xを 使 用 で き ま せ ん: {:?}
    println!("can't use x here: {:?}", x);
    
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
  }
  ```

## 13.2 イテレータ

- 一旦イテレータを生成したら、いろんな手段で使用することができる
- 例：for 文での使用

  ```rust
  let v1 = verc![1, 2, 3];
  let v1_iter = v1.iter();

  for val in v1_iter {
    println!({}, val);
  }
  ```

### `Iterator` トレイトと `next` メソッド

- 全てのイテレータは、標準ライブラリで定義されている `Iterator` というトレイトを実装している
- このトレイトの定義は、以下の通り：

  ```rust
  pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // デフォルト実装のあるメソッドは省略
  }
  ```

- `Iterator` 型を実装するには `Item` 型も定義する必要がある
  - `Item` 型が `next` メソッドの返り値の型に使われる（`Item` 型がイテレータから返ってくる型）
  - `type Item`, `Self::Item`: このトレイトとの関連型を定義
    - 関連型 --> 19 章で説明

- `next` メソッド: 1 度に `Some` に包まれたイテレータの 1 要素を返し、繰り返しが終わったら、`None` を返す
  - 例：`next` メソッドの挙動

    ```rust
    #[test]
    fn iterator_demonstration() {
      let v1 = vec![1, 2, 3];

      let mut v1_iter = v1.iter();

      assert_eq!(v1_iter.next(), Some(&1));
      assert_eq!(v1_iter.next(), Some(&2));
      assert_eq!(v1_iter.next(), Some(&3));
      assert_eq!(v1_iter.next(), None);
    }
    ```
  
#### イテレータの生成

- `iter` メソッド: **不変参照**へのイテレータを生成
- `into_iter`: 所有権を奪い、**所有された値を返す**イテレータを生成
- `iter_mut`: **可変参照**へのイテレータを生成

#### イテレータを消費するメソッド

- イテレータの `next` メソッドを呼び出すと、イテレータの内部情報が変わるのでイテレータが"消費"される
- イテレータに実装されているメソッドのうち、`next` メソッドを呼び出すものを「**消費アダプタ**」と呼ぶ
  - 例：`sum` メソッド

    ```rust
    #[test]
    fn iterator_sum() {
      let v1 = vec![1, 2, 3];
      let v1_iter = v1.iter();
      let total: i32 = v1_iter.sum();
      
      assert_eq!(total, 6);
    }
    ```

## 13.3 他のイテレータを生成するメソッド

### イテレータアダプタ

- イテレータアダプタ: イテレータを別の種類のイテレータに変換
- イテレータアダプタは怠惰（lazy）で、消費されないと何もしない
  - 例；`map` （このコードは生成したイテレータを消費していないので警告が表示される）

    ```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
    ```

- `collect` メソッドを使用すればイテレータを消費し、結果の値をコレクションデータ型に集結させられる
  - 例：

    ```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
    assert!(v2, vec![2, 3, 4])
    ```

### `Iterator` トレイトで独自のイテレータを作成する

- 例：1 から 5 をカウントするだけのイテレータ

  ```rust
  struct Counter {
    count: u32;
  }

  impl Counter {
    fn new() -> Counter {
      Counter { count: 0  }
    }
  }

  impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
      self.count += 1;

      if self.count < 6 {
        Some(self.count)
      } else {
        None
      }
    }
  }

  #[test]
  fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
  }

  #[test]
  fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))  // (1, 2), (2, 3), (3, 4), (4, 5) 
                                                               // 入力イテレータのどちらかが None を返したら、zip は None を返却するため、理論的な 5 番目の組の (5, None) は生成されない
                                 .map(|(a, b)| a * b)          // 2, 6, 12, 20
                                 .filter(|x| x % 3 == 0)       // 6, 12
                                 .sum();                       // 6 + 12
    assert_eq!(18, sum);
  }
  ```

## 13.4 第 12 章の入出力プロジェクトを改善する

### `src/main.rs` の修正（返却されるイテレータを直接使う）

- `env::args` 関数はイテレータを返す
  - --> `env::args` から返ってくるイテレータの所有権を直接 `Config::new` に渡す

```diff
// --snip--

fn main() {
-   let args: Vec<String> = env::args().collect();

-   let config = Config::new(&args).unwrap_or_else(|err| {
+   let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

### `src/lib.rs` の修正

#### `Config::new` の修正（添え字の代わりに `Iterator` トレイトのメソッドを使用する）

- `env::args` 関数の返り値の型は `std::env::Args` であり、これはイテレータである
- `new` 関数では引数 `args: std::env::Args` の所有権を奪う
- イテレータの呼び出しによりイテレータに内部変化が起きるので、`args` 引数には `mut` キーワードを追記して、可変にしておく

```diff
// --snip--

- pub struct Config<'a> {
-   pub query: &'a String,
-   pub filename: &'a String,
+ pub struct Config {
+   pub query: String,
+   pub filename: String,
    pub case_sensitive: bool,
}

- impl<'a> Config<'a> {
+ impl Config {

-   pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
+   pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

-       if args.len() < 3 {
-           return Err("not enough arguments!");
-       }

+       args.next();
        
-       let query = &args[1];
+       let query = match args.next() {
+           Some(arg) => arg,
+           None => return Err("Didn't get a query string"),
+       };

-       let filename = &args[2];
+       let filename = match args.next() {
+           Some(arg) => arg,
+           None => return Err("Didn't get a file name"),
+       };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

// --snip--
```

#### `search` 関数の修正（イテレータアダプタでコードをより明確にする）

- 関数型プログラミングスタイルで
  - 可変な状態の量を最小化し
  - コードを明瞭化する

```diff
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
-   let mut result: Vec<&str> = Vec::new();
-
-   for line in contents.lines() {
-       if line.contains(query) {
-           result.push(line);
-       }
-   }
-   result
+   contents
+       .lines()
+       .filter(|line| line.contains(query))
+       .collect()
}
```

```diff
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
-   let mut result: Vec<&str> = Vec::new();
-   let query = query.to_lowercase();

-   for line in contents.lines() {
-       if line.to_lowercase().contains(&query) {
-           result.push(line);
-       }
-   }
-   result
+   let query = query.to_lowercase();
+   contents
+       .lines()
+       .filter(|line| line.to_lowercase().contains(&query))
+       .collect()
}
```

## 13.5 パフォーマンス比較: ループ VS イテレータ

- 要約すると「イテレータとクロージャを恐れなしに使用してよい！」
