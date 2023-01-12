// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                loop {
                    let mut buf = [0; 5];
                    let n = stream.read(&mut buf).unwrap();

                    if n == 0 {
                        println!("Connection closed!!");
                        break;
                    }

                    let command = String::from_utf8_lossy(&buf[0..n]);

                    if command.starts_with("ping") {
                        let mut res = "+pong\r\n".as_bytes();
                        stream.write(&mut res).unwrap();
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
