
use hex_literal::hex;
use std::thread;
use zmq::{Context, DEALER, ROUTER};
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

const END_POINT : &str = "tcp://127.0.0.1:7050";

fn main() {
    let context = Context::new();
    let socket = context.socket(ROUTER).unwrap();
    socket.bind(END_POINT).unwrap();

    thread::spawn(||{
        std::thread::sleep(std::time::Duration::from_secs(1));
        client();
    });

    let mut count = 0u64;
    loop {
        let mut received_parts = socket.recv_multipart(0).unwrap();
        let msg_bytes = received_parts.pop().unwrap();
        let zmq_identity = received_parts.pop().unwrap();
        println!(
            "main thread, received from client, #zmq_identity: {:x?}; #msg_bytes: {:x?}",
            zmq_identity,
            msg_bytes
        );
        socket.send_multipart(vec![zmq_identity, msg_bytes.clone(), b"world".to_vec()], 0).unwrap();
        count += 1;
        println!("main loop count = {}", count);
    }
}

fn client() {
    let context = Context::new();
    let socket = context.socket(DEALER).unwrap();
    socket.set_identity( &hex!("1234567890").to_vec() ).unwrap();
    socket.connect(END_POINT).unwrap();

    socket.send("hello", 0).unwrap();
    let mut received_parts = socket.recv_multipart(0).unwrap();
    //println!("client thread, received from server, #received_parts: {:?}", received_parts);
    println!(
        "\t\tclient thread, received from server, #received_parts: {:x?}; {:x?}",
        received_parts.pop().unwrap(),
        received_parts.pop().unwrap()
    );

    socket.send("hi", 0).unwrap();
    received_parts = socket.recv_multipart(0).unwrap();
    //println!("client thread, received from server, #received_parts: {:?}", received_parts);
    println!(
        "\t\tclient thread, received from server, #received_parts: {:x?}; {:x?}",
        received_parts.pop().unwrap(),
        received_parts.pop().unwrap()
    );

    let mut stream = RlpStream::new_list(2);
    stream.append(&"cat").append(&"dog");
    let out = stream.out();
    assert_eq!(out, vec![0xc8, 0x83, b'c', b'a', b't', 0x83, b'd', b'o', b'g']);
    socket.send(out, 0).unwrap();
    received_parts = socket.recv_multipart(0).unwrap();
    //println!("client thread, received from server, #received_parts: {:?}", received_parts);
    println!(
        "\t\tclient thread, received from server, #received_parts: {:x?}; {:x?}",
        received_parts.pop().unwrap(),
        received_parts.pop().unwrap()
    );
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


// RLP-Encode( method(string), id(number), param(rlp-encoded-list) );
pub struct IpcRequest<'a> {
    pub method: String,
    pub id: u64,
    pub params: Rlp<'a>,
}

// if id < 0 , it means an error-code.
pub struct IpcReply<'a> {
    pub id: u64,
    pub result: Rlp<'a>,
}



