use std::{net::UdpSocket, thread, time::Duration};

use rand::{thread_rng, Rng};

#[test]
pub fn test_main() {
    let socket = UdpSocket::bind("0.0.0.0:8084").expect("Could not bind socket");

    let mut handles = vec![];
    loop {
        let mut buf = [0; 1500];
        let sock = socket.try_clone().expect("Failed clone socket");
        println!("before recv request!");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                println!(
                    "Recved request: {}",
                    String::from_utf8(buf.to_vec()).unwrap()
                );
                handles.push(thread::spawn(move || {
                    println!(
                        "Handling datagram from {} for {}",
                        src,
                        String::from_utf8(buf.to_vec()).unwrap()
                    );
                    let sleep = thread_rng().gen_range(2..8);
                    println!("Sleep {} sec", sleep);
                    thread::sleep(Duration::from_secs(sleep));
                    let size = sock.send_to(&buf, src).expect("Failed to send a response");
                    println!(
                        "Responsed {} byte data for {}\n",
                        size,
                        String::from_utf8(buf.to_vec()).unwrap()
                    );
                }));
            }
            Err(e) => {
                eprintln!("Couldn't recieve a datagram: {}", e);
            }
        }
    }
    // handles.into_iter().map(|h|h.join());
}
