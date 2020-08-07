
use nanomsg::{Protocol, Socket};

use std::thread;
use std::time::Duration;

use std::io::{Read, Write};


pub fn client(client_id : String, end_point : &str) {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut ep = socket.connect(end_point).unwrap();
    let mut count = 1u32;

    let mut reply = String::new();

    loop {
        let request = format!("{} Request #{}", client_id, count);

        match socket.write_all(request.as_bytes()) {
            Ok(..) => println!("Send '{}'.", request),
            Err(err) => {
                println!("Client failed to send request '{}'.", err);
                break;
            }
        }

        match socket.read_to_string(&mut reply) {
            Ok(_) => {
                println!("Recv '{}'.", reply);
                reply.clear()
            }
            Err(err) => {
                println!("Client failed to receive reply '{}'.", err);
                break;
            }
        }
        thread::sleep(Duration::from_millis(1000));
        count += 1;
    }

    ep.shutdown();
}


