# １９章ー５節：高度な機能ーマクロ

## 目次

- [１９章ー５節：高度な機能ーマクロ](#１９章ー５節高度な機能ーマクロ)
  - [目次](#目次)
  - [19.5.0 マクロの概要](#1950-マクロの概要)
  - [19.5.1 マクロと関数の違い](#1951-マクロと関数の違い)
  - [19.5.2 macro\_rules! を用いた宣言的なマクロでメタプログラミング](#1952-macro_rules-を用いた宣言的なマクロでメタプログラミング)
    - [マクロの定義の例：簡略化した `vec!` マクロ](#マクロの定義の例簡略化した-vec-マクロ)
  - [19.5.3 属性からコードを生成する手続き的マクロの](#1953-属性からコードを生成する手続き的マクロの)
    - [手続き的マクロの定義](#手続き的マクロの定義)
  - [19.5.4 カスタムの derive マクロ](#1954-カスタムの-derive-マクロ)
    - [1. トレイトを含むクレートの作成](#1-トレイトを含むクレートの作成)
    - [2. カスタムの derive マクロの定義を含むクレートの作成](#2-カスタムの-derive-マクロの定義を含むクレートの作成)
    - [3. 必要なライブラリの導入](#3-必要なライブラリの導入)
    - [4. マクロの定義](#4-マクロの定義)
      - [4.1 ライブラリをコードに導入](#41-ライブラリをコードに導入)
      - [4.2 `syn` の利用](#42-syn-の利用)
      - [4.3 quote! を用いて Rust のソースコードを出力する](#43-quote-を用いて-rust-のソースコードを出力する)
    - [5. ライブラリクレートのビルド](#5-ライブラリクレートのビルド)
    - [6. 定義したカスタム derice マクロの利用](#6-定義したカスタム-derice-マクロの利用)

## 19.5.0 マクロの概要

TODO:

- マクロとは
  - コードそのものを書くコード
  - マクロを使用すると、それらは全て展開され、手で書いたよりも多くのコードを生成する

- マクロは以下のように分類される
  - `macro_rules!` を用いた宣言的マクロ
  - 手続き的マクロ三種
    - `#[derive(...)]` マクロ
    - 任意の要素に使えるカスタム属性を定義する属性風のマクロ
    - 関数風のマクロ

## 19.5.1 マクロと関数の違い

- マクロは可変長の引数を持つことができる
  - 一方、関数は決まった数の引数をもつ

- マクロは適当な型にトレイトを実装するコードを生成できる

- マクロの欠点：関数より複雑（読みにくく、わかりにくく、管理しづい）

- マクロは、呼び出す前に定義したりスコープに導入しなければならない
  - 一方、関数はどこでも定義できてどこでも呼び出せる

## 19.5.2 macro_rules! を用いた宣言的なマクロでメタプログラミング

- Rust における宣言的マクロは `macro_rules!` 構文を用いて定義される
- 具体例は、`vec!`, `println!` など

- マクロ、例によるマクロ、`macro_rules!` マクロなどとも呼ばれる
- このタイプのマクロは、`match` 式に似た特徴をもつ
  - すなわち、`macro_rules!` マクロは以下のような特徴を持つ：
    - `macro_rules!` マクロは一つ以上の **ルール** をもつ
      - 各ルールは「ソースコードの構造を表すパターン（**matcher**）」とそれに `=>` で紐づけられたコード（**transcriber**）からなる
    - マクロが呼び出されると matcher と「マクロに渡されたソースコード」とが比較される
    - ソースコードが適当な matcher にマッチしたら、そのマクロの呼び出し全体は対応する transcriber の内容で置換される

- マクロのパターン記法については [Rust リファレンス | Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html) を参考にすること

### マクロの定義の例：簡略化した `vec!` マクロ

- マクロを定義するには `macro_rules!` 構文を用いる
- ここでは、`vec!` マクロの定義を通じてマクロの定義について学ぶ

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

- 以下にこのコードの各パーツについて説明していく：

  - `#[macro_export]` 注釈は、マクロを定義するクレートがスコープに持ち込まれたなら無条件でこのマクロが利用可能になることを表す
    - 逆に、この注釈がないと、このマクロはスコープに導入され得ない

  - `macro_rules! マクロ名  { ...マクロの本体の記述... }` はマクロの `!` ぬきの名前とともに、マクロの定義をすることを宣言する

  - `vec!` 本体の構造は `match` 式と類似している：
    - `( $( $x:expr ),* )` はパターンを表し、
    - このパターンと、それに対応するコードのブロックが `=>` で結び付けられている
    - `vec!` 呼び出し時にそれに続くコードがこのアームのパターンに一致したら、対応するコードブロックが出力される
    - `vec!` の定義は、この一つのアームのみから構成されており、マッチする有効なパターンはひとつのみである

  - `( $( $x:expr ),* )` は matcher

    - `$x:expr` は任意の式にマッチして、その式に `$x` という名前をつけて describer 内で使用できるようにする

    - `$(...),*` は、 `*` の直前にあるものの0個以上の繰り返しにマッチすることを表す

      - 今回の場合は、`式,` の形のコードの繰り返しに一致する

      - `,` の部分は `=>`, `;` に置き換えることもできるし何も指定しないこともできる
        - 例えば、`$( $x:expr )*` のように書くと `(式) (式) ... (式)` のようなコードとマッチする
        - また、`$( $x:expr )=>*` のように書くと `(式)=>(式)=>...=>(式)` のようなコードとマッチする

      - `*` の部分は `+`, `?` にも置き換え可能
        - `+` は一個以上の繰り返しに一致する
        - `?` は直前にあるものが0個か1個ある場合にマッチする（オプショナルな項目を表せる）

  - matcher に続く `=> {...}` の部分は describer
    - 基本的に Rust のコードがそのまま書かれているが、`$(...)*` で囲われている部分だけ特殊
      - この部分は、matcher 内の `$(...),*` がマッチした回数分だけ繰り返される
      - また、matcher 内の`$(...),*` 内部の `$x:expr` に対応して、この繰り返しの内部では `$x` が使える
        - これらの `$x` はそれぞれマッチした式に置換される

- 結果的に生成されるコードは以下のようになる：

```rust
// vec![1, 2, 3] の展開結果
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

## 19.5.3 属性からコードを生成する手続き的マクロの

- もう一種類のマクロが、**手続き的マクロ**である
- 手続き的マクロはコードをインプットとして受け取って、そのコードに処理を施し、出力としてコードを生成する
- 手続き的マクロは以下の三種に分類されるが、いずれも似たような挙動を示す
  - カスタムの derive マクロ
  - 属性風マクロ
  - 関数風マクロ

### 手続き的マクロの定義

- 手続き的マクロの定義は、それ専用の特殊なクレート内に置かれる必要がある
- 手続き的マクロは以下のように定義される：
  - `#[some_attribute]` では、どのマクロを使用するかを指定する
    - `#[proc_macro_derive(X)]`: カスタム derive マクロ
    - `#[proc_macro_attribute]`: 属性風マクロ
    - `#[proc_macro]`: 関数風マクロ
  - `TokenStream` を入力として受け取り、`TokenStream` を出力として生成する関数として定義する
    - `TokenStream` は Rust の `proc_macro` クレートで定義されている
    - `TokenStream` はトークンの列を表す
    - つまり、手続き的マクロでは、マクロが操作するソースコードが入力 `TokenStream` になり、マクロが生成するコードが出力 `TokenStream` になる
生成

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

## 19.5.4 カスタムの derive マクロ

- 手続き的マクロの定義を具体例を通じて示す
- この例では、以下を目指す
  - `HelloMacro` というトレイトを定義する
  - このトレイトを適当な型に実装するために `#[derive(HelloMacro)]` で注釈すれば済むようにする
  - この注釈を付与して実装した `HelloMacro` 型の `hello_macro` 関連関数のデフォルト定義は、`"Hello, Macro! My name is <型の名前>"` と `println!` するようなモノとする

- このために以下の手順を踏む
  1. `HelloMacro` トレイトを含むクレートの作成
  2. カスタムの derive マクロを含むクレートを作成する
  3. 後者のクレートに必要なライブラリを導入
  4. マクロの定義
     1. ライブラリをコードに導入
     2. `syn` を使って、`TokenStream` 型の `input` をデータ構造に変換して、解釈・操作可能にする
     3. `quote!` を用いて `syn` で解釈可能にして得られた `DriveInput` 構造体を適当な `TokenStream` に変換する
  5. ライブラリクレートのビルド
  6. マクロの利用

### 1. トレイトを含むクレートの作成

- クレートの作成

  ```sh
  cargo new hello_macro --lib
  ```

- トレイトの定義

  **`hello_macro/src/lib.rs`**

  ```rust
  pub trait HelloMacro {
      fn hello_macro();
  }
  ```

  - この時点ではデフォルト定義はない
  - カスタムの derive マクロの中でデフォルト定義を記述していく

### 2. カスタムの derive マクロの定義を含むクレートの作成

- クレートの作成

  ```sh
  $ pwd
  (...略...)/hello_macro
  
  $ cargo new hello_macro_derive
  ```

  - このクレートは、1. で作成したクレートと密接な関係にあるのでここでは、1. のクレートのディレクトリ内で作成する

- さらに、`Cargo.toml` に以下の記述を追加して、このクレートが手続き的マクロクレートであることを宣言する

  **`Cargo.toml`**

  ```toml
  [lib]
  proc-macro = true
  ```

### 3. 必要なライブラリの導入

- `syn`, `quote` の導入

  ```sh
  cargo add syn quote
  ```

### 4. マクロの定義

#### 4.1 ライブラリをコードに導入

- まず、必要なライブラリをコードに導入し、マクロを定義する関数のひな型を作成する
  
  **`hello_macro/hello_macro_derive/src/lib.rs`**

  ```rust
  // #[proc_macro_derive(X)] などを利用できるようにする
  // proc_macro 自体は Rust に付随してくるので cargo add する必要はない
  extern crate proc_macro;

  // マクロの引数と返り値の型である TokenStream を使えるようにする
  use proc_macro::TokenStream;

  // TokenStream（Rust コード）を生成するためのテンプレート機能を提供する quote の導入
  use quote::quote;

  // TokenStream から Rust コードを構文解析して、人間が扱いやすいデータ構造に変換する syn の導入
  use syn;

  // #[proc_macro_derive(X)] を付けた関数が derive マクロになる
  //  --> つまり、#[derive(X)] できるようになる
  //  --> これにより引数 X で指定したトレイトを簡単に実装できるようになる
  // derive マクロの名前は、関数名に関係なく、引数 X の値で決定されることに注意
  // ここで定義する関数は、引数の型も出力の型も TokenStream のものとする
  #[proc_macro_derive(HelloMacro)]
  pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
      //...
  }
  ```

#### 4.2 `syn` の利用

- `syn` は `TokenStream` を人間が解釈・操作しやすい形式に変換する機能を提供する
- 使い方は以下の通り：
  
  **`hello_macro/hello_macro_derive/src/lib.rs`**

  ```rust
  extern crate proc_macro;

  use proc_macro::TokenStream;
  use quote::quote;
  use syn;

  #[proc_macro_derive(HelloMacro)]
  pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
      // syn を用いて、TokenStream 型の input をデータ構造に変換して、解釈・操作可能にする
      // 出力は DeriveInput 構造体（パースされた Rust コードを表す）
      // パースが失敗したときはパニックさせる（この関数の返り値の型は Result 型ではないので。。）
      let ast = syn::parse(input).unwrap();
      
      // トレイトの実装内容（後述）
      impl_hello_macro(&ast);
  }
  ```

  - このコードは `impl_hello_macro` を実装しないとコンパイルできないことに注意
  - また、実際にコードでは、`unwrap` ではなく `expect` や `panic!` を用いて、ユーザーに何が間違っているのかを具体的なエラーメッセージを通じて伝えるのが好ましい

- なお、例えば、`struct Pancakes;` という文字列をパースすることで得られる `DeriveInput` 構造体は以下のような形になる：

    ```rust
    DeriveInput {
        // --snip--

        ident: Ident {
            ident: "Pancakes",
            span: #0 bytes(95..103)
        },
        data: Struct(
            DataStruct {
                struct_token: Struct,
                fields: Unit,
                semi_token: Some(
                    Semi
                )
            }
        )
    }
    ```

- 詳細は [syn documentation for DeriveInput](https://docs.rs/syn/1.0/syn/struct.DeriveInput.html) を参考にせよ

#### 4.3 quote! を用いて Rust のソースコードを出力する

- 最後に `quote!` を使用して Ruat コードを生成する処理を記述してマクロの定義は完了する：

  ```rust
  extern crate proc_macro;

  use proc_macro::TokenStream;
  use quote::quote;
  use syn;

  #[proc_macro_derive(HelloMacro)]
  pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
      let ast = syn::parse(input).unwrap();

      // トレイトを実装する処理
      impl_hello_macro(&ast)
  }

  fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
      // 注釈された型の識別子を取得
      let name = &ast.ident;

      // quote! マクロを用いて Rust コードを生成
      // quote! マクロ内で #hoge と書くと、その部分は hoge という変数の値と置き換えられる
      // stringify! マクロは。1 + 2 などのような Rust の式を取り、コンパイル時に "1 + 2" のような文字列リテラルにその式を変換する
      let gen = quote! {
          impl HelloMacro for #name {
              fn hello_macro() {
                  println!("Hello, Macro! My name is {}", stringify!(#name));
              }
          }
      };

      // TokenStream に変換しなおす
      gen.into()
  }
  ```

  - quote の詳細は [the quote crate’s docs](https://docs.rs/quote) を参考にせよ

### 5. ライブラリクレートのビルド

```sh
$ pwd
...(略).../hello_macro

$ cargo build

$ cd hello_macro_derive

$ pwd
...(略).../hello_macro/hello_macro_derive

$ cargo build
```

### 6. 定義したカスタム derice マクロの利用

- 新しいバイナリプロジェクトの作成

  ```sh
  cargo new pancakes
  ```

- `Cargo.toml` で依存関係を定義

  **`pancakes/Cargo.toml`** に以下を追記

  ```toml
  [dependencies]
  hello_macro = { path = "../hello_macro" }
  hello_macro_derive = { path = "../hello_macro/hello_macro"_derive }
  ```

- マクロの利用：
  
  **`pancakes/src/main.rs`**

  ```rust
  // 両方のクレートから HelloMacro を導入
  use hello_macro::HelloMacro;
  use hello_macro_derive::HelloMacro;
  
  // derive で関連関数を実装
  #[derive(HelloMacro)]
  struct Pancakes;

  fn main() {
      // 実装した関連関数を呼び出し
      Pancakes::hello_macro();
  }
  ```

