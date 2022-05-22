# 12章：入出力プロジェクト: コマンドラインプログラムを構築する


## 目次

- [12章：入出力プロジェクト: コマンドラインプログラムを構築する](#12章入出力プロジェクト-コマンドラインプログラムを構築する)
  - [目次](#目次)
  - [12.0 概要](#120-概要)
  - [12.1 コマンドライン引数を受け取る](#121-コマンドライン引数を受け取る)
    - [`std::env::args` で引数を受け取る](#stdenvargs-で引数を受け取る)
    - [引数の値を変数に保存](#引数の値を変数に保存)
  - [12.2 ファイルを読み込む](#122-ファイルを読み込む)
  - [12.3 リファクタリングしてモジュール性とエラー処理を向上させる](#123-リファクタリングしてモジュール性とエラー処理を向上させる)
    - [リファクタリングの概要](#リファクタリングの概要)
    - [1. & 2. バイナリプロジェクトの責任の分離](#1--2-バイナリプロジェクトの責任の分離)
      - [引数解析器を抽出する](#引数解析器を抽出する)
      - [Config のコンストラクタを作成する](#config-のコンストラクタを作成する)
    - [3. & 4. エラー処理を修正する](#3--4-エラー処理を修正する)
      - [渡す引数の数が少なければパニックを起こさせる](#渡す引数の数が少なければパニックを起こさせる)
      - [`panic!` を呼び出す代わりに `new` から `Result` を返す](#panic-を呼び出す代わりに-new-から-result-を返す)
      - [main からロジックを抽出する](#main-からロジックを抽出する)
      - [run 関数からエラーを返す](#run-関数からエラーを返す)
      - [main で run から返ってきたエラーを処理する](#main-で-run-から返ってきたエラーを処理する)
    - [コードをライブラリクレートに分割する](#コードをライブラリクレートに分割する)
  - [12.4 テスト駆動開発でライブラリの機能を開発する](#124-テスト駆動開発でライブラリの機能を開発する)
    - [失敗するテストを記述する](#失敗するテストを記述する)
    - [テストを通過させるコードを書く](#テストを通過させるコードを書く)
    - [run 関数内で search 関数を使用する](#run-関数内で-search-関数を使用する)
  - [12.5 環境変数を取り扱う](#125-環境変数を取り扱う)
    - [大文字小文字を区別しない search 関数用に失敗するテストを書く](#大文字小文字を区別しない-search-関数用に失敗するテストを書く)
    - [`search_case_insensitive` 関数を実装する](#search_case_insensitive-関数を実装する)
    - [run 関数から新しい search_case_insensitive 関数を呼び出す](#run-関数から新しい-search_case_insensitive-関数を呼び出す)
  - [12.6 標準出力ではなく標準エラーにエラーメッセージを書き込む](#126-標準出力ではなく標準エラーにエラーメッセージを書き込む)
    - [標準出力の中身を画面の代わりにファイルに書き込む(ファイルにリダイレクトする)](#標準出力の中身を画面の代わりにファイルに書き込むファイルにリダイレクトする)
    - [エラーを標準エラーに出力する](#エラーを標準エラーに出力する)

## 12.0 概要
- aaa


## 12.1 コマンドライン引数を受け取る
- create project
  ```sh
  $ cargo new minigrep
  ```
### `std::env::args` で引数を受け取る
- この関数はコマンドライン引数のイテレータを返す
  - イテレータは一連の値を生成する
  - `collect` 関数を用いて、イテレータが生成する要素全部をコレクションに矯正できる

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```
> - 引数のどれかが不正なユニコードを含んでいたら、`std::env::args` はパニックする
> - プログラムが不正なユニコードを含む引数を受け付ける必要があるなら、代わりに `std::env::args_os` を使用する
>   - この関数は、`String` 値ではなく、`OsString` 値を生成するイテレータを返す

```sh
$ cargo run
["target/debug/minigrep"]

$ cargo run needle haystack
["target/debug/minigrep", "needle", "haystack"]
```

### 引数の値を変数に保存
- 
```diff
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

+   let query = &args[1];
+   let filename = &args[2];

+   println!("Searching for {}", query);
+   println!("In file {}", filename);
}
```

## 12.2 ファイルを読み込む
- add `poem.txt`
  ```txt
  I'm nobody! Who are you?
  Are you nobody, too?
  Then there's a pair of us - don't tell!
  They'd banish us, you know.
  How dreary to be somebody!
  How public, like a frog
  To tell your name the livelong day
  To an admiring bog!
  ```

- `std::fs::File::open` でファイルのハンドルを取得
- ファイルのハンドルの `read_to_text` メソッドで読み取ったファイルの中身を `String` 変数に収める
  - （このメソッドは `std::io::prelude::*` 内で定義されているトレイトの中で定義されている）

```diff
use std::env;
+ use std::fs::File;
+ use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

+   let mut f = File::open(filename).expect("file not found");
+   let mut contents = String::new();

+   f.read_to_string(&mut contents)
+       .expect("something went wrong reading the file");

+   println!("With text:\n{}", contents);
}
```

```sh
$ cargo run fas poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep fas poem.txt`
Searching for fas
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

## 12.3 リファクタリングしてモジュール性とエラー処理を向上させる
### リファクタリングの概要
1. main 関数が複数の責任を受け持っているのでこれを分割するのが望ましい
- 機能を小分けして、各関数が 1 つの仕事のみに責任を持つようにするのが最善
   - ここでの複数の責任とは？以下の二つ：
      - 引数を解析し
      - ファイルを開いている
2. 設定用変数を一つの構造に押し込め、目的を明瞭化するのが最善
   - スコープにある変数が増えれば、各々の目的を追うのも大変になるという問題への対処
3. エラーハンドリングすべき
4. エラー処理のコードが全て 1 箇所に存在
し、将来エラー処理ロジックが変更になった時に、メンテナンス者が 1 箇所のコードのみを考慮すればいいようにするのが最善

### 1. & 2. バイナリプロジェクトの責任の分離
- main が肥大化し始めた際にバイナリプログラムの個別の責任を分割
するためのガイドライン
   - プログラムを `main.rs` と `lib.rs` に分け、**ロジックを `lib.rs` に移動**
     - 解析ロジックが小規模な限り、`main.rs` に置いても良い
     - 解析ロジックが複雑化の様相を呈し始めたら、`main.rs` から抽出して `lib.rs` に移動


- この工程の後に `main` 関数に残る責任は以下に限定される:
  - 引数の値でコマンドライン引数の解析ロジックを呼び出す
  - 他のあらゆる設定を行う
  - `lib.rs` の `run` 関数を呼び出す
  - `run` がエラーを返した時に処理する

#### 引数解析器を抽出する
- **`src/main.rs`**
  ```diff
  use std::env;
  use std::fs::File;
  use std::io::prelude::*;

  + use minigrep::*;

  fn main() {
      let args: Vec<String> = env::args().collect();

  -   let query = &args[1];
  -   let filename = &args[2];
  +   let config = parse_config(&args);

  -   println!("Searching for {}", query);
  -   println!("In file {}", filename);
  +   println!("Searching for {}", config.query);
  +   println!("In file {}", config.filename);

  -   let mut f = File::open(filename).expect("file not found");
  +   let mut f = File::open(config.filename).expect("file not found");
      let mut contents = String::new();

      f.read_to_string(&mut contents)
          .expect("something went wrong reading the file");

      println!("With text:\n{}", contents);
  }
  ```
  **`src/lib.rs`**
  ```rust
  pub struct Config<'a> {
      pub query: &'a String,
      pub filename: &'a String,
  }

  pub fn parse_config(args: &Vec<String>) -> Config {
      let query = &args[1];
      let filename = &args[2];

      Config { query, filename }
  }
  ```


#### Config のコンストラクタを作成する
- `parse_config` をただの関数から `Config` 構造体に紐づく `new` という関数に変える
- **`src/main.rs`**
  ```diff
  use std::env;
  use std::fs::File;
  use std::io::prelude::*;

  use minigrep::*;

  fn main() {
      let args: Vec<String> = env::args().collect();

  -   let config = parse_config(&args);
  +   let config = Config::new(&args);
      
      println!("Searching for {}", config.query);
      println!("In file {}", config.filename);

      let mut f = File::open(config.filename).expect("file not found");
      let mut contents = String::new();

      f.read_to_string(&mut contents)
          .expect("something went wrong reading the file");

      println!("With text:\n{}", contents);
  }
  ```
- **`src/lib.rs`**
  ```diff
  pub struct Config<'a> {
      pub query: &'a String,
      pub filename: &'a String,
  }

  - pub fn parse_config(args: &Vec<String>) -> Config {
  -     let query = &args[1];
  -     let filename = &args[2];
  -
  -    Config { query, filename }
  - }
  + impl<'a> Config<'a> {
  +     pub fn new(args: &Vec<String>) -> Config {
  +         let query = &args[1];
  +         let filename = &args[2];
  + 
  +         Config { query, filename }
  +     }
  + }
  ```

### 3. & 4. エラー処理を修正する
#### 渡す引数の数が少なければパニックを起こさせる
- `Config::new` 関数に、添え字 1 と 2 にアクセスする前にスライスが十分長いことを実証するチェックを追加
  **`src/lib.rs`**
  ```diff
  pub struct Config<'a> {
      pub query: &'a String,
      pub filename: &'a String,
  }

  impl<'a> Config<'a> {
      pub fn new(args: &Vec<String>) -> Config {
  +       if args.len() < 3 {
  +           panic!("not enough arguments!");
  +       }
  +
          let query = &args[1];
          let filename = &args[2];

          Config { query, filename }
      }
  }
  ```
- このコードの実行時のエラー出力は、ユーザーに伝えたくない内容も含んでしまう...
  ```sh
  $ cargo run fas
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep fas`
  thread 'main' panicked at 'not enough arguments!', src/lib.rs:9:13
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  ```

#### `panic!` を呼び出す代わりに `new` から `Result` を返す
- `new` メソッドの返り値の型を `Result` に変更する
- `Reslut::unwrap_or_else` メソッドで `Result` 値が `Err` であるときに、引数に渡したクロージャを実行する
- `std::process::exit` 関数でプロセスを終了する. 引数には終了コードを渡す. 
  - 0 以外の終了コードは、我々のプログラムを呼び出したプロセスにプログラムがエラー状態で終了したことを通知する慣習

**`src/main.rs`**
```diff
use std::env;
use std::fs::File;
use std::io::prelude::*;
+ use std::process;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();

-   let config = Config::new(&args);
+   let config = Config::new(&args).unwrap_or_else(|err| {
+       println!("Problem parsing arguments: {}", err);
+       process::exit(1);
+   });
+
+   # なお、これは以下と同等のコード
+   # let config = match Config::new(&args) {
+   #     Ok(c) => c,
+   #     Err(err) => {
+   #         println!("Problem parsing arguments: {}", err);
+   #         process::exit(1);
+   #     },
+   # };
    
    --snip--
}
```

**`src/lib.rs`**
```diff
pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

impl<'a> Config<'a> {
-   pub fn new(args: &Vec<String>) -> Config {
+   pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
-           panic!("not enough arguments!");
+           return Err("not enough arguments!");
        }

        let query = &args[1];
        let filename = &args[2];

-       Config { query, filename }
+       Ok(Config { query, filename })
    }
}
```
- 実行結果：この出力の方が遥かにユーザに優しい
```sh
$ cargo run fas
   Compiling minigrep v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/minigrep fas`
Problem parsing arguments: not enough arguments!
```

#### main からロジックを抽出する
**`src/main.rs`**
```diff
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

+   run(config);
-   // ここから下のコードはすべて run 関数の中に移動した
}

+ fn run(config: Config) {
+     let mut f = File::open(config.filename).expect("file not found");
+     let mut contents = String::new();
+
+     f.read_to_string(&mut contents)
+         .expect("something went wrong reading the file");
+
+     println!("With text:\n{}", contents);
+ }
```

#### run 関数からエラーを返す
- `run` 関数内で `expect` を呼び出してプログラムにパニックさせる代わりに、`Rusult` 型のエラー値を返す
  - `?` 演算子を使うことで `Result` が `Err` 値を持つときにその `Err` 値を `return` することができる<br/>（詳しくは9章の `### エラー委譲のショートカット: ?演算子` を参照のこと）
- `Box<dyn Error>` は、関数が `Error` トレイトを実装する型を返すことを意味する
  - 戻り値の型を具体的に指定しなくても良い
  - これにより、エラーケースによって異なる型のエラー値を返す柔軟性を得る

**`src/main.rs`**
```diff
// --snip--
+ use std::error::Error;

fn main() {
    // --snip--
}

- fn run(config: Config) {
+ fn run(config: Config) -> Result<(), Box<dyn Error>> {

-   let mut f = File::open(config.filename).expect("file not found");
+   let mut f = File::open(config.filename)?;
    
    let mut contents = String::new();
-   f.read_to_string(&mut contents)
-       .expect("something went wrong reading the file");
+   f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

#### main で run から返ってきたエラーを処理する
- `if let` で `run` が `Err` 値を返したかどうかを確認し、そうなら `process::exit(1)` を呼び出す
  - `unwrap_or_else` を使わない理由は？
    - --> `run` 関数を使う際には、 `Config::new` とは異なり、返り値が `Ok(...)` であるときの `(...)` の中身に興味がないから

**`src/main.rs`**
```diff
// --snip--

fn main() {
    // --snip--
-   run(config)
+   if let Err(e) = run(config) {
+       println!("Application error: {}", e);
+       process::exit(1);
+   }
}

fn run(config: Config) {
    // --snip--
}
```

### コードをライブラリクレートに分割する
- ライブラリの導入をよりそれらしい形にする（`extern crate` の使用）
  - 関数の導入時には、それを含むモジュールを導入し
  - 構造体などは、それ自体を導入するという慣習に従う
  - （詳しくは、7章の「use のパス指定の慣例」を参照せよ）
- `run` 関数を `src/lib.rs` に移動（それに伴って、`run` のみが依存するクレートに関する `use` 句も `src/lib.rs` に移動）

**`src/main.rs`**
```diff
use std::env;
- use std::fs::File;
- use std::io::prelude::*;
use std::process;

- use minigrep::*;
+ extern crate minigrep;
+ use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

-   if let Err(e) = run(config) {
+   if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
-
- fn run(config: Config) {
-     // --snip--
- }
```

**`src/lib.rs`**
```diff
+ use std::fs::File;
+ use std::io::prelude::*;
+ use std::error::Error;

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = &args[1];
        let filename = &args[2];

        Ok(Config { query, filename })
    }
}

+ pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
+     let mut f = File::open(config.filename)?;
+     
+     let mut contents = String::new();
+     f.read_to_string(&mut contents)?;
+ 
+     println!("With text:\n{}", contents);
+ 
+     Ok(())
+ }
```

## 12.4 テスト駆動開発でライブラリの機能を開発する
### 失敗するテストを記述する
**`src/lib.rs`**
```rust
// --snip--

pub struct Config<'a> {
    // --snip--
}

impl<'a> Config<'a> {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }
}
```
- test の実行結果
  - テストは全く想定通りに失敗する
```sh
$ cargo test
   Compiling minigrep v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/minigrep)

--snip--

running 1 test
test test::one_result ... FAILED

failures:

---- test::one_result stdout ----
thread 'test::one_result' panicked at 'assertion failed: `(left == right)`
  left: `["safe, fast, productive."]`,
 right: `[]`', src/lib.rs:49:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

### テストを通過させるコードを書く
- 文字列を行ごとに繰り返すメソッド `lines` メソッドを使う
  - `lines` メソッドはイテレータを返す
- ある文字列がクエリ文字列を含むか確認するために、`contains` メソッドを使う

**`src/lib.rs`**
```rust
// --snip--

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str>= Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }
}
```
- test を実行する
  - 期待通りに通る：
```sh
$ cargo test
   Compiling minigrep v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests (target/debug/deps/minigrep-662cb87b3d895995)

running 1 test
test test::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/minigrep-33abce92ed029d2f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### run 関数内で search 関数を使用する
**`src/lib.rs`**
```diff
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

-   println!("With text:\n{}", contents);
+   for line in search(config.query, &contents) {
+       println!("{}", line);
+   }

    Ok(())
}
```

- プログラムを実行してみる：
```sh
$ cargo run frog poem.txt
   Compiling minigrep v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/minigrep frog poem.txt`
Searching for frog
In file poem.txt
How public, like a frog
```
```sh
$ cargo run body poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep body poem.txt`
Searching for body
In file poem.txt
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```
```sh
$ cargo run monomorphization poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep monomorphization poem.txt`
Searching for monomorphization
In file poem.txt
```

## 12.5 環境変数を取り扱う
環境変数でユーザがオンにできる大文字小文字無視の検索用のオプションを追加する
### 大文字小文字を区別しない search 関数用に失敗するテストを書く
- 環境変数がオンの場合に呼び出す search_case_insensitive 関数を新しく追加
  - テストモジュールを以下のように書き換える：

**`src/lib.rs`**
```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str>= Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents)
        )
    }
}
```
- テストを実行：
  - 期待通り失敗する
```sh
$ cargo test
   Compiling minigrep v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/minigrep)

--snip--

running 2 tests
test test::case_insensitive ... FAILED
test test::case_sensitive ... ok

failures:

---- test::case_insensitive stdout ----
thread 'test::case_insensitive' panicked at 'assertion failed: `(left == right)`
  left: `["Rust:", "Trust me."]`,
 right: `[]`', src/lib.rs:77:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::case_insensitive

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

### `search_case_insensitive` 関数を実装する
- `to_lowercase` メソッドで `query` と各 `line` を小文字化

**`src/lib.rs`**
```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}
```
- テストを実行する
  - 期待通り成功する
```sh
$ cargo test
   Compiling minigrep v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 0.39s
     Running unittests (target/debug/deps/minigrep-662cb87b3d895995)

running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/minigrep-33abce92ed029d2f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### run 関数から新しい search_case_insensitive 関数を呼び出す
- `Config` 構造体に設定オプションを追加

**`src/lib.rs`**
```diff
pub struct Config {
    pub query: String,
    pub filename: String,
+   pub case_sensitive: bool,
}
```

- `run` 関数に、`case_sensitive` フィールドの値を確認し、`search` 関数 `search_case_insensitive` 関数を呼ぶかを決定してもらう
```diff
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

+   let results = if config.case_sensitive {
+       search(&config.query, &contents)
+   } else {
+       search_case_insensitive(&config.query, &contents)
+   };

-   for line in search(config.query, &contents) {
+   for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

- `new` メソッドを修正する
  - 環境変数を扱う関数は、標準ライブラリの `env` モジュールにある
    - `use std::env;` する必要がある
  - `env` モジュールから `var` 関数を使用して `CASE_INSENSITIVE` という環境変数のチェックを行う
    - `env::var` 関数は、環境変数がセットされていたら、環境変数の値を含む `Ok` 列挙子の成功値になる `Result` を返す
    - 環境変数がセットされていなければ、`Err` 列挙子を返す
  - ここでは、`Result` enum の `is_err` メソッドを使用して、
    - `CASE_INSENSITIVE` 環境変数がセットされていなければ `true`
    - `CASE_INSENSITIVE` 環境変数がセットされていれば `false` を返すようにしている
```diff
impl<'a> Config<'a> {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = &args[1];
        let filename = &args[2];

+       let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

-       Ok(Config { query, filename })
+       Ok(Config { query, filename, case_sensitive })
    }
}
```
- 実行してみる
```sh
$ cargo run to poem.txt
   Compiling minigrep v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/minigrep to poem.txt`
Searching for to
In file poem.txt
Are you nobody, too?
How dreary to be somebody!
```
```sh
$ export CASE_INSENSITIVE=1
$ echo $CASE_INSENSITIVE
1

$ cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
Searching for to
In file poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

## 12.6 標準出力ではなく標準エラーにエラーメッセージを書き込む
- 多くの端末は、2種類の出力を提供する: 
  - 普通の情報用の標準出力 (stdout) と
  - エラーメッセージ用の標準エラー出力 (stderr)
- ユーザは、エラーメッセージを画面に表示しつつ、プログラムの成功した出力をファイルにリダイレクトすることを選択できる
- `println!` は標準出力にしか出力する能力がない

### 標準出力の中身を画面の代わりにファイルに書き込む(ファイルにリダイレクトする)
- `>` 記法により、標準出力の中身を画面の代わりに `output.txt` に書き込む
  - この時、エラーの内容は
```sh
$ cargo run > output.txt
   Compiling minigrep v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/minigrep`
```
**`output.txt`**
```txt
Problem parsing arguments: not enough arguments!

```

### エラーを標準エラーに出力する
- `src/main.rs` を編集する：
  - 表示エラー出力に出力したい部分を `eprintln!` マクロに置き換える

**`src/main.rs`**
```diff
use std::env;
use std::process;

extern crate minigrep;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
-       println!("Problem parsing arguments: {}", err);
+       eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
-       println!("Application error: {}", e);
+       eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
```
- 実行してみる（エラーを起こす場合）：
```sh
$ cargo run > output.txt
   Compiling minigrep v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments!
```

**`output.txt`**
```txt

```
- 実行してみる（エラーを起こさない場合）：
```sh
$ cargo run to poem.txt > output.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
```
**`output.txt`**
```txt
Searching for to
In file poem.txt
Are you nobody, too?
How dreary to be somebody!

```

