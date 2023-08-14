# ９章

## 目次

- [９章](#９章)
  - [目次](#目次)
  - [9.0 概要](#90-概要)
  - [9.1 `panic!` で回復不能なエラーを記述する](#91-panic-で回復不能なエラーを記述する)
    - [panic!バックトレースを使用する](#panicバックトレースを使用する)
  - [Result で回復可能なエラーを記述する](#result-で回復可能なエラーを記述する)
    - [unwrap と expect（`Result<T, E>` 型のヘルパーメソッドの一例）](#unwrap-と-expectresultt-e-型のヘルパーメソッドの一例)
    - [エラーを委譲する](#エラーを委譲する)
    - [エラー委譲のショートカット: ?演算子](#エラー委譲のショートカット-演算子)
  - [9.3 panic!すべきかするまいか](#93-panicすべきかするまいか)
    - [要約](#要約)

## 9.0 概要

- `panic!`
- `Result<T, E>` enum
  - `unwrap` と `expect`

## 9.1 `panic!` で回復不能なエラーを記述する

- `panic!("クラッシュ時に表示するメッセージ")` でプログラムを強制終了させることができる
  - パニック時の動作は次の二通り：
    - プログラムは巻き戻しを始め、言語がスタックを遡り、遭遇した各関数のデータを片付ける
    - 即座に異常終了し、片付けをせずにプログラムを終了。片付けはOSに任せる（型付けの処理をOSに任せる分、実行可能ファイルは小さくなる）
  - 実行可能ファイルを極力小さくする必要があれば、`Cargo.toml` ファイルの適切な `[profile]` 欄に `panic = 'abort'` を追記する(パニック時に巻き戻すのではなく、異常終了するように切り替えることができる)

    ```toml
    [profile.release]
    panic = 'abort'
    ```

### panic!バックトレースを使用する

- たとえば、以下のコードはパニックを起こす

  ```rust
  fn main() {
    let v = vec![1, 2, 3];

    v[99];
  }
  ```

  ```sh
  $ cargo run 
   Compiling example_09_01 v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/example_09_01)
      Finished dev [unoptimized + debuginfo] target(s) in 0.28s
       Running `target/debug/example_09_01`
  thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  ```

- コンパイラの指示通りに`RUST_BACKTRACE=1`を設定して cargo を走らせると、バックトレースを確認できる
  - バックトレースを読むコツは、頭からスタートして自分のファイルを見つけるまで読むこと
  - そこが、問題の根源になる
  - 自分のファイルを言及している箇所以前は、自分のコードで呼び出したコード; それ以後は、自分のコードを呼び出しているコードなので無視してもよい

  ```sh
  $ RUST_BACKTRACE=1 cargo run
      Finished dev [unoptimized + debuginfo] target(s) in 0.00s
       Running `target/debug/example_09_01`
  thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
  stack backtrace:
     0: rust_begin_unwind
               at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/std/src/panicking.rs:584:5
     1: core::panicking::panic_fmt
               at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/panicking.rs:143:14
     2: core::panicking::panic_bounds_check
               at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/panicking.rs:85:5
     3: <usize as core::slice::index::SliceIndex<[T]>>::index
               at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/slice/index.rs:189:10
     4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
               at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/slice/index.rs:15:9
     5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
               at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/alloc/src/vec/mod.rs:2531:9
     6: example_09_01::main
               at ./src/main.rs:4:5
     7: core::ops::function::FnOnce::call_once
               at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/ops/function.rs:227:5
  note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
  ```

## Result で回復可能なエラーを記述する

- `Result` enum は以下のように `Ok` と `Err` の 2 列挙子からなるよう定義されている

  ```rust
  enum Result<T, E> {
    Ok(T),
    Err(E),
  }
  ```

- 例：ファイルを開けたらそのハンドルを返し、開けなかったらパニックを起こす

  ```rust
  use std::fs::File;
  
  fn main() {
  let f: Result<std::fs::File, std::io::Error> = File::open("hello.txt");
  
  let f = match f {
      Ok(file) => file,
      Err(error) => {
        panic!("There was a problem opening the file: {:?}", error)
      },
    };
  }
  ```

- 例：ファイルが存在しないために File::open が失敗したら、ファイルを作成し、その新しいファイルへのハンドルを返す
  - 以下のコードの `if error.kind() == ErrorKind::Notfound` という条件式は、マッチガードと呼ばれる
    - アームのパターンをさらに洗練する match アーム上のおまけの条件式
    - この条件式は、そのアームのコードが実行されるには真でなければいけない
    - パターンの `ref` は、`error` がガード条件式にムーブされないように必要
      - パターンの文脈において、`&` は参照にマッチし、その値を返す
      - `ref` は値にマッチし、それへの参照を返す

  ```rust
  use std::fs::File;
  use std::io::ErrorKind;
  fn main() {
      fn main() {
          let f = File::open("hello.txt");
          let f = match f {
              Ok(file) => file,
              Err(ref error) if error.kind() == ErrorKind::NotFound => {
                  match File::create("hello.txt") {
                      Ok(fc) => fc,
                      Err(e) => {
                          panic!("Tried to create file but there was a problem: {:?}", e)
                      }
                  }
              }
              Err(error) => {
                  panic!("There was a problem opening the file: {:?}", error)
              }
          };
      }
  }
  ```

### unwrap と expect（`Result<T, E>` 型のヘルパーメソッドの一例）

- `unwrap`
  - `Result` 値が `Ok` 列挙子なら、`unwrap` は `Ok` の中身を返し
  - `Err` 列挙子なら、`unwrap` は `panic!` マクロを呼んでくれる

  ```rust
  use std::fs::File;
  
  fn main() {
    let f = File::open("hello.txt").unwrap();
  }
  ```

- `expect`
  - 動作は `unwrap` と同じ
  - ただし、`unwrap` とは異なり、`panic!` のエラーメッセージを記述できる

  ```rust
  use std::fs::File;
  
  fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
  }
  ```

### エラーを委譲する

失敗する可能性のある何かを行う関数を書く際、関数内でエラーを処理する代わりに、呼び出し元がどうするかを決められるようにエラーを返すことができる（エラーの委譲）

- 例１：

  ```rust
  use std::io;
  use std::io::Read;
  use std::fs::File;

  fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    
    let mut f = match f {
      Ok(file) => file,
      Err(e) => return Err(e),
    };

    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
      Ok(_) => Ok(s),
      Err(e) => Err(e),
    }
  }
  ```

### エラー委譲のショートカット: ?演算子

- Result 値の直後に置かれた`?` は
  - Result の値が `Ok` なら、`Ok` の中身がこの式から返ってきて、プログラムは継続
  - 値が Err なら、`return` キーワードを使ったかのように関数全体から Err の中身が返ってくる
- `?` 演算子は、Result を返す関数でしか使用できない（`Err` をリターンするため）
  - もしこのルールを破るとコンパイルエラーが起きる

- 例：下のコードは、先ほどの例１とほぼ同じ動作をする

  ```rust
  use std::io;
  use std::io::Read;
  use std::fs::File;
  
  fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    
    let mut s = String::new();
    
    f.read_to_string(&mut s)?;
    Ok(s)
  }
  ```

- これは以下のようにさらに短く書くこともできる

  ```rust
  use std::io::Read;
  use std::fs::File;

  fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    
    File::open("hello.txt")?.read_to_string(&mut s)?;
    
    Ok(s)
  }
  ```

- 一つの関数内で複数の `?` を使ったときに、各々の `Err` 型が異なる場合はどうすればいいの...?
  - `?` を使ったエラー値は、標準ライブラリの `From` トレイトで定義され、エラーの型を別のものに変換する `from` 関数を通る
  - `?` 演算子が `from` 関数を呼び出すと、受け取ったエラー型が現在の関数の戻り値型で定義されているエラー型に変換される
  - つまり、`?` 演算子を使う限り、`Err`の型変換の面倒を `from` 関数が自動的に見てくれる

## 9.3 panic!すべきかするまいか

--> 読み物に近いのでドキュメントを参照すること

- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch09-03-to-panic-or-not-to-panic.html)

### 要約

- 多くの場合では、`Result` を返却することは、失敗する可能性のある関数を定義する際には、いい第一選択肢になる
  - 呼び出し側に選択肢を与えられるから
- しかし、例やプロトタイプコード、テストではパニックするのが適切になりうる
  - 例を示すのにエラー処置コードも添えるのは本質が分かりにくくなる
  - エラーの処理法を決定する準備ができる前の、プロトタイプの段階では、`unwrap` や `expect` メソッドを使うことで、「より頑強なエラー処理を実現したいならここを直すといいですよ」というマーカーになる
  - メソッド呼び出しがテスト内で失敗したら、そのメソッドがテスト下に置かれた機能ではなかったとしても、テスト全体が失敗してほしい

- コードが悪い状態に陥る可能性があるときにパニックさせるのは、推奨されること
  - ここでいう悪い状態とは、何らかの前提、保証、契約、不変性が破られたこと。例を挙げれば、
    1. 以下のいずれかがコードに渡され、
       - 無効な値
       - 矛盾する値
       - 行方不明な値
    2. かつ、以下のいずれか一つ以上の状態であるときをいう
       - 悪い状態がときに起こるとは予想されないとき
       - この時点以降、この悪い状態にないことを頼りにコードが書かれているとき
       - 使用している型にこの情報をコード化するいい手段がないとき

- 誰かが自分のコードを呼び出して筋の通らない値を渡してきたら、最善の選択肢は panic! し、自分たちのコードにバグがあることをライブラリ使用者に通知すること
- 自分の制御下にない外部コードを呼び出し、修正しようのない無効な状態を返すときに panic! はしばしば適切

- 関数にはしばしば契約が伴う: 入力が特定の条件を満たすときのみ、振る舞いが保証される
  - 契約が侵されたときにパニックすることは、道理が通っている
  - なお、関数の契約は、関数の API ドキュメント内で説明されているべき

- Rust の型システムを使用して契約をコードに落とし込むために独自の型を作ることも有効
- 例：１以上100以下の値のみを取る独自の整数型 Guess を実装する

  ```rust
  pub struct Guess {
    value: u32,
  }
  
  impl Guess {
    pub fn new(value: u32) -> Guess {
      if value < 1 || value > 100 {
        panic!("Guess value must be between 1 and 100, got {}.", value);
      }

      Guess {
        value
      }
    }

    pub fn value(&self) -> u32 {
      self.value
    }
  }
  ```
