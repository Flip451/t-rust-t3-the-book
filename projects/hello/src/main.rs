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
    if buffer.starts_with(get_request) {
        // index.html を開く
        let mut file = File::open("index.html")?;
    
        // ファイルの内容を String に読み出し
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    
        // レスポンスを返却
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes())?;
        stream.flush()?;
    } else {
        // / への GET メソッドのリクエスト以外はエラーを返す
        // 404 エラーを表すステータスラインを構成
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        
        // 404 エラーの時に返す HTML を取得して String に保持
        let mut file = File::open("404.html")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // レスポンス内容を構成
        let response = format!("{}{}", status_line, contents);
        // レスポンスを返却
        stream.write(response.as_bytes())?;
        stream.flush()?;
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
