// use std::string::String;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;


fn handle_client(mut stream: TcpStream) {
    // ...
    let buf = &mut [0; 128];

    match stream.read(buf) {
        Ok(size) => {
            // let slice: &[u8] = &*buf;
            println!("Size {}", size);
            println!("{:?}", &*buf as &[u8]);
        }
        Err(e) => { /* connection failed */ }
    }
}

fn main() {
    let interface = "127.0.0.1";
    let port = "8080";

    let mut server_info = String::new();
    server_info.push_str(interface);
    server_info.push_str(":");
    server_info.push_str(port);
    println!("Starting Server on port {}", server_info);

    let listener = TcpListener::bind(&server_info as &str).unwrap();
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
    // close the socket server
    drop(listener);
}
