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
                // st.write("+PONG\r\n".as_bytes()).unwrap();
                st.write("+PONG\r\n+PONG".as_bytes()).unwrap();
                // for stream in listener.incoming() {
                //     match stream {
                //         Ok(mut st) => {
                //             let mut buffer = vec![0; 4096];
                //
                //             while let Ok(read) = st.read(&mut buffer) {
                //                 if read == 0 {
                //                     break;
                //                 }
                //
                //                 let received = String::from_utf8_lossy(buffer.as_slice());
                //
                //
                //             }
                //         }
                //         Err(e) => {
                //             println!("error: {e}")
                //         }
                //     }
                // }
            }
            Err(e) => {
                println!("error: {e}")
            }
        }
    }
}
