# 20 章：最後のプロジェクト：マルチスレッドの Web サーバ

## 目次

- [20 章：最後のプロジェクト：マルチスレッドの Web サーバ](#20-章最後のプロジェクトマルチスレッドの-web-サーバ)
  - [目次](#目次)
  - [20.0 概要](#200-概要)
  - [20.1 シングルスレッドのWebサーバを構築する](#201-シングルスレッドのwebサーバを構築する)
    - [TCP 接続をリッスンする](#tcp-接続をリッスンする)
    - [クライアントからのリクエストを読み込む](#クライアントからのリクエストを読み込む)
    - [HTTPリクエストを詳しく見る](#httpリクエストを詳しく見る)
    - [レスポンスを記述する](#レスポンスを記述する)
    - [HTML を返す](#html-を返す)
    - [リクエストにバリデーションをかけて選択的にレスポンスを返す](#リクエストにバリデーションをかけて選択的にレスポンスを返す)
    - [リファクタリング](#リファクタリング)
    - [英語版の the book 準拠の実装](#英語版の-the-book-準拠の実装)

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

- 次に、クライアントからのリクエスト内容を読み込む処理を追加する
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

- ここで、`cargo run` でサーバを起動し、`nc 127.0.0.1 7878` でサーバーに接続すると、バイト列を送信できることを確認できる：

  ターミナル１（サーバ側）

  ```sh
  cargo run
  ```
  
  ターミナル２（クライアント側）

  ```sh
  $ nc 127.0.0.1 7878
  hello, world!
  ```

  ターミナル１（サーバ側）

  ```sh
  $ cargo run
  0 個目の stream が生成されました！
  Connection established!
  Request: hello, world!

  ```

### HTTPリクエストを詳しく見る

- 先ほどのコードを `cargo run` で実行してブラウザから <http://127.0.0.1:7878> に接続してみると、標準出力には以下のように表示される：

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

- 上の内容から、ブラウザは三度 `HTTP/1.1` で URI が `/` の文書を GET メソッドで要求していることがわかる
  - （各々のリクエストの `GET / HTTP/1.1` 以降の内容はヘッダーを表す）
  - なお、HTTP のフォーマットは以下の通り：

    ```txt
    Method Request-URI HTTP-Version CRLF
    headers CRLF
    message-body
    ```

### レスポンスを記述する

- 次に、サーバ側からクライアント側にレスポンスを返す
  - HTTP のレスポンスのフォーマットは以下の通り：

    ```txt
    HTTP-Version Status-Code Reason-Phrase CRLF
    headers CRLF
    message-body
    ```

  - `TcpStream` の `write` メソッドは、バイト列を接続相手に送信する
    - なお、`response.as_byte()` として、文字列をバイト列に変換してから送信している
  - `TcpStream` の `flush` メソッドは、コンテンツがすべて宛先に届くように保証してくれる

  ```diff
  use std::io::{prelude::*, Result};
  use std::net::{TcpListener, TcpStream};

  fn handle_connection(mut stream: TcpStream) -> Result<()> {
      println!("Connection established!");

      // スタック上にクライアントから受信したデータを保持する領域（バッファ）を確保
      let mut buffer = [0; 1024];

      // クライアントからの受信内容をバッファに読み込む
      stream.read(&mut buffer)?;

  -   // クライアントからの受信内容を標準出力に表示
  -   println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

  +   // レスポンスを返却
  +   let response = "HTTP/1.1 200 OK\r\n\r\n";
  +   stream.write(response.as_bytes())?;
  +   stream.flush()?;

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

### HTML を返す

- HTML を返す機能を実装する
- プロジェクトのルートディレクトリに `hello.html` を作成する
  
  ```html
  <!DOCTYPE html>
  <html lang="en">
  <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Document</title>
  </head>
  <body>
      <div>
          <h1>Hello</h1>
          <div>world!</div>
      </div>
  </body>
  </html>
  ```

- このファイルの内容をサーバから返すように `main.rs` を編集する

  ```diff
  use std::net::{TcpListener, TcpStream};
  - use std::io::{prelude::*, Result};
  + use std::{
  +    fs::File,
  +    io::{prelude::*, Result},
  + };

  fn handle_connection(mut stream: TcpStream) -> Result<()> {
      println!("Connection established!");

      // スタック上にクライアントから受信したデータを保持する領域（バッファ）を確保
      let mut buffer = [0; 1024];

      // クライアントからの受信内容をバッファに読み込む
      stream.read(&mut buffer)?;

      // // クライアントからの受信内容を標準出力に表示
      // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

  +   // index.html を開く
  +   let mut file = File::open("index.html")?;
  + 
  +   // ファイルの内容を String に読み出し
  +   let mut contents = String::new();
  +   file.read_to_string(&mut contents)?;

      // レスポンスを返却
  -   let response = "HTTP/1.1 200 OK\r\n\r\n";
  +   let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
      stream.write(response.as_bytes())?;
      stream.flush()?;

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

- `cargo run` してブラウザから <http://127.0.0.1:7878> にアクセスすると、期待通りに html の内容がブラウザ上に表示されていることを確認できる

### リクエストにバリデーションをかけて選択的にレスポンスを返す

- リクエストの内容を読み取って、HTML を返却したり、エラーを返したりする機能を追加する

  **`src/main.rs`**

  ```diff
  use std::net::{TcpListener, TcpStream};
  use std::{
      fs::File,
      io::{prelude::*, Result},
  };

  fn handle_connection(mut stream: TcpStream) -> Result<()> {
      println!("Connection established!");

      // スタック上にクライアントから受信したデータを保持する領域（バッファ）を確保
      let mut buffer = [0; 1024];

      // クライアントからの受信内容をバッファに読み込む
      stream.read(&mut buffer)?;

      // // クライアントからの受信内容を標準出力に表示
      // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

  +   // 受信内容の最初の行が GET リクエストのフォーマットと一致しているか否かで分岐
  +   let get_request = b"GET / HTTP/1.1\r\n";
  +   if buffer.starts_with(get_request) {
          // index.html を開く
          let mut file = File::open("index.html")?;
      
          // ファイルの内容を String に読み出し
          let mut contents = String::new();
          file.read_to_string(&mut contents)?;
      
          // レスポンスを返却
          let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
          stream.write(response.as_bytes())?;
          stream.flush()?;
  +   } else {
  +       // / への GET メソッドのリクエスト以外はエラーを返す
  +       // 404 エラーを表すステータスラインを構成
  +       let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
  +       
  +       // 404 エラーの時に返す HTML を取得して String に保持
  +       let mut file = File::open("404.html")?;
  +       let mut contents = String::new();
  +       file.read_to_string(&mut contents)?;
  + 
  +       // レスポンス内容を構成
  +       let response = format!("{}{}", status_line, contents);
  +       // レスポンスを返却
  +       stream.write(response.as_bytes())?;
  +       stream.flush()?;
  +   }

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

  **`404.html`**

  ```html
  <!DOCTYPE html>
  <html lang="en">
  <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Document</title>
  </head>
  <body>
      <h1>Oops!</h1>
      <p>Sorry, I don't know what you're asking for.</p>
  </body>
  </html>
  ```

### リファクタリング

- 以下のように書き換えた方がコードの繰り返しが少ない

```rs
use std::net::{TcpListener, TcpStream};
use std::{
    fs::File,
    io::{prelude::*, Result},
};

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    println!("Connection established!");

    // スタック上にクライアントから受信したデータを保持する領域（バッファ）を確保
    let mut buffer = [0; 1024];

    // クライアントからの受信内容をバッファに読み込む
    stream.read(&mut buffer)?;

    // // クライアントからの受信内容を標準出力に表示
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 受信内容の最初の行が GET リクエストのフォーマットと一致しているか否かで分岐
    let get_request = b"GET / HTTP/1.1\r\n";
    let (file_path, status_line) = if buffer.starts_with(get_request) {
        ("index.html", "HTTP/1.1 200 OK\r\n\r\n")
    } else {
        ("404.html", "HTTP/1.1 404 NOT FOUND\r\n\r\n")
    };

    // index.html を開く
    let mut file = File::open(file_path)?;

    // ファイルの内容を String に読み出し
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // レスポンスを返却
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes())?;
    stream.flush()?;

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

### 英語版の the book 準拠の実装

- `BufReader`: バッファの利用をラップする構造体
  - `lines` メソッドで reader の各行に渡るイテレータを取得可能
- `let contents = fs::read_to_string(file_path)?;`: 以下の処理を一行で記述
  
  ```rs
  let mut file = File::open(file_path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  ```

```rs
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::{
    fs,
    io::{prelude::*, Result},
};

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    // バッファの最初の行を取得
    if let Some(request_line) = buf_reader.lines().next() {
        let request_line = request_line?;

        // 受信内容の最初の行が GET リクエストのフォーマットと一致しているか否かで分岐
        let (file_path, status_line) = if request_line == "GET / HTTP/1.1" {
            ("index.html", "HTTP/1.1 200 OK")
        } else {
            ("404.html", "HTTP/1.1 404 NOT FOUND")
        };

        // index.html を開く
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
