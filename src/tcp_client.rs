use std::{
    cmp,
    io::{self, BufRead, BufReader, Read, Write},
    net::TcpStream,
    time::Duration,
};

#[test]
fn test_main() {
    let mut stream =
        TcpStream::connect_timeout(&"127.0.0.1:8080".parse().unwrap(), Duration::from_secs(1))
            .expect("Could not connect to server");

    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .expect("Could not set a read timemout");

    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = vec![];

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        stream
            .write(&input.as_bytes())
            .expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);

        reader
            .read_until(b'\n', &mut buffer)
            .expect("Failed read from the server");

        print!(
            "{}",
            String::from_utf8(buffer).expect("Failed convert buffer to utf8 string")
        );
    }
}
