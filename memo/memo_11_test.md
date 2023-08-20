<!-- markdownlint-configure-file {
  "no-inline-html": {
    "allowed_elements": [
      "br"
    ]
  }
} -->

# １１章：自動テストを書く

## 目次

- [１１章：自動テストを書く](#１１章自動テストを書く)
  - [目次](#目次)
  - [11.0 概要](#110-概要)
    - [テストの実行](#テストの実行)
    - [テストの定義](#テストの定義)
    - [単体テストの慣習](#単体テストの慣習)
    - [結合テストの慣習](#結合テストの慣習)
      - [ライブラリクレートの場合](#ライブラリクレートの場合)
      - [バイナリクレートの場合](#バイナリクレートの場合)
  - [11.1 テストの記述法](#111-テストの記述法)
  - [11.2 テスト関数の構成](#112-テスト関数の構成)
    - [ライブラリプロジェクトの生成とテスト関数の自動生成](#ライブラリプロジェクトの生成とテスト関数の自動生成)
    - [各種マクロ (`assert!`, `assert_eq!`, `assert_ne!`)](#各種マクロ-assert-assert_eq-assert_ne)
    - [`#[should_panic]`](#should_panic)
    - [Result\<T, E\>をテストで使う](#resultt-eをテストで使う)
  - [11.3 テストの実行のされ方を制御する](#113-テストの実行のされ方を制御する)
    - [テストを並行または連続して実行する](#テストを並行または連続して実行する)
    - [関数の出力を表示する](#関数の出力を表示する)
    - [名前でテストの一部を実行する](#名前でテストの一部を実行する)
    - [特に要望のない限りテストを無視する(`#[ignore]`)](#特に要望のない限りテストを無視するignore)
  - [11.4 テストの体系化](#114-テストの体系化)
    - [テストの種類](#テストの種類)
    - [単体テスト](#単体テスト)
      - [`#[cfg(test)]`](#cfgtest)
    - [結合テスト](#結合テスト)
      - [ライブラリクレート用の結合テスト](#ライブラリクレート用の結合テスト)
      - [バイナリクレート用の結合テスト](#バイナリクレート用の結合テスト)

## 11.0 概要

### テストの実行

- テストは `cargo test` で実行できる

- `cargo test --(cargo test にかかるオプション) --(テストバイナリにかかるオプション)` というふうにオプションを追加できる
  - （cf. `cargo test` はコードをテストモードでコンパイルし、出来上がったテストバイナリを実行することに注意
  - この文書では `cargo test` のオプションとして以下を紹介：
    - `cargo test -- --help`
    - `cargo test -- --test-threads=1`
    - `cargo test -- --nocapture`

- また、`cargo test テスト関数名の一部` と指定すると指定したテキストを名前の一部に含むテスト関数のみが実行される

### テストの定義

- Rust では `fn` の前に `#[test]` が付与してあれば、テスト用の関数とみなされる

- `#[test]` に加えて `#[ignore]` が付与された関数は基本的に実行されない
  - `cargo test -- --ignored` を実行すれば、`#[ignore]` が付与されたテストのみを実行できる

- `cargo new hoge --lib` 実行時（ライブラリプロジェクト作成時）には、自動でテストモジュールが作成される

- 各種マクロ：
  - `assert!(b)`: `b` は `true`？
  - `assert_eq!(left, right)`: `left` == `right`？
  - `assert_ne!(left, right)`: `left` != `right`？
- 各種マクロの第３引数にフォーマット、それ以降にフォーマットに組み込む値を列挙すると、失敗時のメッセージを定義できる

- `#[test]` に加えて `#[should_panic]` を付与すると関数内のコードがパニックしたら、テストを通過させる
  - 以下のように制約を加えることも可能：`#[should_panic(expected = "失敗メッセージに含まれていることを期待するメッセージ内容")]`

- `#[test]` が付与された関数内の返り値の型を `Result<T, E>` にすると、「`Ok(...)` が返るとテスト成功」「`Err(...)` が返るとテスト失敗」となる
  - このようなテスト関数内では `?` を使えるので便利

### 単体テストの慣習

- 単体テストのテスト関数は、`src` ディレクトリの各ファイル内でテスト対象のコードと共に記述する
- 以下のようにテスト用のモジュールを定義する：

```rust
// ... テスト対象のコード

#[cfg(test)]
mod tests {
  #[test]
  fn hoge () {
    // テスト関数の内容
  }

  // ...他のテスト関数
}
```

- `#[cfg(test)]` で注釈されたモジュールは `cargo build` 時には無視されて、`cargo test` 時にのみコンパイルされる

### 結合テストの慣習

#### ライブラリクレートの場合

- `src` と同階層に `tests` ディレクトリを作成
  - `tests/hoge.rs` 内にテストを記述
  - なお、`#[cfg(test)]` 注釈は不要
  - `extern crate hoge;` のようにクレートを導入すること

- 特定のテストファイルを実行するには、`cargo test --test テストファイル名（拡張子抜き）` する

- `tests` ディレクトリのサブディレクトリ内のファイルは個別クレートとしてコンパイルされたり、テスト出力に区域が表示されることがない
  - &rarr; `tests/hoge/fuga.rs` を作成することで、各テストに共通の関数などを定義できる

#### バイナリクレートの場合

- バイナリを提供する Rust のプロジェクトの基本形は、
`src/lib.rs` ファイルに存在するロジックを呼び出す単純な `src/main.rs` ファイルとするべし！
- あとのテスト手順は [ライブラリクレートの場合](#ライブラリクレートの場合) に従えばよい

## 11.1 テストの記述法

- テスト関数の本体は、典型的には以下の 3 つの動作を行う
  1. 必要なデータや状態をセットアップ
  2. テスト対象のコードを走らせる
  3. 結果が想定通りであることを断定（アサーション）する

## 11.2 テスト関数の構成

- Rust におけるテストは test 属性で注釈された関数
- 関数をテスト関数に変えるには、`fn` の前に `#[test]` を付け加える
- `cargo test` コマンドでテストを実行すると
  - コンパイラは `test` 属性で注釈された関数を走らせるテスト用バイナリをビルド
  - 各テスト関数が通過したか失敗したかを報告する

### ライブラリプロジェクトの生成とテスト関数の自動生成

- 新しいライブラリプロジェクトを Cargo で作ると、テスト関数付きのテストモジュールが自動的に生成される

  ```sh
  cargo new adder --lib
  ```

- テストを実行してみる

  ```sh
  $ cargo test
     Compiling adder v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/adder)
      Finished test [unoptimized + debuginfo] target(s) in 0.33s
       Running unittests (target/debug/deps/adder-9e0eca6a61a7cecc)

  running 1 test
  test tests::it_works ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Doc-tests adder

  running 0 tests

  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```

### 各種マクロ (`assert!`, `assert_eq!`, `assert_ne!`)

- `assert!(bool)` --> `bool` の値が `True` になることを期待する
- `assert_eq!(left, right)` と `assert_ne!(left, right)` --> `left` と `right` の等値性をテストする
  - 比較対象の値は `PartialEq` と `Debug` トレイトを実装していなければならない（理由は以下の通り）
    - `assert_eq!` と `assert_ne!` マクロは、それぞれ `==` と `!=` 演算子を使用.
    - 引数をデバッグフォーマットを使用してプリントする.
  - => 構造体や enum 定義に `#[derive(PartialEq, Debug)]` という注釈を追加すればよい
  - 失敗メッセージと共にカスタムのメッセージが表示されるよう、追加することもできる(第３引数にフォーマット、それ以降にフォーマットに組み込む値を列挙する)
    - 例：

      ```rust
      #[test]
      fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
          result.contains("Carol"),
          "Greeting did not contain name, value was `{}`",
          result
        );
      }
      ```

### `#[should_panic]`

- `#[should_panic]` --> 関数内のコードがパニックしたら、テストを通過させる

  ```rust
  #[test]
  #[should_panic] 
  fn another() {
      panic!()
  }
  ```

- `expected` 引数を追加して、失敗メッセージに与えられたテキストが含まれていることを確かめる

  ```rust
  #[test]
  #[should_panic(expected = "Guess value must be less than or equal to 100")]
  fn greater_than_100() {
    Guess::new(200);
  }
  ```

### Result<T, E>をテストで使う

- 例：関数内で `assert_eq!` マクロを呼び出す代わりにテストが成功すれば `Ok(())` を、失敗すれば `Err` に `String` を入れて返す

  ```rust
  #[test]
  fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }
  ```

- `Result<T, E>` を返すようなテストを書くと、`?` 演算子をテストの中で使える
- `Result<T, E>` を使うテストに `#[should_panic]` 注釈を使うことはできないので注意

## 11.3 テストの実行のされ方を制御する

- `cargo test` はコードをテストモードでコンパイルし、出来上がったテストバイナリを実行
- コマンドラインオプションを指定して `cargo test` の既定動作を変更することができる
  - `cargo test --(cargo test にかかるオプション) --(テストバイナリにかかるオプション)`
  - 例：`cargo test --help`: `cargo test` で使用できるオプションの一覧を表示
  - 例：`cargo test -- --help`: `--` という区分記号の後に使えるオプションの一覧を表示

### テストを並行または連続して実行する

- 複数のテストを実行するとき、標準では、スレッドを使用して並行に走る
- 並行にテストを実行したくなかったり、使用されるスレッド数をよりきめ細かく制御したい場合 --> `--test-threads` フラグと使用したいスレッド数をテストバイナリに送る
- 例：

  ```sh
  cargo test -- --test-threads=1
  ```

### 関数の出力を表示する

- 標準ではテスト時に標準出力にはテスト結果と、失敗したテストからの標準出力しか表示されない
- 通過するテストについても出力される値が見たかったら、出力キャプチャ機能を`--nocapture` フラグで無効化することができる
- 例：

  ```sh
  cargo test -- --nocapture
  ```

### 名前でテストの一部を実行する

- `cargo test` に走らせたいテストの名前を引数として渡すことで、実行するテストを選ぶことができる
- 例：`one_hundred` というテスト関数だけ実行する

  ```sh
  cargo test one_hundred
  ```

- 例：テスト名の一部を指定して、その値に合致するあらゆるテストを走らせる

  ```sh
  $ cargo test add
  (add と名の付くすべてのテスト関数が実行される)
  ```

- テスト名には、テスト関数を含むモジュール名も含むので、**モジュール名でフィルターをかけることで、あるモジュール内のテスト全てを実行できる**

### 特に要望のない限りテストを無視する(`#[ignore]`)

- `#[ignore]` 属性でテストを除外する
- 例：

  ```rust
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  #[ignore]
  fn expensive_test() {
    // code that takes an hour to run
  }
  ```

- 無視されるテストのみを実行したかったら、`cargo test -- --ignored` を使う

## 11.4 テストの体系化

### テストの種類

- 単体テスト
  - 個別に 1 回に 1 モジュールをテスト
  - 非公開のインターフェイスもテストする
- 結合テスト
  - 外部コード同様に自分のコードを使用
  - 公開インターフェイスのみ使用
  - 1 テストにつき複数のモジュールを用いることもある

### 単体テスト

- テスト対象となるコードと共に、`src` ディレクトリの各ファイルに置く
- 慣習にしたがえば、各ファイルに `tests` という名前のモジュールを作り、テスト関数を含ませ、そのモジュールを `#[cfg(test)]` で注釈する

#### `#[cfg(test)]`

- `#[cfg(test)]` という注釈は、`cargo build` した時ではなく、`cargo test` した時だけ、テストコードをコンパイルし走らせるよう指示
- 単体テストはコードと同じファイルに存在するので、`#[cfg(test)]` を使用してコンパイル結果に含まれないよう指定する
  - `cfg` という属性は、configuration を表していて、コンパイラに続く要素が、ある特定の設定オプションを与えられたら、含まれるように指示する
  - たとえば、`#[cfg(test)]` とすれば `cargo test` で積極的にテストを実行した場合のみ、Cargo がテストコードをコンパイルする

### 結合テスト

#### ライブラリクレート用の結合テスト

- 結合テストを作成するには、まず **`tests` ディレクトリ**が必要になる
  - プロジェクトディレクトリのトップ階層、`src` の隣に `tests` ディレクトリを作成
  - このディレクトリ内にいくらでもテストファイルを作成可能
  - Cargo はそれぞれのファイルを個別のクレートとしてコンパイルする
  - `Cargo` は `tests` ディレクトリを特別に扱い、`cargo test` を走らせた時にのみこのディレクトリのファイルをコンパイルする
    - --> `tests/hoge.rs` 内のコードは `#[cfg(test)]` で注釈する必要がない
- **特定のテストファイルを実行**させるには、`cargo test --test <テストファイル名（拡張子抜き）>` とすればよい

  ```sh
  $ cargo test --test integration_test
  (このコマンドは、`tests/integration_test.rs` ファイルにあるテストのみを実行)
  ```

- `tests` ディレクトリのサブディレクトリ内のファイルは個別ク
レートとしてコンパイルされたり、テスト出力に区域が表示されることがない
  - --> `tests/hoge/mod.rs` を作成することで、テスト結果の出力には影響を与えずに、共通関数などを定義できる
    - （`tests/hoge/mod.rs` で定義した関数を `tests/fuga.rs` から呼び出すことができる、）
  - 例：<br/>
    **`tests/integration_test.rs`**

    ```rust
    extern crate adder;

    mod common;

    #[test]
    fn it_adds_two() {
      common::setup();
      assert_eq!(4, adder::add_two(2));
    }
    ```

    **`tests/common/mod.rs`**

    ```rust
    pub fn setup() {
      println!("do nothing...");
    }
    ```

#### バイナリクレート用の結合テスト

もしもプロジェクトが `src/main.rs` ファイルのみを含み、`src/lib.rs` ファイルを持たないバイナリクレートだったら...

- `tests` ディレクトリに結合テストを作成し、`extern crate <パッケージ名>;` を使用して `src/main.rs` ファイルに定義された関数をインポートすることはできない
- `extern crate <パッケージ名>;` を使用して呼び出せるのはいつでも `src/lib.rs` で定義されるクレート

--> **バイナリを提供する Rust のプロジェクトの基本形**は、<br/>
**`src/lib.rs` ファイルに存在するロジックを呼び出す単純な `src/main.rs` ファイル**とすべき

- この形なら、`extern crate <パッケージ名>;` を使用してプロジェクトの主要な機能をテストすることができる
- ただし、`src/main.rs` ファイルの少量のコードはテストできないので注意
