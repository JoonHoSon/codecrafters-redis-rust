// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

const CRLF: &[u8; 2] = b"\r\n";
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
                    let rr = st.read(&mut buffer).expect("Stream read fail!");
                    
                    if rr == 0 {
                        break;
                    }

                    read.append(&mut buffer.to_vec());
                }

                while let Ok(r) = st.read(&mut buffer) {
                    if r == 0 {
                        
                    }
                    // * array
                    // + simple string
                    // $ bulk string
                    // windows ref : https://stackoverflow.com/a/35907071
                    let mut pos: usize = 0;
                    let mut data_type: char = buffer[0] as char;
                    let mut payload: String = String::new();
                    let mut data_size: usize = 0;
                    let mut data: Vec<String> = vec![];
                    let mut size_value: Vec<u8> = vec![];
                    let mut start = 0;

                    if '*' == data_type {
                        // array
                        pos = buffer.windows(CRLF_SIZE).position(|w| w == CRLF).unwrap(); // \r\n
                        start = 1;
                        size_value = buffer[start..pos].to_vec(); // 2
                        data_size = String::from_utf8(size_value)
                            .unwrap()
                            .parse::<usize>()
                            .unwrap();

                        data = Vec::with_capacity(data_size);
                        start = pos + CRLF_SIZE; // 2\r\n

                        if '$' == buffer[start] as char {
                            start += 1;
                            pos = buffer[start..]
                                .windows(CRLF_SIZE)
                                .position(|w| w == CRLF)
                                .unwrap(); // \r\n after $
                            size_value = buffer[start..pos].to_vec();
                            data_size = String::from_utf8(size_value)
                                .unwrap()
                                .parse::<usize>()
                                .unwrap();
                            start += pos;

                            data.push(String::from_utf8(buffer[start..data_size].to_vec()).unwrap())
                        }
                    }

                    //     for (_, b) in buffer.iter_mut().enumerate() {
                    //         println!("char -> {b:?}");
                    //
                    //         if 0u8 == *b {
                    //             println!("terminate loop");
                    //
                    //             break;
                    //         }
                    //
                    //         // first byte
                    //         if '\0' == data_type {
                    //             data_type = *b as char;
                    //             println!("check data type : {data_type:?}");
                    //
                    //             continue;
                    //         }
                    //
                    //         if 13u8 == *b || 10u8 == *b {
                    //             // \r or \n
                    //             println!("find carriage return");
                    //             crlf.push(*b);
                    //
                    //             continue;
                    //         }
                    //
                    //         if crlf.len() == 2 {
                    //             // \r\n
                    //             let tmp = String::from_utf8(payload.clone()).unwrap();
                    //
                    //             if ('*' == data_type || '$' == data_type) && data_size == 0 {
                    //                 data_size = tmp.parse::<u32>().unwrap();
                    //
                    //                 println!("============= data size : {data_size:?}");
                    //
                    //                 if '*' == data_type {
                    //                     data = Vec::with_capacity(data_size as usize);
                    //                 }
                    //
                    //                 data_type = '\0'; // reset data type
                    //             } else {
                    //                 data.push(tmp);
                    //
                    //                 println!("============= collected data : {data:?}");
                    //             }
                    //
                    //             payload.clear(); // clear payload
                    //
                    //             continue;
                    //         }
                    //
                    //         payload.push(*b);
                    //     }
                    //
                    //     st.write("+PONG\r\n".as_bytes()).unwrap();
                    // }
                    //
                    // st.flush().unwrap();
                    st.flush().unwrap();
                }
            }
            Err(e) => {
                println!("error: {e}")
            }
        });
    }
}

fn parse_string(buffer: &[u8], before_start: usize) -> (String, usize) {
    // $4\r\necho\r\n
    let mut start = 1;
    // find \r\n position
    let pos = buffer[start..]
        .windows(CRLF_SIZE)
        .position(|w| w == CRLF)
        .unwrap()
        + 1; // start '4'

    println!("{:?}", String::from_utf8(buffer.to_vec()).unwrap());
    println!("buffer : {buffer:?}");
    println!("start : {start:?}, pos : {pos:?}");

    let string_length = String::from_utf8(buffer[start..pos].to_vec())
        .unwrap()
        .parse::<usize>()
        .unwrap();
    start += CRLF_SIZE + 1;

    println!("string_length : {string_length:?}");
    println!("after start : {start:?}");

    return (
        String::from_utf8(buffer[start..=(string_length + 1 + pos)].to_vec()).unwrap(),
        start + string_length + pos + before_start,
    );
}

#[cfg(test)]
mod tests {
    use crate::parse_string;

    #[test]
    fn parse_string_test() {
        let raw_data = b"$4\r\necho\r\n$5\r\nhello";
        let (result, start) = parse_string(raw_data, 0);

        assert_eq!("echo", result);
        assert_eq!(10, start);

        let (result, start) = parse_string(&raw_data[start..], start);

        assert_eq!("hello", result);
        assert_eq!(raw_data.len(), start);
    }
}
