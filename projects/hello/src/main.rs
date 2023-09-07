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
        let stream = stream?;
        // 各コネクションごとにスレッドを生成して、その内部で処理を遂行する
        thread::spawn(|| {
            handle_connection(stream).expect("Error at handle_connection");
        });
    }
    Ok(())
}
