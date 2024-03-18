// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::io::Write;
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut st) => {
                st.write("+PONG\r\n".as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {e}")
            }
        }
    }
}
