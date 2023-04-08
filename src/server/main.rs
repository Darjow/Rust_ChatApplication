mod client_handler;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use client_handler::ClientHandler;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
  println!("Server listening on port 8080");

  let clients: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        println!("New client connected: {:?}", stream.peer_addr());

        let mut client_handler = ClientHandler::new(&stream, clients.clone());
        let username = client_handler.get_username();

        clients.lock().unwrap().insert(username.clone(), stream.try_clone().unwrap());
        println!("{} joined the chat", username);

        client_handler.broadcast(&format!("{} joined the chat\n", username));

        thread::spawn(move || {
          client_handler.handle();
        });
        
      }
      Err(e) => {
          println!("Error accepting client connection: {}", e);
      }
      }
  }
}