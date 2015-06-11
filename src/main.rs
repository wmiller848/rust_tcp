extern crate rustc_serialize;
use rustc_serialize::{Decodable, Encodable, json};

extern crate bincode;
use bincode::SizeLimit;

use std::str;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(RustcDecodable, RustcEncodable)]
pub struct JsonDataStruct  {
    foo: String,
    other_foo: String,
}

fn handle_client(mut stream: TcpStream) {
    // ...
    let buf = &mut [0; 128];

    match stream.read(buf) {
        Ok(size) => {
            println!("Size {}", size);

            for x in 0..size {
                print!("{:#X} ", buf[x]);
            };
            println!("");

            let s = match str::from_utf8(&buf[..size]) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            println!("{:?}", s);

            let decoded: JsonDataStruct = json::decode(s).unwrap();
            println!("{:?}", decoded.foo);
            println!("{:?}", decoded.other_foo);
            let encoded: Vec<u8> = bincode::encode(&decoded, SizeLimit::Infinite).unwrap();
            for x in 0..encoded.len() {
                print!("{:#X} ", encoded[x]);
            };
            println!("");

            let decoded: JsonDataStruct = bincode::decode(&encoded[..]).unwrap();
            println!("{:?}", decoded.foo);
            println!("{:?}", decoded.other_foo);
        }
        Err(e) => {
            println!("{:?}", e);
        }
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
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}
