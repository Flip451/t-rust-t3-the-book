extern crate hello;

use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::{
    fs,
    io::{prelude::*, Result},
};

use hello::ThreadPool;

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
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream?;

        pool.excute(|| {
            handle_connection(stream).expect("Error at handle_connection");
        })
    }

    Ok(())
}
