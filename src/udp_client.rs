use std::{io, net::UdpSocket};

#[test]
fn test_main() {
    let socket = UdpSocket::bind("127.0.0.1:3333").expect("Failed bind client socket");
    socket
        .connect("127.0.0.1:8084")
        .expect("Failed connect to server");

    let mut count = 0;
    loop {
        if count > 5 {
            break;
        }
        count += 1;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed read from stdin");

        socket
            .send(input.as_bytes())
            .expect("Failed send data to server");
    }
    // listen response
    loop {
        let mut buf = [0; 1500];
        socket
            .recv_from(&mut buf)
            .expect("Failed recv data from server");

        print!(
            "{}",
            String::from_utf8(buf.to_vec()).expect("Failed write buffer as string")
        );
    }
}
