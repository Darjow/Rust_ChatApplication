use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::io::Write as IoWrite;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected to server");

    let server_stream = stream.try_clone().unwrap();
    thread::spawn(move || {
        let mut reader = BufReader::new(server_stream);
        loop {
            let mut message = String::new();
            reader.read_line(&mut message).unwrap();
            print!("{}", message);
        }
    });

    loop {
        let mut message = String::new();
        std::io::stdin().read_line(&mut message).unwrap();

        loop {
            if let Err(e) = stream.write(message.as_bytes()) {
                println!("Error sending message: {}", e);
                continue;
            }
            break;
        }

       
    }
}
