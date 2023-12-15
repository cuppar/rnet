use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::TcpStream,
};

#[test]
fn test_main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Could not connect to server");

    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = vec![];

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        stream
            .write(input.as_bytes())
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
