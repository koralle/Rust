use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

use log::{debug, error};

pub fn serve(address: &str) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(address)?;

    loop {
        // accept()を実行して3-way handshakeによるコネクションを確立済みのTCPソケットを返却する
        // TCPソケットが生成されるまでスレッドをブロックする
        let (stream, _) = listener.accept()?;

        // spawnは、一般的に子プロセスを作成する機能を指す。
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
}

fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);

    let mut buffer = [0u8; 1024];

    loop {
        // TCPソケットからデータを読み取ってbufferに格納する
        // そのためデータの格納先のbufferはmutableである必要がある
        let nbytes = stream.read(&mut buffer)?;

        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }

        print!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes])?;
    }
}
