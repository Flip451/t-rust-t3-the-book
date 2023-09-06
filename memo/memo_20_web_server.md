# 20 章：最後のプロジェクト：マルチスレッドの Web サーバ

## 目次

- [20 章：最後のプロジェクト：マルチスレッドの Web サーバ](#20-章最後のプロジェクトマルチスレッドの-web-サーバ)
  - [目次](#目次)
  - [20.0 概要](#200-概要)
  - [20.1 シングルスレッドのWebサーバを構築する](#201-シングルスレッドのwebサーバを構築する)
    - [TCP 接続をリッスンする](#tcp-接続をリッスンする)
    - [クライアントからのリクエストを読み込む](#クライアントからのリクエストを読み込む)
    - [HTTPリクエストを詳しく見る](#httpリクエストを詳しく見る)

## 20.0 概要

- この章では the book の総まとめとして、`hello` とブラウザに表示する Web サーバを構築する

    ![web page](https://doc.rust-jp.rs/book-ja/img/trpl20-01.png)

- 以下のようなプランで進める：
  1. TCP と HTTP について少し学ぶ
  2. ソケットで TCP 接続をリッスンする
  3. 少量の HTTP リクエストを構文解析する
  4. 適切な HTTP レスポンスを生成する
  5. スレッドプールでサーバのスループットを強化する

## 20.1 シングルスレッドのWebサーバを構築する

### TCP 接続をリッスンする

- プロジェクトの作成

  ```sh
  cargo new --bin hello
  code hello
  ```

- [`std::net::TcpListener` のドキュメント](https://doc.rust-lang.org/stable/std/net/struct.TcpListener.html#examples) に従って `main.rs` を書き換える

  **`hello/src/main.rs`**

  ```rs
  use std::io::Result;
  use std::net::{TcpListener, TcpStream};

  fn handle_connection(stream: TcpStream) {
    println!("Connection established!");
  }

  fn main() -> Result<()> {
      // TCP リスナーの作成
      let listener = TcpListener::bind("127.0.0.1:7878")?;
      
      // listener.incoming() の返り値のイテレータは一連のストリームを返す
      // 各ストリームは、クライアント・サーバ間の接続に対応する
      // ストリームはスコープを抜けると `drop` 実装の一部として close される
      for stream in listener.incoming() {
          handle_connection(stream?);
      }
      Ok(())
  }
  ```

  - ここで、`listener.incoming` の返り値のイテレータの各要素 `stream` は、各々、クライアント・サーバ間の**接続**に対応する
    - なお、「接続 (connection) 」は、クライアントがサーバに接続し、サーバがレスポンスを生成し、サーバが接続を閉じるというリクエストとレスポンス全体の過程の名前
    - この接続は、`stream` がスコープをぬけると `drop` 処理の一環で閉じられる

    > - 例えば、以下のように、`stream` が一つ生成されるごとに何個目の `stream` が生成されたかを標準出力するようにコードを書き換えて実験する：
    >
    >  ```diff
    >  use std::io::Result;
    >  use std::net::{TcpListener, TcpStream};
    >
    >  fn handle_connection(stream: TcpStream) {
    >      println!("Connection established!");
    >  }
    >
    >  fn main() -> Result<()> {
    >      let listener = TcpListener::bind("127.0.0.1:7878")?;
    >      // listener.incoming() の返り値のイテレータは一連のストリームを返す
    >      // 各ストリームは、クライアント・サーバ間の接続に対応する
    >      
    >  -   for stream in listener.incoming() {
    >  +   for (index, stream) in listener.incoming().enumerate() {
    >  +       println!("{} 個目の stream が生成されました！", index);
    >          handle_connection(stream?);
    >      }
    >      Ok(())
    >  }
    >  ```
    >
    > - この状態で、`cargo run` して、複数の別のターミナルから `nc 127.0.0.1 7878` して TCP 接続すると、その度に `stream` が生成されることが確かめられる

### クライアントからのリクエストを読み込む

- `use std::io::prelude::*;` して、ストリームからの読み書きを定義しているトレイトを利用できるようにする（`stream.read(...)` の部分）
  - `std::io::prelude` で実装されている `TcpStream` の `read` メソッドで、バッファに受信データを書き込む

- `String::from_utf8_lossy`: `&[u8]` を取り、`String` を生成する関数

```diff
- use std::io::Result;
+ use std::io::{prelude::*, Result};
use std::net::{TcpListener, TcpStream};

- fn handle_connection(stream: TcpStream) {
-   println!("Connection established!");
- }

+ fn handle_connection(mut stream: TcpStream) -> Result<()>{
+     println!("Connection established!");
+ 
+     // スタック上にクライアントから受信したデータを保持する領域（バッファ）を確保
+     let mut buffer = [0; 1024];
+     
+     // クライアントからの受信内容をバッファに読み込む
+     stream.read(&mut buffer)?;
+ 
+     // クライアントからの受信内容を標準出力に表示
+     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
+ 
+     Ok(())
+ }

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    // listener.incoming() の返り値のイテレータは一連のストリームを返す
    // 各ストリームは、クライアント・サーバ間の接続に対応する
    // ストリームはスコープを抜けると `drop` 実装の一部として close される
    for (index, stream) in listener.incoming().enumerate() {
        println!("{} 個目の stream が生成されました！", index);
-       handle_connection(stream?);
+       handle_connection(stream?)?;
    }
    Ok(())
}
```

### HTTPリクエストを詳しく見る

- 先ほどのコードを `cargo run` で実行してブラウザから 127.0.0.1:7878 に接続してみると、標準出力には以下のように表示される：

  ```sh
  0 個目の stream が生成されました！
  Connection established!
  Request: GET / HTTP/1.1
  Host: 127.0.0.1:7878
  Connection: keep-alive
  Cache-Control: max-age=0
  sec-ch-ua: "Chromium";v="116", "Not)A;Brand";v="24", "Google Chrome";v="116"
  sec-ch-ua-mobile: ?0
  sec-ch-ua-platform: "Windows"
  Upgrade-Insecure-Requests: 1
  User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36
  Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
  Sec-Fetch-Site: none
  Sec-Fetch-Mode: navigate
  Sec-Fetch-User: ?1
  Sec-Fetch-Dest: document
  Accept-Encoding: gzip, deflate, br
  Accept-Language: ja,en-US;q=0.9,en;q=0.8


  1 個目の stream が生成されました！
  Connection established!
  Request: GET / HTTP/1.1
  Host: 127.0.0.1:7878
  Connection: keep-alive
  Cache-Control: max-age=0
  sec-ch-ua: "Chromium";v="116", "Not)A;Brand";v="24", "Google Chrome";v="116"
  sec-ch-ua-mobile: ?0
  sec-ch-ua-platform: "Windows"
  Upgrade-Insecure-Requests: 1
  User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36
  Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
  Sec-Fetch-Site: none
  Sec-Fetch-Mode: navigate
  Sec-Fetch-User: ?1
  Sec-Fetch-Dest: document
  Accept-Encoding: gzip, deflate, br
  Accept-Language: ja,en-US;q=0.9,en;q=0.8


  2 個目の stream が生成されました！
  Connection established!
  Request: GET / HTTP/1.1
  Host: 127.0.0.1:7878
  Connection: keep-alive
  Cache-Control: max-age=0
  sec-ch-ua: "Chromium";v="116", "Not)A;Brand";v="24", "Google Chrome";v="116"
  sec-ch-ua-mobile: ?0
  sec-ch-ua-platform: "Windows"
  Upgrade-Insecure-Requests: 1
  User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36
  Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
  Sec-Fetch-Site: none
  Sec-Fetch-Mode: navigate
  Sec-Fetch-User: ?1
  Sec-Fetch-Dest: document
  Accept-Encoding: gzip, deflate, br
  Accept-Language: ja,en-US;q=0.9,en;q=0.8


  3 個目の stream が生成されました！
  Connection established!

  ```