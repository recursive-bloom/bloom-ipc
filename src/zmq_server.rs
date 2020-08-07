
use zmq::{Context, DEALER, ROUTER};

pub fn server(end_point : &str) {
    let context = Context::new();
    let socket = context.socket(ROUTER).unwrap();
    socket.bind(end_point).unwrap();
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