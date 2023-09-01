# １９章 ー１節：高度な機能ーUnsafe Rust

## 目次

- [１９章 ー１節：高度な機能ーUnsafe Rust](#１９章-ー１節高度な機能ーunsafe-rust)
  - [目次](#目次)
  - [19.1 Unsafe Rust の概要](#191-unsafe-rust-の概要)
  - [生ポインタについて](#生ポインタについて)
    - [生ポインタとは](#生ポインタとは)
    - [生ポインタの生成](#生ポインタの生成)
    - [生ポインタの参照外し](#生ポインタの参照外し)
  - [unsafe な関数やメソッドの利用](#unsafe-な関数やメソッドの利用)
    - [unsafe ブロックを含む実装を安全な関数として抽象化する](#unsafe-ブロックを含む実装を安全な関数として抽象化する)
      - [unsafe コードを抽象化する関数の実装の例：`split_at_mut`](#unsafe-コードを抽象化する関数の実装の例split_at_mut)
    - [`extern` 関数を使用して、外部のコードを呼び出す](#extern-関数を使用して外部のコードを呼び出す)
    - [他の言語から Rust の関数を呼び出す](#他の言語から-rust-の関数を呼び出す)
  - [可変で静的な変数にアクセスしたり変更する](#可変で静的な変数にアクセスしたり変更する)
  - [unsafe なトレイトの実装](#unsafe-なトレイトの実装)

## 19.1 Unsafe Rust の概要

- Unsafe Rust: Rust の保証の一部を抜けてその保証に関してプログラマ側で責任を負う方法

- `unsafe{...}` で囲ったブロック内では、通常の Rust では許されない以下の機能が使えるようになる：
  - 生ポインタの参照外し
    - 「借用規則に反するが安全である」とプログラマが判断できるコードを書く方法
  - unsafe な関数やメソッドの呼び出し
    - unsafe な関数を利用する
    - `extern "ABI 名" {...}` で FFI を利用する
    - `pub extern "ABI 名" fn 関数名(...) {...}` で他の言語に Rust 関数を呼び出すインターフェースを提供する
  - 可変で静的な変数にアクセスしたり変更を加える
    - `static` キーワードを用いて静的変数（他の言語のグローバル変数のようなもの）を定義できる
    - 静的変数は `const` で定義される定数とは異なり、`mut` キーワードで可変にできる
    - 可変な静的変数の読みとり・書き込みはどちらも unsafe である
  - unsafe なトレイトの実装

## 生ポインタについて

### 生ポインタとは

- unsafe Rust には**生ポインタ**という二つの新しい型がある（ここで登場する `*` は参照外し演算子ではないことに注意）：
  - `*const T`: 不変な生ポインタ (参照外し後に直接ポインタに代入できない)
  - `*mut T`: 可変な生ポインタ

- 生ポインタでは
  - Rust の借用規則が無視される
    - 不変なポインタと可変なポインタが同時に存在できる
    - 複数の可変なポインタの存在が許される
  - 有効なメモリを指していると保証されない
  - null の可能性がある
  - 自動的な片付けは実装されていない

### 生ポインタの生成

- 生ポインタの生成自体は safe コードでも可能
  - だが、unsafe ブロックの外では参照外しできない

- 生ポインタの生成の例１：

    `as` を使用して生ポインタ型にキャスト（有効な参照から生成しているので、これらの生ポインタも有効であることがわかる）

  ```rust
  let mut num = 5;

  // 不変ポインタと可変ポインタの共存
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;
  ```

- 生ポインタ生成の例２：

    メモリの任意の箇所を指す生ポインタ（そのアドレスにデータはあるかもしれないし、ないかもしれない。コンパイラがコードを最適化してメモリアクセスがなくなったり、プログラムがセグメンテーションフォールトでエラーを起こす可能性もある）
  
  ```rust
  let address = 0x012345usize;
  let r = address as *const i32;
  ```

### 生ポインタの参照外し

- 生ポインタの参照外し（生ポインタの指す値へのアクセス）は unsafe ブロック内でしか実行できない

- 生ポインタの参照外しの例：

  ```rust
  let mut num = 5;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  // 生ポインタが指す値を読むために unsafe ブロックを作成
  unsafe {
    // 通常の参照外しと同じように * で参照外し
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
  }
  ```

## unsafe な関数やメソッドの利用

- `unsafe` 関数の定義（定義だけなら safe コード）
- `unsafe` 関数の利用は `unsafe` ブロック内と `unsafe` 関数内に限られる
- `unsafe` 関数を利用する際は、**その関数のドキュメンテーションを読み、関数の契約を守っている**とプログラマ側で責任を負う必要がある

  ```rust
  // unsafe 関数の定義
  unsafe fn dangerous() {
    // ...
  }

  // unsafe 関数の利用
  unsafe {
    dangerous();
  }
  ```

### unsafe ブロックを含む実装を安全な関数として抽象化する

- `unsafe` ブロックを含む関数は `unsafe` 関数でなければならない**わけではない**
- safe 関数内で `unsafe` ブロックを使うのは一般的な抽象化

#### unsafe コードを抽象化する関数の実装の例：`split_at_mut`

- 標準ライブラリ内に存在する可変なスライスに定義されたメソッド `spllit_at_mut` の実装を参考例に考える
- この関数の動作は以下の通り：

  ```rust
  let mut v = vec![1, 2, 3, 4, 5, 6];

  let r = &mut v[..];

  let (a, b) = r.split_at_mut(3);

  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);
  ```

- この関数の実装は以下のようになると思われる（が、実際にはこのままでは問題がある）：
  - Rust の借用チェッカーは与えたスライスの（互いに重複しない）異なる部分を借用していることを理解できないので、同じデータに対する二つ以上の可変参照があると勘違いしてコンパイルエラーを起こす

  ```rust
  fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
      let len = slice.len();

      assert!(mid <= len);

      (&mut slice[..mid],
      &mut slice[mid..])
  }
  ```

- コンパイラによる「勘違い」を回避するために `unsafe` を導入して、この関数の実装をする方法は以下の通り：
  - `slice::from_raw_parts_mut` 関数は、生ポインタと長さを取って、スライスを生成する関数
  - `ptr.offset(num as isize)` は `ptr` から `num` 個分だけ後ろのポインタを返すメソッド
  - これら二つの関数・メソッドはどちらも生ポインタの指す値を返すので unsafe である

  ```rust
  fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    // スライスの生ポインタを取得
    let ptr = slice.as_mut_ptr();

    // 無効なアドレスへのアクセスを防止するために mid > len の場合は unsafe ブロックに入る前にパニックを起こす
    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
  }
  ```

- この関数は、内部で unsafe ブロックを使用しているが、プログラマ側で安全な関数だと責任を負っている
  - &rarr; safe Rust で使用可能な関数

- **悪い** unsafe の使用例：

  ```rust
  use std::slice;

  // アドレスを直接指定して生ポインタを取得
  // (値が有効だと全く保証されていない...恐ろしい...）
  let address = 0x012345usize;
  let r = address as *mut i32;

  // 先の生ポインタから連続で 10000 個のアドレスにわたるスライスを生成
  // (10000 個のアドレスのどこもかしこも有効な値が入っている保証がない...恐ろしすぎる!!!)
  let slice = unsafe {
    slice::from_raw_parts_mut(r, 10000)
  };
  ```

### `extern` 関数を使用して、外部のコードを呼び出す

- `extern` キーワードは FFI (Foreign Function Interface: 外部関数インターフェース)の生成・利用を容易にする
  - FFI: ある言語に関数を定義させて、異なる言語からその関数を呼び出せるようにする方法のこと

- FFI で導入した関数を Rust 内で使用する場合 unsafe となる

- 例：C 言語の標準ライブラリから `abs` 関数を統合する

  - ABI (application binary interface): アプリケーション・バイナリ・インターフェイス. 関数の呼び出し方法をアセンブリレベルで定義する.
    - 代表例は `"C"` ABI (C 言語の ABI に従う)

  ```rust
  // まずどの ABI(application binary interface: アプリケーション・バイナリ・インターフェイス) から FFI を利用するかを extern で指定
  extern "C" {
    // 呼び出した関数の名前とシグネチャを列挙
    fn abs(input: i32) -> i32;
  }

  fn main() {
      unsafe {
          println!("-3 の絶対値は C によると: {}", abs(-3));
      }
  }
  ```

### 他の言語から Rust の関数を呼び出す

- `pub extern "ABI名" fn ...` で他の言語に Rust の関数を呼び出させるためのインターフェースを生成することができる
- ここで、`#[no_mangle]` 注釈を追加して、Rust コンパイラに関数名をマングルしないように指示する必要がある
  - マングル：コンパイラがコンパイルの過程で関数名を（人間にとって読みにくい）異なる名前に変更すること
- これは `unsafe` ではない

- 例：

  ```rust
  #[no_mangle]
  pub extern "C" fn call_from_c() {
      println!("C から Rust 関数を呼び出しました！");
  }
  ```

## 可変で静的な変数にアクセスしたり変更する

- グローバル変数： Rust では static 変数（静的変数）と呼ぶ
  - 定義は定数と似たり寄ったり（`const` を `static` にするだけだと思えばよい）

- 静的変数と定数の違い：
  - 静的変数の値は固定されたメモリアドレスになる
    - 静的変数の値を使用すると常に同じデータにアクセスする
    - 定数は使用のたびにデータを複製可能
  - 静的変数は**可変にもなりうる**

- 不変な静的変数の使用例：

  ```rust
  static HELLO_WORLD: &str = "Hello, world!";

  fn main() {
    println!("name is: {}, HELLO_WORLD");
  }
  ```

- 可変な静的変数にアクセスしたり変更を加えることは unsafe
  - なぜならば、2 つのスレッドが同じ可変なグローバル変数にアクセスしていたら、データ競合を起こすことがあるから

  - 例：

    ```rust
    // 通常の変数と同じく mut キーワードで可変にできる
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        // 可変な静的変数に変更を加えるのは unsafe
        unsafe {
            COUNTER += inc;
        }
    }

    fn main() {
        add_to_count(3);

        // 可変な静的変数は読むだけでも unsafe
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
    ```

## unsafe なトレイトの実装

- 1つ以上のメソッドにコンパイラが確かめられないなんらかの不変性があると、トレイトは unsafe になる
- そのようなトレイトには `unsafe` キーワードを付与してトレイトが unsafe であることを明示する
- また、トレイトを何らかの構造体や Enum に実装する際にも `unsafe` キーワードを付与する

```rust
unsafe trait Foo {
    // メソッドのシグネチャ
}

unsafe impl Foo for i32 {
    // メソッドの実装
}

fn main() {}
```
