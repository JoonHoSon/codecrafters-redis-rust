// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    // *2\r\n$4\r\necho\r\n$3\r\nhey\r\n
    // ["echo", "hey"]

    for stream in listener.incoming() {
        thread::spawn(move || match stream {
            Ok(mut st) => {
                let mut buffer: [u8; 1024] = [0; 1024];

                while let Ok(_) = st.read(&mut buffer) {
                    // * array
                    // + simple string
                    // $ bulk string
                    // windows ref : https://stackoverflow.com/a/35907071
                    let mut data_type: char = '\0';
                    let mut payload: Vec<u8> = vec![];
                    let mut crlf: Vec<u8> = Vec::with_capacity(2);
                    let mut data_size = 0;

                    for (idx, b) in buffer.iter_mut().enumerate() {
                        println!("char -> {b:?}");

                        if 0u8 == *b {
                            println!("terminate loop");

                            break;
                        }

                        // first byte
                        if '\0' == data_type {
                            data_type = *b as char;
                            println!("check data type");

                            continue;
                        }

                        if 13u8 == *b || 10u8 == *b {
                            // \r or \n
                            println!("find carriage return");
                            crlf.push(*b);

                            continue;
                        }

                        if crlf.len() == 2 {
                            // \r\n
                            println!("redis protocol terminate mark");
                            let mut tmp = String::new();

                            if '*' == data_type && data_size == 0 {
                                payload.iter().for_each(|&p| tmp.push(p as char));

                                println!("tmp ===> {tmp:?}");

                                data_size = tmp.parse::<u32>().unwrap();

                                println!("data size : {data_size:?}");
                            } else if '$' == data_type && data_size == 0 {
                            }

                            payload.clear(); // clear payload

                            continue;
                        }

                        payload.push(*b);
                    }

                    st.write("+PONG\r\n".as_bytes()).unwrap();
                }

                st.flush().unwrap();
            }
            Err(e) => {
                println!("error: {e}")
            }
        });
    }
}
