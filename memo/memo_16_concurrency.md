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

## 16.0 概要

- aaa

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
