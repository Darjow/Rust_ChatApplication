use std::io::{BufRead, BufReader, Write};
use std::sync::Mutex;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::Arc;

type ClientList = Arc<Mutex<HashMap<String, TcpStream>>>;

pub struct ClientHandler {
  stream: TcpStream,
  clients: ClientList,
}

impl ClientHandler {
  pub fn new(stream: &TcpStream, clients: ClientList) -> Self {
    Self { stream: stream.try_clone().unwrap(), clients }
}

  pub fn get_username(&self) -> String {
    let mut reader = BufReader::new(&self.stream);
    let mut username = String::new();
    reader.read_line(&mut username).unwrap();
    username.trim().to_string()
  }
  pub fn broadcast(&self, message: &str) {
    let mut active_clients = self.clients.lock().unwrap();
    for (username, client_stream) in active_clients.iter_mut() {
      if client_stream.peer_addr().unwrap() != self.stream.peer_addr().unwrap() {
        client_stream.write(message.as_bytes()).unwrap();
      }
    }
  }

  pub fn handle(&mut self) {
    let username = self.get_username();
    let mut message = format!("{} has joined the chat\n", username);
    self.broadcast(&message);

    loop {
      let mut message = String::new();
      match BufReader::new(&self.stream).read_line(&mut message) {
        Ok(0) => break,
        Ok(_) => {
          message = message.trim().to_string();
          message = format!("{}: {}\n", username, message);
          self.broadcast(&message);
        }
        Err(_) => {
          break;
        }
      }
    }

    self.clients.lock().unwrap().remove(&username);
    message = format!("{} has left the chat\n", username);
    self.broadcast(&message);
  }
}

