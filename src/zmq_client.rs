
use hex_literal::hex;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use zmq::{Context, DEALER, ROUTER};

pub fn client(end_point : &str) {
    let context = Context::new();
    let socket = context.socket(DEALER).unwrap();
    socket.set_identity( &hex!("1234567890").to_vec() ).unwrap();
    socket.connect(end_point).unwrap();

    socket.send("hello", 0).unwrap();
    let mut received_parts = socket.recv_multipart(0).unwrap();
    //println!("client thread, received from server, #received_parts: {:?}", received_parts);
    println!(
        "\tclient thread, received from server, #received_parts: {:x?}; {:x?}",
        received_parts.pop().unwrap(),
        received_parts.pop().unwrap()
    );

    socket.send("hi", 0).unwrap();
    received_parts = socket.recv_multipart(0).unwrap();
    //println!("client thread, received from server, #received_parts: {:?}", received_parts);
    println!(
        "\tclient thread, received from server, #received_parts: {:x?}; {:x?}",
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
        "\tclient thread, received from server, #received_parts: {:x?}; {:x?}",
        received_parts.pop().unwrap(),
        received_parts.pop().unwrap()
    );
}
