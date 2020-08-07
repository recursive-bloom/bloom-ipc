
mod zmq_server;
mod zmq_client;
mod nanomsg_server;
mod nanomsg_client;

use hex_literal::hex;
use std::thread;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

const END_POINT1 : &'static str = "tcp://127.0.0.1:7050";
const END_POINT2: &'static str = "tcp://127.0.0.1:7050";
// const END_POINT2: &'static str = "ipc:///tmp/reqrep_example.ipc";

fn main_zmq() {
    thread::spawn(||{
        std::thread::sleep(std::time::Duration::from_secs(1));
        zmq_client::client(END_POINT1);
    });
    zmq_server::server(END_POINT1);
}

fn main_nanomsg(proc_type : &str, client_id : String) {
    match proc_type {
        "client" => nanomsg_client::client(client_id, END_POINT2),
        "server" => nanomsg_server::server(END_POINT2),
        _ => usage(),
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        return usage();
    }
    match args[1].as_ref() {
        "nano" => main_nanomsg(args[2].as_ref(), args[3].to_string()),
        "zmq" =>  main_zmq(),
        _ => usage(),
    }
}

fn usage() {
    println!("Usage 0: bloom-ipc zmq");
    println!("Usage 1: bloom-ipc nano server start");
    println!("Usage 2: bloom-ipc nano client Alice");
    println!("Usage 3: bloom-ipc nano client Bob");

}

#[test]
fn test_rlp_encode_decode() {

    let data = hex!("f84d0589010efbef67941f79b2a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");
    let rlp = Rlp::new(&data);
    println!("{}", rlp);
    // ["0x05", "0x010efbef67941f79b2", "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421", "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"]

    let mut stream = RlpStream::new_list(2);
    stream.append(&"cat").append(&"dog");
    let out = stream.out();
    let rlp = Rlp::new(&out);
    println!("{}", rlp);
    // ["0x636174", "0x646f67"]

    let mut stream = RlpStream::new_list(2);
    stream.begin_list(2).append(&"cat").append(&"dog");
    stream.append(&"");
    println!("len {}", &stream.len());
    let out = stream.out();
    assert_eq!(out, vec![0xca, 0xc8, 0x83, b'c', b'a', b't', 0x83, b'd', b'o', b'g', 0x80]);
    let rlp = Rlp::new(&out);
    println!("{}", rlp);
    // [["0x636174", "0x646f67"], "0x"]

    let mut stream1 = RlpStream::new();
    stream1.begin_unbounded_list();
    stream1.append(&"cat").append(&"dog");
    stream1.append(&"");
    stream1.append(&out);
    stream1.finalize_unbounded_list();
    println!("len {}", &stream1.len());
    let out = stream1.out();
    let rlp = Rlp::new(&out);
    println!("{}", rlp);
    // [["0x636174", "0x646f67"], "0x"]
}

