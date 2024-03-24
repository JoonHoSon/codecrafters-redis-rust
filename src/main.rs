// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

const BUFFER_SIZE: usize = 1024;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let mut map: HashMap<String, String> = HashMap::new();
    let (tx, rx) = mpsc::channel();

    // *2\r\n$4\r\necho\r\n$3\r\nhey\r\n
    // ["echo", "hey"]

    for stream in listener.incoming() {
        thread::spawn(move || match stream {
            Ok(mut st) => {
                handle_request(&mut st, &tx);
            }
            Err(e) => {
                println!("error: {e}")
            }
        });
    }
}

fn handle_request(stream: &mut TcpStream, sender: &Sender<Vec<String>>) {
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

        println!("read ==> {read:#?}");

        let converted: String = String::from_utf8(read).unwrap();
        let items = converted.split("\r\n");
        let mut payload: Vec<String> = vec![];

        for (idx, item) in items.enumerate() {
            if idx == 0 && !item.is_empty() {
                println!("first item : {item:?}");
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

        println!("payload ====> {payload:?}");

        if payload.len() == 0 {
            break;
        }

        if &payload[0] == "ping" {
            stream.write_all(b"+PONG\r\n").unwrap();
        } else if &payload[0] == "echo" {
            let response_size = payload[1].len();
            let response = format!("${}\r\n{}\r\n", response_size, payload[1]);

            stream.write_all(response.as_bytes()).unwrap();
        } else if &payload[0] == "set" && payload.len() == 3 {
            map.insert(payload[1].to_owned(), payload[2].to_owned());

            stream.write_all(b"+OK\r\n").unwrap();
        } else if &payload[0] == "get" && payload.len() == 2 {
            println!("map ==> {map:#?}");
            stream
                .write_all(map.get(&payload[1]).unwrap().as_bytes())
                .unwrap();
        }

        stream.flush().unwrap();
    }
}
