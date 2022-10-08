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

## 16.0 概要

- `std::thread` の基本的な使い方
  - --> [`spawn` で新規スレッドを生成する](#spawn-で新規スレッドを生成する), [`join` ハンドルで全スレッドの終了を待つ](#join-ハンドルで全スレッドの終了を待つ)
  - 新規スレッド作成時には `move` で環境変数の所有権を奪い取ることが多い
    - --> [スレッドで `move` クロージャを使用する](#スレッドで-move-クロージャを使用する)

- `std::sync::mpsc` の基本的な使い方
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

### `join` ハンドルで全スレッドの終了を待つ

- `thread::spawn` の戻り値を変数に保存することで、立ち上げたスレッドの実行を強制し、完全に実行されるのを待つことができる
- `thread::spawn` の返り値の型は **`JoinHandle`**
  - `JoinHandle` の `join` メソッドを呼び出すとハンドルが表すスレッドの終了まで現在実行中のスレッドをブロックする

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

- `std::sync::mpsc;` で導入
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
          tx.send(val).unwrap();
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
