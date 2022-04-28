# ７章
## 目次
- [７章](#７章)
  - [目次](#目次)
  - [7章概略](#7章概略)
  - [7.1 パッケージとクレート](#71-パッケージとクレート)
  - [7.2 モジュールを定義して、スコープとプライバシーを制御する](#72-モジュールを定義してスコープとプライバシーを制御する)
    - [モジュール](#モジュール)
      - [モジュールツリー](#モジュールツリー)
  - [7.3 モジュールツリーの要素を示すためのパス](#73-モジュールツリーの要素を示すためのパス)
    - [モジュールのパス](#モジュールのパス)
    - [プライバシー境界](#プライバシー境界)
    - [相対パスを `super` で始める](#相対パスを-super-で始める)
    - [構造体と enum を公開する](#構造体と-enum-を公開する)
      - [struct のプライバシー設定](#struct-のプライバシー設定)
      - [enum のプライバシー設定](#enum-のプライバシー設定)
  - [7.4 use キーワードでパスをスコープに持ち込む](#74-use-キーワードでパスをスコープに持ち込む)
    - [`use` のパス指定の慣例](#use-のパス指定の慣例)
      - [`as`](#as)
    - [`pub use` で名前を再公開する](#pub-use-で名前を再公開する)
    - [`use` をネストさせて省略する](#use-をネストさせて省略する)
    - [glob 演算子](#glob-演算子)
  - [7.5 モジュールを複数のファイルに分割する](#75-モジュールを複数のファイルに分割する)


## 7章概略
- モジュール：構造体、enum、定数、トレイト、関数、あるいは他のモジュールなどの要素をまとめたもの
- モジュールはクレートルート内に形成される `crate` と呼ばれるモジュールを根ノードとする木構造をなす（モジュールツリー）
  - クレートルート：Rust コンパイラの開始点となるソースファイル. `src/main.rs`, `src/lib.rs`, `src/bin/*.rs` のいずれか.
- クレート：一つのモジュールツリー
- パッケージ：複数のクレートをまとめたもの
  - 生成には `cargo new <パッケージ名>` コマンドを用いる

## 7.1 パッケージとクレート
- **クレート**: バイナリ or ライブラリ
  - 関連した機能を一つのスコープにまとめることで、その機能を複数のプロジェクト間で共有しやすくする
  - 例：rand クレート（乱数を生成する機能を提供）
  - あるクレートを他のプロジェクトに持ち込む際、そのクレートが提供する機能には、そのクレートの名前（例えば `rand`）を通じてアクセスできる

- **パッケージ**: ある機能群を提供する 1 つ以上のクレート
  - パッケージの作成は、
    ```sh
    $ cargo new <package name>
    ```
    で実行可能<br/>（「パッケージとは `cargo new` コマンドで作成できるもの」と認識するのがよさそう）
  - パッケージ生成時には、`Cargo.toml`, `src/main.rs` が作成される
    - **`Cargo.toml`**: パッケージ内のクレートをどのようにビルドするかを説明するファイル. パッケージ名やバージョンが記載されている
      ```toml
      [package]
      name = "test_07"
      version = "0.1.0"
      edition = "2021"

      # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

      [dependencies]
      ```
    - `src/main.rs`: パッケージと同じ名前を持つバイナリクレートのクレートの主要部分（クレートルート）
  - `Cargo.toml` がおかれたディレクトリをパッケージディレクトリと呼ぶ
  - パッケージは**0 個か 1 個のライブラリクレート**を持っていないといけない
  - バイナリクレートはいくらでも持って良い


- **クレートルート (crate root)**: Rust コンパイラの開始点となり、クレートのルートモジュールを作るソースファイル
  - Cargo はクレートルートファイルを rustc に渡し、ライブラリやバイナリをビルドする
  - クレートルートは以下のように決まる
    - `src/main.rs` は、パッケージと同じ名前を持つバイナリクレートのクレートルートである
    - パッケージディレクトリに `src/lib.rs` が含まれていたら、パッケージにはパッケージと同じ名前のライブラリクレートが含まれており、`src/lib.rs` がそのクレートルートである
    - ファイルを `src/bin` ディレクトリに置くことで、パッケージは複数のバイナリクレートを持つ。それぞれのファイルが別々のバイナリクレートになる。

## 7.2 モジュールを定義して、スコープとプライバシーを制御する
### モジュール
- モジュールは、mod キーワードを書き、次にモジュールの名前を指定することで定義されます
- モジュールの中には、他のモジュールをおくこともできます
- モジュールにはその他の要素の定義も置くことができます。
  - 例えば、構造体、enum、定数、トレイト、関数
  ```rs
  mod front_of_house {
      mod hosting {
          fn add_to_waitlist() {}
          fn seat_at_table() {}
      }
      mod serving {
          fn take_order() {}
          fn serve_order() {}
          fn take_payment() {}
      }
  }
  ```
- モジュールを使うことで、関連する定義を一つにまとめ、関連する理由を名前で示せます

#### モジュールツリー
- 上記の性質からモジュールは、**モジュールツリー**と呼ばれる木構造をなす
- `src/main.rs` や `src/lib.rs` などのクレートルートでは、そのファイルの中身が `crate` というモジュールを形成する
- 例：
  ```
  crate
    └ ─ ─ front_of_house
                ├ ─ ─ hosting
                │       ├ ─ ─ add_to_waitlist
                │       └ ─ ─ seat_at_table
                └ ─ ─ serving
                        ├ ─ ─ take_order
                        ├ ─ ─ serve_order
                        └ ─ ─ take_payment
  ```

## 7.3 モジュールツリーの要素を示すためのパス
### モジュールのパス
- モジュールツリー内の要素を呼び出すには**パス**を使う
  - 絶対パス: クレートの名前か `crate` という文字列を使うことで、クレートルートからスタートします
  - 相対パス: `self`, `super` または今のモジュール内の識別子を使うことで、現在のモジュールからスタートします
  - 絶対パスも相対パスも、その後に一つ以上の識別子がダブルコロン (`::`) で仕切られて続きます
- 例：(プライバシーに関する記述がないのでコンパイルエラーになる)
  ```rs
  mod front_of_house {
      mod hosting {
          fn add_to_waitlist() {}
          fn seat_at_table() {}
      }
      mod serving {
          fn take_order() {}
          fn serve_order() {}
          fn take_payment() {}
      }
  }

  pub fn eat_at_restaurant() {
      // 絶 対 パ ス
      crate::front_of_house::hosting::add_to_waitlist();
      // 相 対 パ ス
      front_of_house::hosting::add_to_waitlist();
  }
  ```

### プライバシー境界
モジュールは Rust のプライバシー境界も定義します
- あらゆる要素は標準では非公開（関数、メソッド、構造体、enum、モジュールおよび定数）
  - 親モジュールの要素は子モジュールの非公開要素を使えない
  - 子モジュールの要素はその祖先モジュールの要素を使える
  - 兄弟要素は自由に参照できる
- 子モジュールの内部部品を外部の祖先モジュールに見せるには `pub` キーワードを使う
- 例：
  ```rs
  mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
          fn seat_at_table() {}
      }
      mod serving {
          fn take_order() {}
          fn serve_order() {}
          fn take_payment() {}
      }
  }

  pub fn eat_at_restaurant() {
      // 絶 対 パ ス
      crate::front_of_house::hosting::add_to_waitlist();
      // 相 対 パ ス
      front_of_house::hosting::add_to_waitlist();
  }
  ```

### 相対パスを `super` で始める
- `super`: 親モジュールから始まる相対パスを記述するのに用いる（`../` のようなもの）
- 例：
  ```rs
  fn serve_order() {}

  mod back_of_house {
      fn fix_incorrect_order() {
          cook_order();
          super::serve_order();
      }
      fn cook_order() {}
  }
  ```

### 構造体と enum を公開する
#### struct のプライバシー設定
- 構造体定義の前に `pub` を使うと、構造体は公開されるが、**構造体のフィールドは非公開**のまま
- 構造体のフィールドを公開に設定するには、そのフィールド名の前にも `pub` をつける必要がある
- 例：
  ```rs
  mod back_of_house {
      pub struct Breakfast {
          pub toast: String,
          seasonal_fruit: String,
      }
      impl Breakfast {
          pub fn summer(toast: &str) -> Breakfast {
              Breakfast {
                  toast: String::from(toast),
                  seasonal_fruit: String::from("peaches"),
              }
          }
      }
  }
  pub fn eat_at_restaurant() {
      // 夏(summer)にライ麦(Rye)パン付き朝食を注文
      let mut meal = back_of_house::Breakfast::summer("Rye");
      //やっぱり別のパンにする
      meal.toast = String::from("Wheat");
      println!("I'd like {} toast please", meal.toast);

      // 下の行のコメントを外すとコンパイルできない。食事についてくる
      // 季節のフルーツを知ることも修正することも許されていないので
      // meal.seasonal_fruit = String::from("blueberries");
  }
  ```
  > `back_of_house::Breakfast` は非公開のフィールドを持っているので、`Breakfast` のインスタンスを作成 (construct) する公開された関連関数が構造体によって提供されている必要があります（ここでは `summer` と名付けました）. <br/>もし `Breakfast` にそのような関数がなかったら、`eat_at_restaurant` において非公開である `seasonal_fruit` の値を設定できないので、Breakfast のインスタンスを作成できません.

#### enum のプライバシー設定
- enum を公開すると、そのヴァリアントはすべて公開される

## 7.4 use キーワードでパスをスコープに持ち込む
- `use` キーワードを使うことで、パスを一度スコープに持ち込んでしまえば、それ以降はパス内の要素がローカルにあるかのように呼び出すことができる
- スコープに `use` で持ち込まれたパスも、他のパスと同じようにプライバシーがチェックさ
れる
- 例：
  ```rs
  mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
      }
  }
  
  use crate::front_of_house::hosting;
  // use self::front_of_house::hosting; でも同じことができる

  pub fn eat_at_restaurant() {
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
  }
  ```

### `use` のパス指定の慣例
- **関数を `use` で持ち込む**際には、**その関数を含むモジュールをパスで指定**するのが慣例
  - すなわち、その関数を呼び出すためには `<モジュール名>::<関数名>` とする
  - 関数の親モジュールを `use` で持ち込むことで、関数がローカルで定義されていないことを明らかにできる（どこで定義されているかも明らかになる）
- 一方、**構造体や enum その他の要素**を `use` で持ち込むときは、**フルパス**を書くのが慣例的
  - 同じ名前を持つ 2 つの型を同じスコープに持ち込むには親モジュールを使わないと
いけない
- 例：
  ```rs
  use std::collections::HashMap;

  fn main() {
      let mut map = HashMap::new();
      map.insert(1, 2);
  }
  ```
  ```rust
  use std::fmt;
  use std::io;

  fn function1() -> fmt::Result {
      // --snip--
      // （ 略 ）
  #     Ok(())
  }
  fn function2() -> io::Result<()> {
      // --snip--
      // （ 略 ）
  #     Ok(())
  }
  ```

#### `as`
- 同じ名前の 2 つの型を use を使って同じスコープに持ち込むという問題を解決するには `as` を使えばよい
- 例：
  ```rust
  use std::fmt;
  use std::io::Result as IoResult;

  fn function1() -> Result {
      // --snip--
      // （ 略 ）
  #     Ok(())
  }
  fn function2() -> IoResult<()> {
      // --snip--
      // （ 略 ）
  #     Ok(())
  }
  ```

### `pub use` で名前を再公開する
- use キーワードで名前をスコープに持ちこんだ時、新しいスコープで使用できるその名前は非公開
- これを公開に設定するには `pub` をつければよい
- 例：
  ```rust
  
  mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
      }
  }

  pub use crate::front_of_house::hosting;

  pub fn eat_at_restaurant() {
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
  }
  ```
  > pub use を使うことで、外部のコードが hosting::add_to_waitlist を使って add_to_waitlist 関数を呼び出せるようになりました。pub use を使っていなければ、eat_at_restaurant 関数は hosting::add_to_waitlist を自らのスコープ内で使えるものの、外部のコードはこの新しいパスを利用することはできないでしょう。

### `use` をネストさせて省略する
- ネストしたパスを使うことで、複数の要素を 1 行でスコープに持ち込める
- 例：
  ```rust
  use std::cmp::Ordering;
  use std::io;
  ```
  ↓
  ```rust
  use std::{cmp::Ordering, io};
  ```
  ```rust
  use std::io;
  use std::io::Write;
  ```
  ↓
  ```rust
  use std::io::{self, Write};
  ```

### glob 演算子
- パスにおいて定義されているすべての公開要素をスコープに持ち込みたいときは、glob 演算子`*` をそのパスの後ろに続けて書きましょう
  ```rust
  use std::collections::*;
  ```

## 7.5 モジュールを複数のファイルに分割する
- 例えば<br/>
  **`src/lib.rs`**
  ```rs
  mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
      }
  }

  pub use crate::front_of_house::hosting;

  pub fn eat_at_restaurant() {
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
  }
  ```
  を複数のファイルの分割するには以下のようにすればよい<br/>
  **`src/lib.rs`**
  ```rs
  mod front_of_house;

  pub use crate::front_of_house::hosting;

  pub fn eat_at_restaurant() {
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
  }
  ```
  **`src/front_of_house.rs`**
  ```rs
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
  ```
  さらに分割するには以下のようにする：<br/>
  **`src/lib.rs`**
  ```rs
  mod front_of_house;

  pub use crate::front_of_house::hosting;

  pub fn eat_at_restaurant() {
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
  }
  ```
  **`src/front_of_house.rs`**
  ```rs
  pub mod hosting;
  ```
  **`src/front_of_house/hosting.rs`**
  ```rs
  pub fn add_to_waitlist() {}
  ```