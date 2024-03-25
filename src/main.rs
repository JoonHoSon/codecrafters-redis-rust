// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 1024;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let map = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    //let (tx, rx) = mpsc::channel();

    // *2\r\n$4\r\necho\r\n$3\r\nhey\r\n
    // ["echo", "hey"]

    for stream in listener.incoming() {
        match stream {
            Ok(mut st) => {
                println!("peer addr : {:#?}", st.peer_addr().unwrap());

                let mut mm = Arc::clone(&map);

                thread::spawn(move || handle_request(&mut st, &mut mm));
            },
            Err(e) => {
                println!("error: {e}");
            }
        }
    }
}

fn handle_request(stream: &mut TcpStream, map: &Arc<Mutex<HashMap<String, String>>>) {
    loop {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let mut read: Vec<u8> = vec![];

        loop {
            let received = stream.read(&mut buffer).expect("Stream read fail!");

            read.extend_from_slice(&buffer[..received]);

            if received < BUFFER_SIZE {
                break;
            }
        }

        let converted: String = String::from_utf8(read).unwrap();
        let items = converted.split("\r\n");
        let mut payload: Vec<String> = vec![];
        // let client_id = &stream.peer_addr().unwrap();

        for (idx, item) in items.enumerate() {
            if idx == 0 && !item.is_empty() {
                payload = Vec::with_capacity(item[1..].parse::<usize>().unwrap());

                continue;
            }

            if item.is_empty() {
                continue;
            }

            let first_bytes = &item.as_bytes()[0];

            if first_bytes != &b'*' && first_bytes != &b'$' {
                payload.push(item.to_owned());
            }
        }

        if payload.len() == 0 {
            break;
        }

        println!("payload : {payload:?}");

        if &payload[0] == "ping" {
            stream.write_all(b"+PONG\r\n").unwrap();
        } else if &payload[0] == "echo" {
            let response_size = payload[1].len();
            let response = format!("${}\r\n{}\r\n", response_size, payload[1]);

            stream.write_all(response.as_bytes()).unwrap();
        } else if &payload[0] == "set" && payload.len() == 3 {
            let mut mm = map.lock().unwrap();

            mm.insert(payload[1].to_owned(), payload[2].to_owned());
            stream.write_all(b"+OK\r\n").unwrap();
        } else if &payload[0] == "get" && payload.len() == 2 {
            let mm = map.lock().unwrap();
            let data = mm.get(&payload[1]).unwrap();
            let response = format!("${}\r\n{}\r\n", data.len(), data);

            stream.write_all(response.as_bytes()).unwrap();
        }

        stream.flush().unwrap();
    }
}
