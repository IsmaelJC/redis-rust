#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut buf: [u8; 512] = [0; 512];

                loop {
                    match stream.read(&mut buf) {
                        Ok(num_bytes) if num_bytes > 0 => {
                            println!("Received {} bytes from client", num_bytes);
                            stream.write(b"+PONG\r\n").unwrap_or_else(|_| {
                                println!("Error writing to TCP stream");
                                0
                            });
                        }
                        _ => break,
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
