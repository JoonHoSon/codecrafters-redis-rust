// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

const CRLF: &'static [u8; 2] = &b"\r\n";
const CRLF_SIZE: usize = 2;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    // *2\r\n$4\r\necho\r\n$3\r\nhey\r\n
    // ["echo", "hey"]

    for stream in listener.incoming() {
        thread::spawn(move || match stream {
            Ok(mut st) => {
                let mut buffer: [u8; 1024] = [0; 1024];
                let mut read: Vec<u8> = vec![];

                loop {
                    // read all bytes
                    let rr = st.read(&mut buffer).expect("Stream read fail!");

                    if rr == 0 {
                        break;
                    }

                    read.append(&mut buffer.to_vec());

                    if read.len() == 1024 && &read[read.len() - 2..] == &[0, 0] {
                        break; // terminate empty bytes
                    }
                }

                if read.len() == 0 {
                    return;
                }

                let payload: String = String::from_utf8(buffer.to_vec()).unwrap();

                println!("payload : {payload:?}");

                // ping or echo
                if buffer[0] == b'*' {
                    // 2\r\n$4\r\necho\r\n$3\r\nhey\r\n
                    let crlf_pos = crlf_pos(&buffer[1..]);
                    let size = parse_data_size(&buffer[1..]);
                    println!("crlf_pos ====> {crlf_pos:?}");
                    println!("size : {size:?}");
                    let response: Vec<String> = vec![];

                    for i in 1..=size {
                        let rst = parse_string(&buffer[crlf_pos + CRLF_SIZE + 1 + size..], i);

                        println!("rst : {rst:?}");
                    }
                }

                st.write_all(payload.as_bytes()).expect("write fail!");
            }
            Err(e) => {
                println!("error: {e}")
            }
        });
    }
}

fn parse_string(buffer: &[u8], length: usize) -> String {
    String::from_utf8(buffer[..=length].to_vec()).unwrap()
}

fn parse_data_size(buffer: &[u8]) -> usize {
    // $4\r\n
    let start = 0;
    let pos = crlf_pos(buffer);

    let dd = String::from_utf8(buffer.to_vec()).unwrap();

    println!("parse_data_size buffer : {dd:?}");
    println!("parse_data_size pos    : {pos:?}");

    let raw_size = buffer[start..pos].to_vec();

    println!("raw_size : {raw_size:?}");
    let size = String::from_utf8(raw_size)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    println!("parse_data_size result : {size:?}");

    size
}

fn crlf_pos(buffer: &[u8]) -> usize {
    buffer.windows(CRLF_SIZE).position(|w| w == CRLF).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_string;

    //#[test]
    fn parse_string_test() {
        //let raw_data = b"$4\r\necho\r\n$5\r\nhello";
        //let (result, start) = parse_string(raw_data, 0);

        //assert_eq!("echo", result);
        //assert_eq!(10, start);
        //assert_eq!(&[13u8, 10u8], CRLF);

        //let (result, start) = parse_string(&raw_data[start..], start);

        // TODO(joonho): 2024-03-21 parse_string 수정 후 다시 테스트 필요
        //assert_eq!("hello", result);
        //assert_eq!(raw_data.len(), start);
    }
}
