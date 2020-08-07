
use nanomsg::{Protocol, Socket};

use std::thread;
use std::time::Duration;

use std::io::{Read, Write};

pub fn server(end_point : &str) {
    let mut socket = Socket::new(Protocol::Rep).unwrap();
    // let mut endpoint = socket.connect(CS_URL).unwrap();
    let mut ep = socket.bind(end_point).unwrap();
    let mut count = 1u32;

    let mut request = String::new();

    println!("Server is ready.");

    loop {
        match socket.read_to_string(&mut request) {
            Ok(_) => {
                println!("Recv '{}'.", request);

                let reply = format!("Request: {}; Server Reply => {}", request, request);
                match socket.write_all(reply.as_bytes()) {
                    Ok(..) => println!("Sent '{}'.", reply),
                    Err(err) => {
                        println!("Server failed to send reply '{}'.", err);
                        break;
                    }
                }
                request.clear();
                thread::sleep(Duration::from_millis(400));
                count += 1;
            }
            Err(err) => {
                println!("Server failed to receive request '{}'.", err);
                break;
            }
        }
    }

    ep.shutdown();
}



