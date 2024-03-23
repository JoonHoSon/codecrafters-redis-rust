// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

const BUFFER_SIZE: usize = 1024;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    // *2\r\n$4\r\necho\r\n$3\r\nhey\r\n
    // ["echo", "hey"]

    for stream in listener.incoming() {
        thread::spawn(move || match stream {
            Ok(mut st) => {
                let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                let mut read: Vec<u8> = vec![];

                loop {
                    // read all bytes
                    let received = st.read(&mut buffer).expect("Stream read fail!");

                    read.extend_from_slice(&buffer[..received]);

                    if received < BUFFER_SIZE {
                        break;
                    }
                }

                let converted: String = String::from_utf8(read).unwrap();
                let items = converted.split("\r\n");
                let mut payload: Vec<String> = vec![];

                for (idx, item) in items.enumerate() {
                    if idx == 0 {
                        payload = Vec::with_capacity(item[1..].parse::<usize>().unwrap());

                        continue;
                    } else if item.is_empty() {
                        continue;
                    }

                    let first_bytes = &item.as_bytes()[0];

                    if first_bytes != &b'*' && first_bytes != &b'$' {
                        payload.push(item.to_owned());
                    }
                }

                if payload.len() == 1 && &payload[0] == "ping" {
                    st.write_all(b"+PONG\r\n").unwrap();
                } else if payload.len() == 2 && &payload[0] == "echo" {
                    let response_size = payload[1].len();
                    let response = format!("${}\r\n{}\r\n", response_size, payload[1]);

                    st.write_all(response.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {e}")
            }
        });
    }
}
