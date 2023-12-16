use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rand::{thread_rng, Rng};
type Result<T> = std::result::Result<T, std::io::Error>;

fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        println!("read: {}", String::from_utf8_lossy(&buf[..bytes_read]));
        let sleep = Duration::from_secs(thread_rng().gen_range(0..6));
        println!("Sleeping for {:?} before replying", sleep);
        thread::sleep(sleep);
        stream.write(&buf[..bytes_read])?;
    }
}

#[test]
fn test_main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|e| eprintln!("{}", e));
                });
            }
            Err(e) => {
                eprintln!("failed: {}", e);
            }
        }
    }
}
