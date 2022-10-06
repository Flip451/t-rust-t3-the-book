# 15 章：スマートポインタ

## 目次

- [15 章：スマートポインタ](#15-章スマートポインタ)
  - [目次](#目次)
  - [15.0 概要](#150-概要)
    - [各参照を選択する理由](#各参照を選択する理由)
    - [その他の重要な概念](#その他の重要な概念)
  - [15.1 `Box<T>`: ヒープのデータを指す](#151-boxt-ヒープのデータを指す)
    - [`Box<T>` を使ってヒープにデータを格納する](#boxt-を使ってヒープにデータを格納する)
    - [ボックスで再帰的な型を可能にする](#ボックスで再帰的な型を可能にする)
      - [再帰的なデータ型の例：コンスリスト](#再帰的なデータ型の例コンスリスト)
        - [コンスリストとは](#コンスリストとは)
        - [問題のある定義](#問題のある定義)
        - [`Box<T>` を用いて既知のサイズの再帰的な型を得る](#boxt-を用いて既知のサイズの再帰的な型を得る)
  - [15.2 `Deref` トレイトでスマートポインタを普通の参照のように扱う](#152-deref-トレイトでスマートポインタを普通の参照のように扱う)
    - [通常の参照に参照外し演算子を適用して値までポインタを追いかける](#通常の参照に参照外し演算子を適用して値までポインタを追いかける)
    - [`Box<T>` に参照外し演算子を適用して値までポインタを追いかける](#boxt-に参照外し演算子を適用して値までポインタを追いかける)
    - [独自のスマートポインタを定義する（`Box<T>` に似たスマートポインタの定義）](#独自のスマートポインタを定義するboxt-に似たスマートポインタの定義)
    - [`Deref` トレイトを実装して型を参照のように扱う](#deref-トレイトを実装して型を参照のように扱う)
    - [関数やメソッドで暗黙的な参照外し型強制](#関数やメソッドで暗黙的な参照外し型強制)
    - [参照外し型強制を可変性と組合わせる](#参照外し型強制を可変性と組合わせる)
  - [15.3 `Drop` トレイトで片付け時にコードを走らせる](#153-drop-トレイトで片付け時にコードを走らせる)
    - [`std::mem::drop` で早期に値をドロップする](#stdmemdrop-で早期に値をドロップする)
  - [15.4 `Rc<T>` は、参照カウント方式のスマートポインタ](#154-rct-は参照カウント方式のスマートポインタ)
    - [`Rc<T>` の使い方の例](#rct-の使い方の例)
      - [参照カウントが変化することを確認する（`Rc::strong::count` 関数）](#参照カウントが変化することを確認するrcstrongcount-関数)
  - [15.5 `RefCell<T>` と内部可変性パターン](#155-refcellt-と内部可変性パターン)
    - [`RefCell<T>` で実行時に借用規則を強制する](#refcellt-で実行時に借用規則を強制する)
      - [内部可変性のユースケース：モックオブジェクト（`RefCell<T>` の使用例）](#内部可変性のユースケースモックオブジェクトrefcellt-の使用例)
    - [`Rc<T>` と `RefCell<T>` を組み合わせることで可変なデータに複数の所有者を持たせる](#rct-と-refcellt-を組み合わせることで可変なデータに複数の所有者を持たせる)
  - [15.6 循環参照は、メモリをリークすることもある](#156-循環参照はメモリをリークすることもある)
    - [循環参照の例](#循環参照の例)
    - [循環参照を回避する: `Rc<T>` を `Weak<T>` に変換する](#循環参照を回避する-rct-を-weakt-に変換する)
      - [要するに](#要するに)
      - [`Weak<T>` の使用例：木データ構造を作る（子ノードのある Node）](#weakt-の使用例木データ構造を作る子ノードのある-node)
        - [子供から親への参照を追加する](#子供から親への参照を追加する)
        - [`strong_count` と `weak_count` への変更を可視化する](#strong_count-と-weak_count-への変更を可視化する)

## 15.0 概要

- スマートポインタは、**ポインタのように振る舞う**だけでなく、**追加のメタデータと能力がある**データ構造
  - スマートポインタは、参照とは異なり、**指しているデータを所有**する
  - 普通、構造体を使用して実装されている
  - スマートポインタは通常の構造体と異なり、**`Deref` と `Drop` トレイトを実装**している
    - `Deref`：参照外し演算子 `*` を作用させたときの動作を定義する
    - `Drop`：値がスコープを抜けるときの動作を定義する
  - 既出の例：**`String`**, **`Vec<T>`**
  - この章で取り扱う新しい例：
    - **`Box<T>`**: ヒープに値を確保する
    - **`Rc<T>`**: 複数の所有権を可能にする参照カウント型
    - **`Ref<T>`, `RefMut<T>`**: `RefCell<T>` を通してアクセスされ、コンパイル時ではなく実行時に借用規則を強制する型
      - **内部可変性パターン**: 不変な型が、内部の値を変更するための API を公開する（`RefCell<T>` の使用例）
      - **循環参照**：Rust でもメモリリークの可能性がある（`Rc<T>` と `RefCell<T>` を組合わせると循環参照が発生しうる）
  - その他、この章では扱わない例として、`Cell<T>` や `Mutex<T>` が挙げられる（標準ライブラリのドキュメンテー
ションをチェックせよ）

### 各参照を選択する理由

- `Rc<T>` は、同じデータに**複数の所有者**を持たせてくれる
  - 一方、`Box<T>` と `RefCell<T>` は**単独の所有者**
- `Box<T>` では、不変借用も可変借用も**コンパイル時に精査**できる
  - `Rc<T>` では**不変借用のみがコンパイル時に精査**できる
  - `RefCell<T>` では、不変借用も可変借用も**実行時に精査**される
- `RefCell<T>` は**実行時に精査される可変借用**を許可する（不変借用も作成可能）
  - `RefCell<T>` が不変でも、`RefCell<T>` 内の値を可変化できる

### その他の重要な概念

- 参照外し型強制
- 

## 15.1 `Box<T>`: ヒープのデータを指す

- `Box<T>` により、スタックではなく**ヒープにデータを格納**できる
  - スタックにはヒープデータへのポインタが作成される
- 使用する場面：
  - **コンパイル時にはサイズを知ることができない型**があり、正確なサイズを要求する文脈でその型の値を使用する時
  - **多くのデータがあり、その所有権を移したい**が、その際にデータがコピーされないようにしたい時（パフォーマンスが向上する）
  - 値を所有する必要があり、特定の型であることではなく、特定のトレイトを実装する型であることのみ気にかけている時（**トレイトオブジェクト**）

### `Box<T>` を使ってヒープにデータを格納する

- `Box::new` メソッドでボックスを生成できる
- ボックスは、スコープを抜けるとき、メモリから解放される
  - メモリの解放は、以下の二つに対して起きる：
    - （スタックに格納されている）ボックス
    - （ヒープに格納されている）データ

  - 例：ボックスを使用してヒープに `i32` の値を格納する
    - 値 `5` はヒープに確保される
    - `main` 関数の終わりで `b` は解放される

    ```rust
    fn main() {
        let b = Box::new(5);
        println!("b = {}", b);  // b = 5
    }
    ```

### ボックスで再帰的な型を可能にする

- **再帰的な型** はコンパイル時にサイズがわからないので、ボックスを利用して実装する必要がある

#### 再帰的なデータ型の例：コンスリスト

##### コンスリストとは

- コンスリストの各要素は、二つの要素を含む：
  - 現在の要素の値
  - 次の要素
- リストの最後の要素は `Nil` と呼ばれる値だけを含む（`null`, `nil` とは異なるので注意）

##### 問題のある定義

- コンスリストの `enum` 定義（この時点では List 型のサイズが不明なのでコンパイル不可）：

  ```rust
  enum List {
    Cons(i32, List),
    Nil
  }
  ```

  - 使用例：

    ```rust
    use List::{Cons, Nil};

    fn main() {
        let list = Cons(1, Cons(2, Cons(3, Nil)));
    }
    ```

##### `Box<T>` を用いて既知のサイズの再帰的な型を得る

- **間接参照**（値の代わりに値へのポインタを格納する）を利用して再帰的な型を実現する
- 使用例：
  - `List` 列挙子は１つの `i32` のサイズに加えてボックスのポインタデータを格納する領域を必要とする（がそれ以上は必要としない）
    - --> コンパイラは `List` 値を格納するのに必要なサイズを計算できる

  ```rust
  enum List {
    Cons(i32, Box<List>),
    Nil
  }

  use List::{Cons, Nil};

  fn main() {
    let list = Cons(1, 
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil)
            ))
        ))
    );
  }
  ```

## 15.2 `Deref` トレイトでスマートポインタを普通の参照のように扱う

- `Deref` トレイトを実装することで、参照外し演算子の `*` の振る舞いをカスタマイズできる
- **参照外し型強制**で参照やスマートポインタをうまく使うことができる

### 通常の参照に参照外し演算子を適用して値までポインタを追いかける

- 例：i32 値への参照を生成してから参照外し演算子を使ってデータまで参照を辿る

  ```rust
  fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5):
  }
  ```

### `Box<T>` に参照外し演算子を適用して値までポインタを追いかける

- 例：i32 値への参照を生成してから参照外し演算子を使ってデータまで参照を辿る

  ```rust
  fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5):
  }
  ```

### 独自のスマートポインタを定義する（`Box<T>` に似たスマートポインタの定義）

- `Box<T>` 型を模倣するために一要素のタプル構造を定義する:

  ```rust
  struct MyBox<T> (T);

  impl<T> MyBox<T> {
      fn new(x: T) -> MyBox<T> {
          MyBox<T>(x)
      }
  }
  ```

- `MyBox<T>` の参照外しの方法を指定していないので、以下のコードはコンパイルできない：

  ```rust
  fn main() {
      let x = 5;
      let y = MyBox::new(x);

      assert_eq!(x, 5);
      assert_eq!(*y, 5);
  }
  ```

### `Deref` トレイトを実装して型を参照のように扱う

- **`*` 演算子で参照外しできるようにするには、`Deref` トレイトを実装する**必要がある
  - `Deref` トレイトは `deref` という 1 つのメソッドの実装を要求する
    - `deref` は `self` を借用し、内部のデータへの参照を返すメソッド
  - また、`type Target = Hoge;` で `Deref` トレイトが使用する関連型を定義する（参照外しの結果得られる値の型を定義する？）
  - 例：

  ```rust
  use std::ops::Deref;

  struct MyBox<T> (T);

  impl<T> Deref for MyBox<T> {
      type target = T;

      fn deref(&self) -> &T {
          &self.0
      }
  }
  ```

- `Deref` トレイトを実装している型の変数に参照外しを行うと Rust は水面下で **`+(hoge.deref())`** のようなコードを実行する
  - （Rust が **`*` 演算子を `deref` メソッドの呼び出しと普通の参照外しへと置き換え**てくれる）
  - （普通の参照か `Deref` を実装した型であるかどうかに関わらず、等しく機能するコードを書ける）
- `*` 演算子が `deref` メソッドの呼び出しと `*` 演算子の呼び出しに置き換えられるのは、コード内で `*` を打つ毎にただ 1 回だけ
  - （`*` 演算子の置き換えは無限に繰り返されない）

### 関数やメソッドで暗黙的な参照外し型強制

- **参照外し型強制**：`Deref` を実装する型への参照が自動で型変換される
- 例：`Deref` トレイトを実装している `MyBox` 型のインスタンスへの参照を `&str` 型の引数を期待する関数に渡すと型強制が行われる：
  - ここでは、`MyBox<T>` 型に実装された `deref` メソッドによって `&MyBox<String>` が `&String` に変換され、`String` 型に実装された `deref` メソッドによって、`&String` 型が `&str` に変換される

  ```rust
  // hello 関数は &str 型の引数を期待する
  fn hello(name: &str) {
      println!("Hello, {}!", name);
  }

  fn main() {
      let m = MyBox::new(String::from("Rust"));
      hello(&m);  // &str を期待する関数に &MyBox<String> を渡すと参照外し型強制される
      
      // このような参照外し型強制が Rust の機能として実装されていなければ、以下のようなコードを書く必要がある（この機能によりコードがかなり簡素化されている）：
      // hello(&(*m)[..]);
  }
  ```

### 参照外し型強制を可変性と組合わせる

- `DerefMut` トレイトを実装することで、可変参照に `*` 演算子を作用させたときに可変参照を得る動作を定義できる
  - （`Deref` トレイトを実装することで、不変参照に `*` 演算子を作用させたときに参照を定義できることに対応）
- コンパイラは以下の三つの場合に参照外し型強制を実行する：
  1. `T: Deref<Target=U>` の時、`&T` から `&U`
  2. `T: DerefMut<Target=U>` の時、`&mut T` から `&mut U`
  3. `T: Deref<Target=U>` の時、`&mut T` から `&U`
  
  > 1. `&T` があり、`T` が何らかの型 `U` への `Deref` を実装しているなら、透過的に `&U` を得られる
  >
  > 2. `&mut T` があり、`T` が何らかの型 `U` への `DerefMut` を実装しているなら、透過的に `&mut U` を得られる
  >
  > 3. `&mut T` があり、`T` が何らかの型 `U` への `Deref` を実装しているなら、透過的に `&U` を得られる
  >
  > ※不変参照を参照外し型強制で可変参照に変えることはできない

## 15.3 `Drop` トレイトで片付け時にコードを走らせる

- どんな型に対しても `Drop` トレイトの実装を提供できる
  - スコープを抜けそうになった時に起こることをカスタマイズできる
- `Drop` トレイトを実装するには、`self` への可変参照を取る `drop` という 1 つのメソッドを実装する必要がある
  - `drop` 関数には、自分の型のインスタンスがスコープを抜ける時に走らせたいロジックを実装する
- 例：

  ```rust
  struct CustomSmartPointer {
      data: String,
  }

  impl Drop for CustomSmartPointer {
      fn drop(&mut self) {
          println!("Dropping CustomSmartPointer with data `{}`!", self.data);
      }
  }

  fn main() {
      let c = CustomSmartPointer {
          data: String::from("my stuff"),
      };
      let d = CustomSmartPointer {
          data: String::from("other stuff"),
      };
      println!("CustomSmartPointers created.");
  } // ここで "Dropping CustomSmartPointer with data `other stuff`!" と "Dropping CustomSmartPointer with data `my stuff`!" が標準出力に表示される
  // 変数は、生成されたのと逆の順序でドロップされるので、`d` は `c` より先にドロップされる
  ```

### `std::mem::drop` で早期に値をドロップする

- Rust は、`Drop` トレイトの `drop` メソッドを手動で呼ばせてくれない
- スコープが終わる前に値を強制的にドロップさせたいなら、代わりに標準ライブラリが提供する **`std::mem::drop` 関数を呼ぶ**
  - （この関数は初期化処理に含まれている）
- 例：

  ```rust
  fn main() {
      let c = CustomSmartPointer { data: String::from("some data") };
      println!("CustomSmartPointer created.");
      drop(c);
      println!("CustomSmartPointer dropped before the end of main.");
  }
  ```

  - このコードを実行すると以下のように出力される（`Drop` トレイトで定義した `drop` メソッドが `drop(c);` の部分で実行される）：

    ```rust
    CustomSmartPointer created.
    Dropping CustomSmartPointer with data `some data`!
    CustomSmartPointer dropped before the end of main.
    ```

## 15.4 `Rc<T>` は、参照カウント方式のスマートポインタ

- 単独の値が複数の所有者を持つ場合もある
  - 例：グラフデータ構造では、複数の辺が同じノードを指す可能性がある。概念的にそのノードはそれを指す全ての辺に所有されるので、指す辺がなくならない限り、ノードは片付けられるべきではない。

- `Rc<T>` という型（reference counting（参照カウント）の略）によって、複数の所有権が可能になる
- `Rc<T>` 型は、値への参照の数を追跡する
  - これにより値がまだ使用中かどうか決定される
  - 値への参照が 0 なら、どの参照も無効にすることなく、値は片付けられる
- ヒープにプログラムの複数箇所で読む何らかのデータを確保したいが、コンパイル時にはどの部分が最後にデータを使用し終わるか決定できない時に `Rc<T>` 型を使用する
- ※`Rc<T>` は、シングルスレッドの筋書きで使用するためだけのもの

### `Rc<T>` の使い方の例

- `Rc<T>` の使い方：
  - `use` 文を追加して `Rc<T>` をスコープに導入
  - `let hoge = Rc::new(fuga)` で `fuga` への参照カウンタを作成し、
  - `Rc::clone(&hoge)` で `hoge` のクローンを作成すると参照カウンタのカウンタ値が＋１される

- 例：`5` と `10` を含むコンスリスト `a` を作る。さらに、`3` で始まる `b` と `4` で始まる `c` のふたつのコンスリストを作る。 `b` と `c` のどちらも `a` リストに接続する
  - （`b`, `c` のコンスリストはどちらも同じコンスリスト `a` を共有する）

  ![コンスリストの共有](https://doc.rust-lang.org/book/img/trpl15-03.svg)

- 以下のような単純な実装はコンパイルエラーを吐く：

  ```rust
  enum List {
      Cons(i32, Box<List>),
      Nil,
  }

  use crate::List::{Cons, Nil};

  fn main() {
      let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
      let b = Cons(3, Box::new(a));  // コンスリスト a のヒープデータの所有権は,この行で b にムーブ済みなので
      let c = Cons(4, Box::new(a));  // ここではもう a を呼び出せない
  }
  ```

- そこで、`Box<T>` の代わりに `Rc<T>` を使用することでコンスリストの各要素が複数の所有者を持てるように変更する：
  - 新しい所有者を生み出す際には、`Rc::clone` を実行して値をクローンする（これを実行する度に、参照カウンタの値が１増える）
  - 初期化処理に含まれていないので、use 文を追加して `Rc<T>` をスコープに導入する

  ```rust
  enum List {
      Cons(i32, Rc<List>),
      Nil,
  }

  use crate::List::{Cons, Nil};
  use std::rc::Rc;

  fn main() {
      let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
      let b = Cons(3, Rc::clone(&a));
      let c = Cons(4, Rc::clone(&a));
  }
  ```

#### 参照カウントが変化することを確認する（`Rc::strong::count` 関数）

- `Rc::strong::count` 関数で参照カウントのカウント値を取得できる
- 例：

  ```rust
  enum List {
      Cons(i32, Rc<List>),
      Nil,
  }

  use crate::List::{Cons, Nil};
  use std::rc::Rc;

  fn main() {
      let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
      println!("count after creating a = {}", Rc::strong_count(&a)); // count after creating a = 1

      let b = Cons(3, Rc::clone(&a));
      println!("count after creating b = {}", Rc::strong_count(&a)); // count after creating b = 2
      {
          let c = Cons(4, Rc::clone(&a));
          println!("count after creating c = {}", Rc::strong_count(&a)); // count after creating c = 3

      }
      println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // count after c goes out of scope = 2

  }
  ```

## 15.5 `RefCell<T>` と内部可変性パターン

- **内部可変性**：そのデータへの**不変参照がある時でさえもデータを可変化できる**デザインパターン
  - （`unsafe` を用いて実装されている）

### `RefCell<T>` で実行時に借用規則を強制する

- `RefCell<T>` 型は、保持するデータに対して**単独の所有権**を表す
- 以下の借用規則 a, b の取り扱いが通常の参照と異なる：
  - 通常の参照 `&hoge` と `Box<T>` は以下のようなルールが**コンパイル時に**強制される（破ると**コンパイルエラー**になる）
    1. 参照は、一つの可変参照か複数の不変参照
    2. 参照は常に有効でなければならない
  - 一方、`RefCell<T>` では上記の借用規則は**実行時に**強制される（破ると**実行時にパニックを起こす**）
- `RefCell<T>` インスタンスに対して以下のメソッドを呼び出すことでスマートポインタを生成できる：
  - `borrow` メソッド：`Ref<T>` を返す（≈ 不変参照）
  - `borrow_mut` メソッド：`RefMut<T>` を返す（≈ 可変参照）

> - cf. 通常の参照：
>   - `&hoge`：不変参照
>   - `&mut hoge`：可変参照

- どちらの参照も `Deref` トレイトを実装しているので、普通の参照のように扱える
- `RefCell<T>` は `Ref<T>` 型あるいは `RefMut<T>` 型のスマートポインタの数を数える：
  - `borrow` メソッド（`borrow_mut` メソッド）を呼び出すたびに不変参照（可変参照）のカウント値を増やす
  - `Ref<T>` （`RefMut<T>`）の値がスコープを抜けるたびに不変参照（可変参照）のカウント値を減らす
  - 実行時に、借用規則（参照は、一つの可変参照か複数の不変参照）が破られるとパニックを起こす
    - 例：可変参照を複数生成するとパニックが起こる（コンパイルできる）

      ```rust
          impl Messenger for MockMessenger {
              fn send(&self, message: &str) {
                  let mut one_borrow = self.sent_messages.borrow_mut();
                  let mut two_borrow = self.sent_messages.borrow_mut();

                  one_borrow.push(String::from(message));
                  two_borrow.push(String::from(message));
              }
          }
      ```

- コードが借用規則に従っているとプログラマは確証を得ているが、コンパイラがそれを理解し保証することができない時に `RefCell<T>` 型は有用
- ※ `RefCell<T>` もシングルスレッドの筋書きで使用するためのもの（マルチスレッドの文脈で使ってみようとすると、コンパイルエラーを出す）

<!-- 
### 内部可変性：不変値への可変参照（`RefCell<T>`）

- 借用規則に従う限り、不変値は可変借用できない：
  - 例：以下はコンパイルできない：

    ```rust
    fn main() {
        let x = 5;
        let y = &mut x; // 不変値の可変参照は生成できない
    }
    ```

- しかし、**内部可変性が有効な場面もある**
  - -> `RefCell<T>` が有用 
-->

#### 内部可変性のユースケース：モックオブジェクト（`RefCell<T>` の使用例）

- **テストダブル**：ソフトウェアテストにおいて、テスト対象が依存しているコンポーネントを置き換える代用品のこと
- **モックオブジェクト**：テスト中に起きることを記録するテストダブルの特定の型

- 考えるテストの筋書き：

  > - 値を最大値に対して追跡し、現在値がどれくらい最大値に近いかに基づいてメッセージを送信するライブラリを作成
  > - ライブラリのコードは以下のような感じ：
  >
  >   ```rust
  >   pub trait Messenger {
  >       fn send(&self, msg: &str);
  >   }
  > 
  >   pub struct LimitTracker<'a, T: Messenger> {
  >       messenger: &'a T,
  >       value: usize,
  >       max: usize,
  >   }
  > 
  >   impl<'a, T> LimitTracker<'a, T>
  >   where
  >       T: Messenger,
  >   {
  >       pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
  >           LimitTracker {
  >               messenger,
  >               value: 0,
  >               max,
  >           }
  >       }
  > 
  >       pub fn set_value(&mut self, value: usize) {
  >           self.value = value;
  > 
  >           let percentage_of_max = self.value as f64 / self.max as f64;
  > 
  >           if percentage_of_max >= 1.0 {
  >               self.messenger.send("Error: You are over your quota!");
  >           } else if percentage_of_max >= 0.9 {
  >               self.messenger
  >                   .send("Urgent warning: You've used up over 90% of your quota!");
  >           } else if percentage_of_max >= 0.75 {
  >               self.messenger
  >                   .send("Warning: You've used up over 75% of your quota!");
  >           }
  >       }
  >   }
  >   ```

- 重要な点：
  - `Messenger` トレイトには、`self` への不変参照とメッセージのテキストを取る `send` というメソッドが 1 つある
  - `LimitTracker` の `set_value` メソッドの振る舞いをテストしたい
    - しかし、`set_value` メソッドは `assert` できるものを返してくれない... 

- --> `LimitTracker` の `set_value` メソッドをテストするために、`Messenger` トレイトを実装するモックオブジェクトを作成して、そのモックオブジェクトが期待通りのメッセージをもつことを確認することでテストを実行したい...
- --> 以下のような実装を考える（しかし、これは借用チェッカーに拒否される）：

  - ここで、もともと、`LimitTracker` の `messenger` が不変借用される想定であり、`send` メソッドが `self` への不変参照をとることを考えると、`mock_messenger` が内部に `Vec<String>` を持ち、そこにメッセージを蓄えるという設計であることは借用規則に反している：

  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      struct MockMessenger {
          sent_messages: Vec<String>,
      }

      impl MockMessenger {
          fn new() -> MockMessenger {
              MockMessenger {
                  sent_messages: vec![],
              }
          }
      }

      impl Messenger for MockMessenger {
          fn send(&self, message: &str) {
              self.sent_messages.push(String::from(message));
          }
      }

      #[test]
      fn it_sends_an_over_75_percent_warning_message() {
          let mock_messenger = MockMessenger::new();
          let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

          limit_tracker.set_value(80);

          assert_eq!(mock_messenger.sent_messages.len(), 1);
      }
  }
  ```

- --> この問題は内部可変性を使えば解消できる：

  ```diff
  #[cfg(test)]
  mod tests {
      use super::*;
  +   use std::cell::RefCell;

      struct MockMessenger {
  -       sent_messages: Vec<String>,
  +       sent_messages: RefCell<Vec<String>>,
      }

      impl MockMessenger {
          fn new() -> MockMessenger {
              MockMessenger {
  -               sent_messages: vec![],
  +               sent_messages: RefCell::new(vec![]),
              }
          }
      }

      impl Messenger for MockMessenger {
          fn send(&self, message: &str) {
  -           self.sent_messages.push(String::from(message));
  +           self.sent_messages.borrow_mut().push(String::from(message));  // borrow_mut() で可変参照を一時的に得て値を push する
          }
      }

      #[test]
      fn it_sends_an_over_75_percent_warning_message() {
          // --snip--
          let mock_messenger = MockMessenger::new();
          let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

          limit_tracker.set_value(80);

  -       assert_eq!(mock_messenger.sent_messages.len(), 1);
  +       assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);  //  borrow() で不変参照を得る
      }
  }
  ```

### `Rc<T>` と `RefCell<T>` を組み合わせることで可変なデータに複数の所有者を持たせる

- `Rc<T>`：何らかのデータに複数の所有者を作成できるが、**不変のアクセスしかさせてくれない**
- `RefCell<T>` を抱える `Rc<T>` （**`Rc<RefCell<T>>`**）を作成することで、**複数の所有者を持ちつつ、可変的な値を扱える**

- 例：複数の所有者に所有される要素からなるコンスリストを可変化する

  ```rust
  #[derive(Debug)]
  enum List {
      Cons(Rc<RefCell<i32>>, Rc<List>),
      Nil,
  }

  use crate::List::{Cons, Nil};
  use std::cell::RefCell;
  use std::rc::Rc;

  fn main() {
      let value = Rc::new(RefCell::new(5));  // Rc で包むことで `RefCell { value: 5 }` の所有権を複数生成できるようにする. RefCell で包むことで `5` の不変・可変参照に関する借用規則は実行時にチェックされる

      let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));  // a にも 'value の Rc の中身'の所有権を共有

      let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));  // b にも 'a の Rc の中身'の所有権を共有
      let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));  // c にも 'a の Rc の中身'の所有権を共有

      *value.borrow_mut() += 10;  //  value の borrow_mut メソッドを呼び出すことで自動参照外しして Rc を剥きながら、RefMut （RefCell の中身への可変参照）に変換し、この可変参照に対して `*` を適用することで value の中身 `5` を可変的に扱っている

      println!("a after = {:?}", a);  // a after = Cons(RefCell { value: 15 }, Nil)
      println!("b after = {:?}", b);  // b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
      println!("c after = {:?}", c);  // c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
  }
  ```

## 15.6 循環参照は、メモリをリークすることもある

- `Rc<T>` と `RefCell<T>` を組合わせると循環参照によって参照のカウント値が０にならず、メモリリークを起こす例を構築できる
- すなわち、**Rust ではメモリリークを完全に回避することはできない**
- `Rc<T>` 値を含む `RefCell<T>` 値があるなどの内部可変性と参照カウントのある型がネストして組み合わさっていたら、循環していないことをプログラマの側で保証しなければならない！（コンパイラは保証してくれない...）

### 循環参照の例

- 例：循環参照が起こる例

  ```rust
  use crate::List::{Cons, Nil};
  use std::cell::RefCell;
  use std::rc::Rc;

  #[derive(Debug)]
  enum List {
      Cons(i32, RefCell<Rc<List>>),
      Nil,
  }

  impl List {
      fn tail(&self) -> Option<&RefCell<Rc<List>>> {
          match self {
              Cons(_, item) => Some(item),
              Nil => None,
          }
      }
  }

  fn main() {
      let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

      println!("a initial rc count = {}", Rc::strong_count(&a));  // 1
      println!("a next item = {:?}", a.tail());  // Some(RefCell { value: Nil })

      let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

      println!("a rc count after b creation = {}", Rc::strong_count(&a));  // 2
      println!("b initial rc count = {}", Rc::strong_count(&b));  // 1
      println!("b next item = {:?}", b.tail());  // Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

      if let Some(link) = a.tail() {
          *link.borrow_mut() = Rc::clone(&b);  // ここで a のしっぽを b に向けさせて循環を作成する
      }

      println!("b rc count after changing a = {}", Rc::strong_count(&b));  // 2
      println!("a rc count after changing a = {}", Rc::strong_count(&a));  // 2

      // Uncomment the next line to see that we have a cycle;
      // it will overflow the stack
      // println!("a next item = {:?}", a.tail());
  }
  ```

  - 生成される循環参照を表す図：
    ![循環参照](https://doc.rust-lang.org/book/img/trpl15-04.svg)

### 循環参照を回避する: `Rc<T>` を `Weak<T>` に変換する

- `Rc::clone` を呼び出して、`Rc<T>` インスタンスの `strong_count` を +1 する代わりに、
- **`Rc:downgrade`** を呼び出して、`Rc<T>` インスタンスの **`weak_count` を +1**することができる（`Weak<T>` 型のスマートポインタが得られる）

  - `strong_count` が 0 になると、`Rc<T>` インスタンスが片づけられるが、 `weak_count` が 0 になってもとくに何も起こらない
- `Weak<T>` が参照する値はドロップされてしまっている可能性がある
  - → `Weak<T>` の `upgrade` メソッドを呼び出すことで `Option<Rc<T>>` を取得する

#### 要するに

- **`Rc::downgrade`**：`Rc::clone` の代わり.
  - `weak_count` を +1 する. 
  - `Weak<T>` 型のスマートポインタを生成
- `Weak<T>` の **`upgrade`** メソッド：参照外し兼 `Option` の unwrap

#### `Weak<T>` の使用例：木データ構造を作る（子ノードのある Node）

- `Node` 構造体を定義：
  - `value`：各ノードの値
  - `children`：ノードに続くノードの配列

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

##### 子供から親への参照を追加する

- `Node` 構造体に `parent` フィールドを追加する
  - 循環参照を避けるために `parent` の型は `RefCell<Weak<Node>>` とする（`RefCell` は `parent` を可変化するために使用）
  - ただし、親ノードは子ノードを所有すべき（親が消えたら子も消えるべき）なので、`children` の型は `RefCell<Vec<Rc<Node>>>` のままでよい

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),  // `Weak::new` で中身が空の `Weak<T>` スマートポインタを作成
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());  // leaf parent = None  （この時点では `leaf.parent` の中身は `None`）

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),  // `Weak::new` で中身が空の `Weak<T>` スマートポインタを作成
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // `parent` の `RefCell` の中身（`Weak<Node>` 型）に対して参照外し演算子 `*` を作用させて `parent` の中身に変更を加えている. 
    // `Rc::downgrade` で参照 `&branch` を `Weak<T>` 型のスマートポインタに変換している
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);  

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());  /* 
      leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
      children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
      children: RefCell { value: [] } }] } })
    （この時点では `leaf.parent` の中に `Weak<Rc<Node>>` 型の値（`branch`）が入っている）
    */
}
```

##### `strong_count` と `weak_count` への変更を可視化する

- `strong_count` と `weak_count` への変更を観察する

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    ); // leaf strong = 1, weak = 0

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]), // leaf の strong_count +1
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // branch の weak_count +1

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        ); // branch strong = 1, weak = 1

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        ); // leaf strong = 2, weak = 0
    } // ここで `branch` がドロップする（leaf の strong_count が -1）

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // leaf parent = None
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    ); // leaf strong = 1, weak = 0
}
```
