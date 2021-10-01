use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

pub fn connect(address: &str) -> Result<(), failure::Error> {
    // connect()を呼べば3way handshakeを勝手にやってくれる
    let mut stream = TcpStream::connect(address)?;

    loop {
        let mut input = String::new();
        // 標準入力から1行の文字列を読み取る
        io::stdin().read_line(&mut input)?;
        // エンコードした文字列を送信する
        stream.write_all(input.as_bytes())?;

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;
        print!("{}", str::from_utf8(&buffer)?);
    }
}
