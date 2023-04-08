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

    let mut username = String::new();
    print!("Enter your username: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut username).unwrap();

    username = username.trim().to_string();

    loop {
        if let Err(e) = stream.write(username.as_bytes()) {
            println!("Error sending username: {}", e);
            continue;
        }
        if let Err(e) = stream.write(b"\n") {
            println!("Error sending newline: {}", e);
            continue;
        }
        break;
    }

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
