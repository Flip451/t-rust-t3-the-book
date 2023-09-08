# 20 章２節：最後のプロジェクト：マルチスレッドの Web サーバ

## 目次

- [20 章２節：最後のプロジェクト：マルチスレッドの Web サーバ](#20-章２節最後のプロジェクトマルチスレッドの-web-サーバ)
  - [目次](#目次)
  - [20.2.0 概要](#2020-概要)
  - [20.2 シングルスレッドサーバをマルチスレッド化する](#202-シングルスレッドサーバをマルチスレッド化する)
    - [処理に時間のかかるリクエストをシミュレーションする](#処理に時間のかかるリクエストをシミュレーションする)
    - [スレッドプールでスループットを向上する](#スレッドプールでスループットを向上する)
    - [各リクエストごとにスレッドを作成する場合](#各リクエストごとにスレッドを作成する場合)
    - [有限の数のスレッドの生成](#有限の数のスレッドの生成)
    - [コンパイラ駆動開発でスレッドプールを構築する](#コンパイラ駆動開発でスレッドプールを構築する)
    - [スレッドを格納する領域を作成する](#スレッドを格納する領域を作成する)
    - [スレッドプールからスレッドへのコードの送信を担当する `Worker` 構造体](#スレッドプールからスレッドへのコードの送信を担当する-worker-構造体)
    - [チャンネル経由でスレッドにリクエストを送信する](#チャンネル経由でスレッドにリクエストを送信する)
    - [`excute` メソッドの実装](#excute-メソッドの実装)

## 20.2.0 概要

- 20.1 節で作成したシングルスレッドウェブサーバをマルチスレッド化する

## 20.2 シングルスレッドサーバをマルチスレッド化する

### 処理に時間のかかるリクエストをシミュレーションする

- 現在の実装は、処理に時間のかかるリクエストが来た時に、他のリクエストの処理にも影響が出てしまうようになっている
- この節では、これを確認したい
- そのために、応答前に５秒間スリープする `/sleep` というルートを追加する：

  ```rs
  use std::io::BufReader;
  use std::net::{TcpListener, TcpStream};
  use std::thread;
  use std::time::Duration;
  use std::{
      fs,
      io::{prelude::*, Result},
  };

  fn handle_connection(mut stream: TcpStream) -> Result<()> {
      let buf_reader = BufReader::new(&mut stream);
      // バッファの最初の行を取得
      if let Some(request_line) = buf_reader.lines().next() {
          let request_line = request_line?;

          // 受信内容の最初の行の内容で分岐
          let (file_path, status_line) = match &request_line[..] {
              "GET / HTTP/1.1" => ("index.html", "HTTP/1.1 200 OK"),
              "GET /sleep HTTP/1.1" => {
                  thread::sleep(Duration::from_secs(5));
                  ("index.html", "HTTP/1.1 200 OK")
              }
              _ => ("404.html", "HTTP/1.1 404 NOT FOUND"),
          };

          // index.html の内容を取得
          let contents = fs::read_to_string(file_path)?;

          // ヘッダーを作成
          let length = contents.len();
          let headers = format!("Content-Length: {}\r\n", length);

          // レスポンスを返却
          let response = format!("{}\r\n{}\r\n{}", status_line, headers, contents);
          stream.write_all(response.as_bytes())?;
      }

      Ok(())
  }

  fn main() -> Result<()> {
      // TCP リスナーを作成
      let listener = TcpListener::bind("127.0.0.1:7878")?;

      // listener.incoming() の返り値のイテレータは一連のストリームを返す
      // 各ストリームは、クライアント・サーバ間の接続に対応する
      // ストリームはスコープを抜けると `drop` 実装の一部として close される
      for (index, stream) in listener.incoming().enumerate() {
          println!("{} 個目の stream が生成されました！", index);
          handle_connection(stream?)?;
      }
      Ok(())
  }

  ```

  - このコードを実行し、<http://127.0.0.1:7878/sleep> にアクセスするとページの表示までに遅延が発生することを確かめられる
  - 一方、<http://127.0.0.1:7878/> にアクセスしたときには遅延は発生しない
  - しかし、<http://127.0.0.1:7878/sleep> へアクセスして、レスポンスを待っている間に <http://127.0.0.1:7878/> にアクセスすると、前者からのレスポンスが返ってくるまで後者からのレスポンスも返ってこないことが確かめられる
  - この問題は、スレッドが一つしかないことが原因で起きている

- 次の節以降で、この問題を解消し、時間のかかるリクエストへの処理の間にも他の軽量なリクエストへの対応をできるようにする

### スレッドプールでスループットを向上する

- **スレッドプール**：スレッドを事前に複数作成しておき、タスクを処理する準備ができた状態で待機させておく手法
  - プログラムが新しいタスクを受け取ると、プール内のスレッドの一つをそのタスクに割り当て、そのスレッドがそのタスクを処理する
  - プールに残った他のスレッドは、そのあとに来た新しいタスクの対応に利用できる
  - タスクの処理が終わったスレッドはプールに戻される

- スレッドプールを用いることで、複数のコネクションを同時処理できるようになり、スループット（単位時間当たりのタスク処理量）が向上する

- なお、DoS 攻撃から身を守るために、プール内のスレッド数は少数に制限するのがよい
  - リクエストの度に新規スレッドを作成すると、大量のリクエストを受けたときにサーバーの計算資源が食いつくされてしまう

- 基本設計は以下の通り：
  - プールではリクエストのキューを管理する
  - プール内の各スレッドは、このキューからリクエストをポップオフして、リクエストを処理後、キューに再度リクエストを要求する

- cf. Web サーバのスループットを向上させる方法は、このスレッドプールを用いた方法のほかに、以下にあげる方法もある
  - fork/join モデル
  - シングルスレッド非同期 I/O モデル
  - マルチスレッド非同期 I/O モデル

### 各リクエストごとにスレッドを作成する場合

- 本実装の前に、一旦各リクエストごとにスレッドを作成するような実装を以下に示す

  ```diff
  // ...snip...

  fn main() -> Result<()> {
      // TCP リスナーを作成
      let listener = TcpListener::bind("127.0.0.1:7878")?;

      // listener.incoming() の返り値のイテレータは一連のストリームを返す
      // 各ストリームは、クライアント・サーバ間の接続に対応する
      // ストリームはスコープを抜けると `drop` 実装の一部として close される
      for (index, stream) in listener.incoming().enumerate() {
          println!("{} 個目の stream が生成されました！", index);
  -       handle_connection(stream?)?;
  +       let stream = stream?;
  +       // 各コネクションごとにスレッドを生成して、その内部で処理を遂行する
  +       thread::spawn(|| {
  +           handle_connection(stream).expect("Error at handle_connection");
  +       });
      }
      Ok(())
  }
  ```

  - ただし、この実装は DoS 攻撃に耐性がないという欠陥がある
  - だが、前々小節で述べた `/sleep` 周りの問題は解消している
    - ためしに、<http://127.0.0.1:7878/sleep> へアクセスして、レスポンスを待っている間に <http://127.0.0.1:7878/> にアクセスすると、前者の画面表示を待たずに後者の画面が表示されることを確かめられる

- 次節からは、この実装が抱えている DoS 攻撃への脆弱性を解消するように、スレッドプールを用いた実装を進める

### 有限の数のスレッドの生成

- スレッドプールを使った実装にあたって、`thread::spawn` と似た使い慣れたインターフェースを提供するように `ThreadPool` 構造体を定義したい
- すなわち、`ThreadPool` を `main` 関数内で以下のように利用できるよう定義する：

  ```rs
  fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    // ここでスレッドプールを作成する
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream?;

        // スレッドプールで処理を実行するには `excute` メソッドを呼び出すだけでよいように定義したい
        pool.excute(|| {
            handle_connection(stream).expect("Error at handle_connection");
        })
    }

    Ok(())
  }
  ```

### コンパイラ駆動開発でスレッドプールを構築する

- 本節ではコンパイラ駆動開発でスレッドプールの構築を進める
- まず、前節のコードを `main.rs` に記述する
  - すると、当然以下のようなコンパイルエラーが発生する：

    ```sh
    error[E0433]: failed to resolve: use of undeclared type `ThreadPool`
      --> src/main.rs:43:16
       |
    43 |     let pool = ThreadPool::new(4);
       |                ^^^^^^^^^^ use of undeclared type `ThreadPool`

    For more information about this error, try `rustc --explain E0433`.
    error: could not compile `hello` (bin "hello") due to previous error
    ```

- このコンパイルエラーから、`ThreadPool` 型を定義する必要があることがわかるので、以下のように `src/lib.rs` にこの構造体を定義する：

  **`src/lib.rs`**
  
  ```rs
  pub struct ThreadPool;

  impl ThreadPool {
      /// Create a new ThreadPool.
      /// 
      /// The size is the number of threads in the pool.
      /// 
      /// # Panics
      /// 
      /// The `new` function will panic if the size is zero.
      pub fn new(size: usize) -> Self {
          assert!(size > 0);
          ThreadPool
      }

      pub fn excute<F>(&self, f: F)
      where
          F: FnOnce() -> () + Send + 'static,
      {
      }
  }
  ```

  - なお、`excute` の引数の型は `thread::spawn` のシグネチャを参考にして決定した
    - cf. `thread::spawn` のシグネチャ：

      ```rs
      pub fn spawn<F, T>(f: F) -> JoinHandle<T>
      where
          F: FnOnce() -> T,
          F: Send + 'static,
          T: Send + 'static,
      ```

- ここまでの変更で `cargo check` は無事に通るようになる
- また、`new` 関数のドキュメントを追加している
  - `cargo doc --open` で内容を確認できる

### スレッドを格納する領域を作成する

- ここで再度、`thread::spawn` のシグネチャに着目すると、返り値の型は `JoinHandle` 型である
- 今回は、この構造体を複数個 `ThreadPool` 構造体で管理することになる
- そこで、以下のように変更する

  ```diff
  + use std::thread::JoinHandle;

  - pub struct ThreadPool;
  + pub struct ThreadPool {
  +     threads: Vec<JoinHandle<()>>
  + }

  impl ThreadPool {
      /// Create a new ThreadPool.
      /// 
      /// The size is the number of threads in the pool.
      /// 
      /// # Panics
      /// 
      /// The `new` function will panic if the size is zero.
      pub fn new(size: usize) -> Self {
          assert!(size > 0);
  
  -       ThreadPool
  +       let mut threads = Vec::with_capacity(size);
  +       
  +       for _ in 0..size {
  +           // ここでスレッドを作成して threads に追加する
  +           todo!()
  +       }
  + 
  +       ThreadPool { threads }
      }

      pub fn excute<F>(&self, f: F)
      where
          F: FnOnce() -> () + Send + 'static,
      {
      }
  }
  ```

### スレッドプールからスレッドへのコードの送信を担当する `Worker` 構造体

- この節では、プーリングの実装でよく使われる "Woker" という概念を導入する
  - 前節では、`ThreadPool` 構造体に `threads: Vec<JoinHandle<()>>` を格納したが、代わりに `Vec<Worker>` を格納するように変更する
- この `Worker` 構造体を以下のような性質を持つ：
  - 各 `Worker` 構造体は、内部にタスクの処理を担当するスレッドを持つ
  - `Worker` は（何らかの方法で）実行すべきコードを拾い上げて、自身のスレッド内で実行する機能を有する

- そこで、以下の実装を行う：
  1. `id` と `JoinHandle<()>` を保持する`Worker`構造体を定義する
  2. `Worker::new` 関数を定義する
     - この関数は `id` を引数に持ち、内部で、スレッドを生成する（この節では仮に空のクロージャでスレッドを生成する）
  3. `ThreadPool` を変更して `Worker` インスタンスのベクタを保持させる
  4. `ThreadPool::new` の実装を変更
     - 複数の `Worker` インスタンスを生成して `id` を付与し、それらをベクタを格納する
  5. `Worker` 内のスレッドにタスクを渡して処理させる機能は次節以降で実装する

```diff
use std::thread::{self, JoinHandle};

pub struct ThreadPool {
-     threads: Vec<JoinHandle<()>>,
+     workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

-       let mut threads = Vec::with_capacity(size);
-       
-       for _ in 0..size {
-           // ここでスレッドを作成して threads に追加する
-           todo!()
-       }
+        let mut workers = Vec::with_capacity(size);
+        for id in 0..size {
+            workers.push(Worker::new(id));
+        }

        Self { workers }
    }

    pub fn excute<F>(&self, f: F)
    where
        F: FnOnce() -> () + Send + 'static,
    {

    }
}

+ struct Worker {
+     id: usize,
+     thread: JoinHandle<()>,
+ }
+ 
+ impl Worker {
+     fn new(id: usize) -> Self {
+         let thread = thread::spawn(|| {});
+         Self { id, thread }
+     }
+ }
```

### チャンネル経由でスレッドにリクエストを送信する

- 次に、`Worker` 構造体内部で実行されているスレッドに、タスクを送り込む機能の実装を進める
- そこで、チャンネルを利用する（cf. 16章）
- すなわち、`ThreadPool` の `excute` メソッドで、引数として受け取ったクロージャを、チャンネル経由で `Worker` 構造体の中のスレッドに送信する機能を以下の方針で実装する：
  
  1. `ThreadPool` はチャンネルの送信側に就く
     - `ThreadPool::new` でチャンネルを生成する
     - `ThreadPool` 構造体は送信機を格納する
  2. 各 `Worker` は、チャンネルの受信側に就く
     - `ThreadPool::new` 内で `Worker::new` される際に受信機を受け取る
     - `Worker::new` は受信機を引数にとり、それを内部のスレッドに渡す
  3. 送受信の対象は、タスクを表す `Job` オブジェクト
  4. `ThreadPool::excute` メソッドは、自インスタンスが格納する送信機を使って `Worker` 内の受信機に `Job` を送り込む
  5. `Worker` は、内部のスレッド内で、チャンネルの受信側をループし、`Job` を受け取ったらそれを実行する

> - 注意点：チャンネル使用時の原則は mpsc (生成者は複数・消費者は一つ) なので、受信側を複数のスレッドに分配する今回のケースでは、`Arc` と `Mutex` の併用が必要になる
>   - cf. 「16.3 状態共有並行性（複数のスレッド間でのメモリ共有）」

- 以下のコードでは、上述の方針のうち、`excute` メソッドと `Worker` 内での `Job` の実行処理以外の部分を実装している：

  ```rs
  use std::{
      sync::{
          mpsc::{self, Receiver, Sender},
          Arc, Mutex,
      },
      thread::{self, JoinHandle},
  };

  pub struct ThreadPool {
      workers: Vec<Worker>,
      sender: Sender<Job>,
  }

  struct Job;

  impl ThreadPool {
      /// Create a new ThreadPool.
      ///
      /// The size is the number of threads in the pool.
      ///
      /// # Panics
      ///
      /// The `new` function will panic if the size is zero.
      pub fn new(size: usize) -> Self {
          assert!(size > 0);

          let (sender, receiver) = mpsc::channel::<Job>();

          let receiver = Arc::new(Mutex::new(receiver));
          let mut workers = Vec::with_capacity(size);
          for id in 0..size {
              let receiver = Arc::clone(&receiver);
              workers.push(Worker::new(id, receiver));
          }
          Self { workers, sender }
      }

      pub fn excute<F>(&self, f: F)
      where
          F: FnOnce() -> () + Send + 'static,
      {
      }
  }

  struct Worker {
      id: usize,
      thread: JoinHandle<()>,
  }

  impl Worker {
      fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
          let thread = thread::spawn(|| {
              receiver;
          });
          Self { id, thread }
      }
  }
  ```

### `excute` メソッドの実装

- 最後に `excute` メソッドを実装する
  - `Job` に、ライブラリの使用者から送られてきたクロージャ（`Thread::excute` の引数として渡ってくる）を格納できるようにする
    - 今回はそのために、ライブラリの使用者から送られてくるクロージャの型に型エイリアスとして `Job` という型名を与えることにする

  - `excute` メソッド内で、`Worker` 内のスレッドへと `Job` を送信するコードを記述する

  - `Worker` 内のスレッドで無限ループを回し、その内部でレシーバで受信を試み続ける
    - 受信に成功したら、受け取ったクロージャを実行する

  ```diff
  use std::{
      sync::{
          mpsc::{self, Receiver, Sender},
          Arc, Mutex,
      },
      thread::{self, JoinHandle},
  };

  pub struct ThreadPool {
      workers: Vec<Worker>,
      sender: Sender<Job>,
  }

  - struct Job;
  + type Job = Box<dyn FnOnce() -> () + Send + 'static>;

  impl ThreadPool {

      // --snip--

      pub fn excute<F>(&self, f: F)
      where
          F: FnOnce() -> () + Send + 'static,
      {
  +         let job = Box::new(f);
  + 
  +         self.sender.send(job).unwrap();
      }
  }

  struct Worker {
      id: usize,
      thread: JoinHandle<()>,
  }

  impl Worker {
      fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
  -       let thread = thread::spawn(|| {
  -           receiver;
  -       });
  +       let thread = thread::spawn(move || loop {
  +           let job = receiver.lock().unwrap().recv().unwrap();
  +
  +           println!("Worker {} got a job; executing.", id);
  +
  +           job();
  +       });
          Self { id, thread }
      }
  }
  ```

- これで、コード自体は完成する
  - 実行して、ためしに、<http://127.0.0.1:7878/sleep> へアクセスして、レスポンスを待っている間に <http://127.0.0.1:7878/> にアクセスすると、前者の画面表示を待たずに後者の画面が表示されることを確かめられる

- なお、`Worker::new` 内のループを `while let` に置き換えたり、loop 内で `unwrap` の代わりに `if let` 式を用いたりすると所有権の問題で期待通りの動作にならないので注意（２敗）
  - ザックリいうと、`while let` や `if let` を用いると `job()` の完了まで 受信機の mutex がロックされたままになるような実装になるのて、他のスレッドが受信機にアクセスできなくなって並行処理がうまくいかない
  - 詳細については本文を熟読して理解すること

```rs
// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);

                job.call_box();
                // この実装だと、ここ↓の `}` で drop されるまで mutex がロックされたままになる
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```