# 16 章：恐れるな！ 並行性

## 目次

- [16 章：恐れるな！ 並行性](#16-章恐れるな-並行性)
  - [目次](#目次)
  - [16.0 概要](#160-概要)
  - [16.1 スレッドを使用してコードを同時に走らせる](#161-スレッドを使用してコードを同時に走らせる)
    - [用語解説](#用語解説)
    - [`spawn` で新規スレッドを生成する](#spawn-で新規スレッドを生成する)
    - [`join` ハンドルで全スレッドの終了を待つ](#join-ハンドルで全スレッドの終了を待つ)
    - [スレッドで `move` クロージャを使用する](#スレッドで-move-クロージャを使用する)
  - [16.2 メッセージ受け渡しを使ってスレッド間でデータを転送する](#162-メッセージ受け渡しを使ってスレッド間でデータを転送する)
    - [チャンネルの生成](#チャンネルの生成)
    - [`tx` からの送信](#tx-からの送信)
    - [`rx` での受信（`recv` メソッドと `try_recv` メソッド）](#rx-での受信recv-メソッドと-try_recv-メソッド)
      - [`recv` メソッド](#recv-メソッド)
      - [`try_recv` メソッド](#try_recv-メソッド)
    - [チャンネルと所有権の転送](#チャンネルと所有権の転送)
    - [複数の値を送信し、受信側が待機するのを確かめる](#複数の値を送信し受信側が待機するのを確かめる)
    - [転送機をクローンして複数の生成器を作成する（multiple producer）](#転送機をクローンして複数の生成器を作成するmultiple-producer)
  - [16.3 状態共有並行性（複数のスレッド間でのメモリ共有）](#163-状態共有並行性複数のスレッド間でのメモリ共有)
    - [`Mutex<T>`：ミューテックス（同時には１つまでのスレッドにしかアクセスを許可しない）](#mutextミューテックス同時には１つまでのスレッドにしかアクセスを許可しない)
    - [シングルスレッドの場合](#シングルスレッドの場合)
    - [マルチスレッドの場合](#マルチスレッドの場合)
    - [`RefCell<T>`/`Rc<T>`と `Mutex<T>`/`Arc<T>` の類似性（内部不変性）](#refcelltrctと-mutextarct-の類似性内部不変性)
  - [16.4 `Sync` と `Send` トレイトで拡張可能な並行性](#164-sync-と-send-トレイトで拡張可能な並行性)
    - [`Send` マーカートレイトでスレッド間の所有権の転送を許可する](#send-マーカートレイトでスレッド間の所有権の転送を許可する)
    - [`Sync` マーカートレイトで複数のスレッドからのアクセスを許可する](#sync-マーカートレイトで複数のスレッドからのアクセスを許可する)

## 16.0 概要

- `std::thread` の基本的な使い方（スレッド）
  - --> [`spawn` で新規スレッドを生成する](#spawn-で新規スレッドを生成する), [`join` ハンドルで全スレッドの終了を待つ](#join-ハンドルで全スレッドの終了を待つ)
  - 新規スレッド作成時には `move` で環境変数の所有権を奪い取ることが多い
    - --> [スレッドで `move` クロージャを使用する](#スレッドで-move-クロージャを使用する)

- `std::sync::mpsc` の基本的な使い方（スレッド間のメッセージのやり取りにチャンネルを用いる）
  - 送信機と受信機の対生成
    - --> [チャンネルの生成](#チャンネルの生成)
  - 送信
    - --> [`tx` からの送信](#tx-からの送信)
  - 受信
    - --> [`rx` での受信（`recv` メソッドと `try_recv` メソッド）](#rx-での受信recv-メソッドと-try_recv-メソッド)
  - `tx` は送信する値の所有権を奪う
    - --> [チャンネルと所有権の転送](#チャンネルと所有権の転送)
  - `rx` をイテレータとして扱い、`tx` からメッセージを複数回受け取る
    - --> [複数の値を送信し受信側が待機するのを確かめる](#複数の値を送信し受信側が待機するのを確かめる)
  - 送信機を複数個に増やすには `mpsc::Sender::clone(&tx)` する
    - --> [転送機をクローンして複数の生成器を作成する（multiple producer）](#転送機をクローンして複数の生成器を作成するmultiple-producer)

- `Mutex<T>` の基本的な使い方（相互排他的なデータアクセス）
  - ミューテックスはロックシステム経由で保持しているデータを死守する (guarding)
  - 生成・ロック・アンロック
    - --> [`Mutex<T>`：ミューテックス（同時には１つまでのスレッドにしかアクセスを許可しない）](#mutextミューテックス同時には１つまでのスレッドにしかアクセスを許可しない)
  - マルチスレッドの場合は `Arc<T>` と併用する
    - --> [マルチスレッドの場合](#マルチスレッドの場合)
  - `Mutex<T>` は、`RefCell<T>` のように**内部可変性**を提供する
    - --> [`RefCell<T>`/`Rc<T>`と `Mutex<T>`/`Arc<T>` の類似性](#refcelltrctと-mutextarct-の類似性内部不変性)
  - デッドロック
    - --> [`RefCell<T>`/`Rc<T>`と `Mutex<T>`/`Arc<T>` の類似性](#refcelltrctと-mutextarct-の類似性内部不変性)

- `Sync` と `Send`（並行性に関連する不変条件を強制するマーカートレイト）
  - --> [16.4 `Sync` と `Send` トレイトで拡張可能な並行性](#164-sync-と-send-トレイトで拡張可能な並行性)

## 16.1 スレッドを使用してコードを同時に走らせる

Rust ではランタイムをできるだけ小さくするために、1:1 スレッドの実装のみを提供する。（M:N スレッドの実装をしたクレートもある）

### 用語解説

- **プロセス**：プログラムのコードの実行単位（１プログラムに対し１プロセス）
- **スレッド**：プログラム内の独立した部分を走らせる機能（１プログラム内で複数走る）
- プログラム言語によってスレッドの実装法は異なる
  - 多くのOSで新規スレッド作成用のAPIが提供されている
  - 言語がOSのAPIを呼び出すモデルを **"1:1"** と呼ぶ. このモデルでは、１つのOSスレッドに対し１つの言語スレッド
  - 言語がスレッドの独自の特別な実装を提供し、各言語スレッドに対し異なる数のOSスレッドが実行されるモデルを **"M:N"** モデルと呼ぶ. また、プログラミング言語が提供するスレッドは**グリーンスレッド**と呼ばれる
- **ランタイム**：、言語によって全てのバイナリに含まれるコード
  - ランタイムが小さいと機能も少ないすが、バイナリのサイズも小
さくなる

### `spawn` で新規スレッドを生成する

- `main` 関数もスレッドを作成する模様
  - &rarr; `main` スレッドなどの既存スレッドの内部から他のスレッドを生成する

- 新規スレッドを生成するには、`thread::spawn` 関数を呼び出す
  - 引数には、新規スレッドで走らせたいコードを含むクロージャを渡す

- スレッドを一定時間休止するには `thread::sleep` 関数を用いる
  - 休止中は（おそらく）他のスレッドが実行される

- 例：以下のコードではスポーンドスレッド内のループが10回実行する前に、メインスレッドが先に実行終了するので、スポーンメソッド内のループはすべて実行されずに動作を終了する

  ```rust
  use std::thread;
  use std::time::Duration;

  fn main() {
      thread::spawn(|| {
          for i in 1..10 {
              println!("hi number {} from the spawned thread!", i);
              thread::sleep(Duration::from_millis(1));
          }
      });

      for i in 1..5 {
          println!("hi number {} from the main thread!", i);
          thread::sleep(Duration::from_millis(1));
      }
  }
  ```

  実行結果：

  ```sh
  hi number 1 from the main thread!
  hi number 1 from the spawned thread!
  hi number 2 from the main thread!
  hi number 2 from the spawned thread!
  hi number 3 from the main thread!
  hi number 3 from the spawned thread!
  hi number 4 from the main thread!
  hi number 4 from the spawned thread!
  hi number 5 from the spawned thread!
  ```

### `join` ハンドルで全スレッドの終了を待つ

- `thread::spawn` の戻り値を変数に保存することで、立ち上げたスレッドの実行を強制し、完全に実行されるのを待つことができる
- `thread::spawn` の返り値の型は **`JoinHandle`**
  - `JoinHandle` の `join` メソッドを呼び出すと、ハンドルが表すスレッドの終了まで `join` メソッド呼び出し元のスレッドをブロックする

- 例：`join` メソッドを呼び出すことで、スポーンドスレッドの終了までメインスレッドの終了を先延ばしさせている

  ```rust
  use std::thread;
  use std::time::Duration;

  fn main() {
      let handle = thread::spawn(|| {
          for i in 1..10 {
              println!("hi number {} from the spawned thread!", i);
              thread::sleep(Duration::from_millis(1));
          }
      });

      for i in 1..5 {
          println!("hi number {} from the main thread!", i);
          thread::sleep(Duration::from_millis(1));
      }

      handle.join().unwrap();
  }
  ```

  実行結果：

  ```sh
  hi number 1 from the main thread!
  hi number 1 from the spawned thread!
  hi number 2 from the main thread!
  hi number 2 from the spawned thread!
  hi number 3 from the spawned thread!
  hi number 3 from the main thread!
  hi number 4 from the spawned thread!
  hi number 4 from the main thread!
  hi number 5 from the spawned thread!
  hi number 6 from the spawned thread!
  hi number 7 from the spawned thread!
  hi number 8 from the spawned thread!
  hi number 9 from the spawned thread!
  ```

### スレッドで `move` クロージャを使用する

- `move` クロージャは、`thread::spawn` とともによく使用される
  - あるスレッドから別のスレッドに値の所有権を移すために新しいスレッドを生成する際に特に有用

- 例：

  ```rust
  use std::thread;

  fn main() {
      let v = vec![1, 2, 3];

      // ここの `move` は必要. なぜなら、spawned thread は v よりも長生きする可能性があるから.
      let handle = thread::spawn(move || {
          println!("Here's vector: {:?}", v);
      });

      // 例えば、ここで、`drop(v);` すると、確実に spawned thread は v よりも長生きする

      handle.join().unwrap();
  }
  ```

## 16.2 メッセージ受け渡しを使ってスレッド間でデータを転送する

> **「メモリを共有することでやり取りするな;代わりにやり取りすることでメモリを共有しろ」**

- Rust では**チャンネル**でメッセージ送信並行性を達成する
  - チャンネルは、転送機（`tx`）と受信機（`rx`）からなる
  - `tx` と `rx` のいずれかがドロップされるとチャンネルは閉じられる

### チャンネルの生成

- `use std::sync::mpsc;` で導入
- `let (tx, rx) = mpsc::channel();` でチャンネルを作成する
- ここで、`mpsc` とは **multiple producer, single consumer** を表す
  - （1チャンネルにつき複数送信機と単一の受信機が存在可能）
- 例：

  ```rust
  use std::sync::mpsc;

  fn main() {
    let (tx, rx) = mpsc::channel();
  }
  ```

### `tx` からの送信

- まず送信側にしたいスレッドに `tx` を所有権ごと渡す
- `tx.send` を呼び出すと `Result<T,E>` 型を返す
  - すでに `rx` がドロップされており、送信先が存在しなければ、`Err` が返ってくる
  - 例えば、`tx.send.unwrap();` とすると `Err` の場合にパニックを起こす
- 例：

  ```rust
  use std::thread;
  use std::sync::mpsc;

  fn main() {
      let (tx, _rx) = mpsc::channel();

      // `move` をつけているので、`tx` はクロージャにムーブされる
      thread::spawn(move || {
          let val = String::from("hi");
          tx.send(val).unwrap();  // あえて unwrap することでエラー発生時にパニックを起こすようにしている
      });
  }
  ```

### `rx` での受信（`recv` メソッドと `try_recv` メソッド）

#### `recv` メソッド

- メッセージを受信したいスレッドで `rx.recv` を呼びだすと `Result<T,E>` が返ってくる
  - `recv` メソッドを呼び出すとそのスレッドの実行をブロックして値がチャンネルに流れてくるのを待機する
  - `tx` がドロップされていたら `recv` メソッドは `Err` を返す
- 例：

  ```rust
  use std::thread;
  use std::sync::mpsc;

  fn main() {
      let (tx, rx) = mpsc::channel();

      // `move` をつけているので、`tx` はクロージャにムーブされる
      thread::spawn(move || {
          let val = String::from("hi");
          tx.send(val).unwrap();
      });

      let received = rx.recv().unwrap();
      println!("Got: {}", received);
  }
  ```

#### `try_recv` メソッド

- `try_recv` メソッドはスレッドの実行をブロックせず、代わりに即座に `Result<T,E>` を返す
  - メッセージがあればそれを含む `Ok` 値
  - なければ `Err` を返す
- メッセージ待機中に他にやることがある場合に有用

### チャンネルと所有権の転送

- `tx.send(hoge)` は `hoge` の所有権を奪う（他のスレッドに移す）
  - おかげで、送信後に誤って再度値を使用するのが防がれる
- 例えば、以下のコードはコンパイルエラーを起こす：

  ```rust
  use std::thread;
  use std::sync::mpsc;

  fn main() {
      let (tx, rx) = mpsc::channel();

      thread::spawn(move || {
          let val = String::from("hi");
          tx.send(val).unwrap();  // ここで `val` はムーブ済み
          println!("val is {}", val);  // なので、ここでは `val` にアクセスできない
      });

      let received = rx.recv().unwrap();
      println!("Got: {}", received);
  }
  ```

### 複数の値を送信し、受信側が待機するのを確かめる

- `rx` をイテレータとして扱うこともできる
  - イテレータの繰り返しは、チャンネルが閉じられると終了する
- この場合、`rx` は `tx` から値を複数回受け取る

- 例：

  ```rust
  use std::thread;
  use std::sync::mpsc;
  use std::time::Duration;

  fn main() {
      let (tx, rx) = mpsc::channel();

      thread::spawn(move || {
          let vals = vec![
              String::from("hi"),
              String::from("from"),
              String::from("the"),
              String::from("thread"),
          ];
          for val in vals {
              tx.send(val).unwrap();
              thread::sleep(Duration::from_secs(1));
          }
      });

      for received in rx {
          println!("Got: {}", received);
      }
  }
  ```

  これを実行すると以下のような出力を返す（一秒おきに一行ずつ表示される）

  ```txt
  Got: hi
  Got: from
  Got: the
  Got: thread
  ```

### 転送機をクローンして複数の生成器を作成する（multiple producer）

- `tx` に対し `mpsc::Sender::clone(&tx)` することで `rx` の対となる送信機を複製することができる（すべて同一の `rx` にメッセージを送信する）
- 例：

  ```rust
  use std::thread;
  use std::sync::mpsc;
  use std::time::Duration;

  fn main() {
      let (tx, rx) = mpsc::channel();

      let tx1 = mpsc::Sender::clone(&tx);

      thread::spawn(move || {
          let vals = vec![
              String::from("hi"),
              String::from("from"),
              String::from("the"),
              String::from("thread"),
          ];

          for val in vals {
              tx1.send(val).unwrap();
              thread::sleep(Duration::from_secs(1));
          }
      });

      thread::spawn(move || {
          let vals = vec![
              String::from("more"),
              String::from("messages"),
              String::from("for"),
              String::from("you"),
          ];

          for val in vals {
              tx.send(val).unwrap();
              thread::sleep(Duration::from_secs(1));
          }
      });

      for received in rx {
          println!("Got: {}", received);
      }
  }
  ```

  コードを実行すると、出力は以下のようなものになる

  ```txt
  Got: hi
  Got: more
  Got: from
  Got: messages
  Got: the
  Got: for
  Got: thread
  Got: you
  ```

## 16.3 状態共有並行性（複数のスレッド間でのメモリ共有）

### `Mutex<T>`：ミューテックス（同時には１つまでのスレッドにしかアクセスを許可しない）

- ミューテックス：”mutual exclusion”(相互排他) の省略形
  - ミューテックスはロックシステム経由で保持しているデータを死守する (guarding)
  - ミューテックス内にアクセスするには、ミューテックスのロックを所望し、アクセスを要請する
  - データの使用が終わったらアンロックする

- 生成には `Mutex::new` を用いる
- 内部のデータにアクセスするにはミューテックスインスタンスの `lock` メソッドを呼び出してロックを取得する
  - この動作は現在のスレッドをブロックする（ロックを得られる順番が来るまで動作を停止する）
  - ロックを保持するスレッドがパニックを起こしたら `lock` メソッドは `Err` を返す
  - `lock` メソッドの返り値は `MutexGuard<T>` というスマートポインタ
    - --> 参照外しやドロップ時の動作などが実装されている

### シングルスレッドの場合

- 例：シングルスレッドでの使用例

  ```rust
  use std::sync::Mutex;

  fn main() {
      let m = Mutex::new(5);

      {
          let mut num: MutexGuard<i32> = m.lock().unwrap();  // `lock` メソッドは `Result<MutexGuard<T>, E>` を返すので `unwrap()` して、`Err` 返却時にはパニックするように設定している
          *num = 6;  // `MutexGuard<T>` は `Deref` トレイトを実装しているので参照外しで内部データにアクセスできる
      }  // `MutexGuard<T>` は `Drop` トレイトを実装しているのでスコープを外れるここで自動的にアンロックされる

      println!("m = {:?}", m);  // m = 6；上のスコープで行った変更が反映されている
  }
  ```

### マルチスレッドの場合

- 複数のスレッド間で一つのミューテックスを共有する場合は `Arc<T>` を用いて `Mutex<T>` を複製・共有する
  - `Arc<T>` は "Atomic refference counter" の略
  - マルチスレッドでも使える `Rc<T>` のようなもの（`Rc<T>` はシングルスレッド用）
- スレッドに `move` する前に `Arc::clone(&mutex)` でクローンし
- クローンしたものをスレッドに渡して、`lock` するなり煮るなり焼くなりする

- 例：複数のスレッド間でミューテックスを共有する

  ```rust
  use std::sync::{Mutex, MutexGuard};
  use std::thread;
  use std::sync::Arc;

  fn main() {
      let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));  // `Arc::new` で複数の所有者の存在を許す（`Rc::new` でも似たことができるがマルチスレッドに対応していない）
      let mut handles = vec![];

      for _ in 0..10 {
          let counter: Arc<Mutex<i32>> = Arc::clone(&counter);  // thread に `move` する前に複製
          let handle = thread::spawn(move || {  // ここで `move` キーワードをつけているので `counter` の所有権はクロージャに移動する
              let mut num: MutexGuard<i32> = counter.lock().unwrap();  // 複製した `Arc<Mutex<i32>>`（`couter`）に対し `unlock` メソッドを呼び出す
              
              *num += 1;
          });  // ここ（クロージャの終り）で複製された `Arc<Mutex<i32>>`（`couter`）はドロップされ、Mutex はアンロックされる
          handles.push(handle);
      }

      for handle in handles {
          handle.join().unwrap();
      }

      println!("Result: {}", counter.lock().unwrap());
  }
  ```

### `RefCell<T>`/`Rc<T>`と `Mutex<T>`/`Arc<T>` の類似性（内部不変性）

- `Mutex<T>` は、`RefCell<T>` のように**内部可変性**を提供する
  - `Arc<Mutex<T>>` 型の変数を `let`（一見して不変）で定義しても、`lock` メソッドでその内部にある値への可変参照（`MutexGuard<T>` 型）を得ることができる
  - 例：[マルチスレッドの場合](#マルチスレッドの場合) の例の中の `counter`

- **デッドロック**には注意！！
  - デッドロック：二つのミューテックス A, B をロックしないと進まない処理が二つあるときに、一方の処理が A をロックし、もう一方が B をロックすると両方の処理が進まなくなる

## 16.4 `Sync` と `Send` トレイトで拡張可能な並行性

- この節の内容はやや発展的
  - 基本的には「**`Send`（`Sync`）を実装している型からなる型は自動で `Send`（`Sync`）になる**」ことを押さえておけばよい
  - これらのマーカートレイトは「並行性に関連する不変条件を強制することに役立つだけ」
    - ここでいう「並行性に関連する不変条件」とは、ある型に関する以下のような決め事のこと：
      - 「その型は複数のスレッド間での所有権の移動を許可しているか？」
      - 「その型は複数のスレッドからのアクセスを許可しているか？」
  - これらのトレイトを手動で実装して、`Send` あるいは `Sync` ではない部品からなる新しい並行な型を構成するには unsafe な Rust コードを実装することが必要になる（詳しくは "[The Rustonomicon](https://doc.rust-jp.rs/rust-nomicon-ja/index.html)" を参照せよ）

### `Send` マーカートレイトでスレッド間の所有権の転送を許可する

- `Send` マーカートレイト：`Send` を実装した型の所有権をスレッド間で転送できることを示唆する
  - Rust のほとんどの型は `Send` を実装している
  - 生ポインタを除くほとんどの基本型も `Send`
  - 完全に `Send` の型からなる型も全て自動的に `Send` と印付けされる
  - `Rc<T>` を含む一部の例外では実装されていない

### `Sync` マーカートレイトで複数のスレッドからのアクセスを許可する

- `Sync` マーカートレイト：`Sync` を実装した型は、複数のスレッドから参照されても安全であることを示唆する
  - `&T`（`T` への参照）が `Send` なら、型 `T` は `Sync`
  - 基本型は `Sync`
  - `Sync` の型からのみ構成される型も `Sync`
  - `Rc<T>` や `Cell` 系などの一部例外では実装されていない
  - 一方 `Mutex<T>` は `Sync` なので[マルチスレッドの場合](#マルチスレッドの場合)の例ような実装ができる
