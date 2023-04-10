use std::io::{BufRead, BufReader, Write};
use std::sync::Mutex;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::Arc;


type ClientList = Arc<Mutex<HashMap<String, TcpStream>>>;

pub struct ClientHandler {
  stream: TcpStream,
  clients: ClientList,
  username: Option<String>,
}

impl ClientHandler {
  pub fn new(stream: &TcpStream, clients: ClientList) -> Self {
    Self { stream: stream.try_clone().unwrap(), clients, username: None}
}

  pub fn get_username(&mut self) -> String {
    if let Some(username) = &self.username {
      return username.clone();
    }
    let mut username = String::new();
    let question = "What is your username?\n";
    self.send_message_to_client(&question);

    let mut reader = BufReader::new(self.stream.try_clone().unwrap());
    if reader.read_line(&mut username).unwrap() > 0 {
      self.username = Some(username.trim().to_owned());
    }
    return username;
  }
  
  pub fn broadcast(&self, message: &str) {
    let mut active_clients = self.clients.lock().unwrap();
    for (_username, client_stream) in active_clients.iter_mut() {
      if client_stream.peer_addr().unwrap() != self.stream.peer_addr().unwrap() {
        client_stream.write(message.as_bytes()).unwrap();
        client_stream.flush().unwrap();
      }
    }
  }

  pub fn handle(&mut self) {
    let username = self.get_username();
    
    self.send_message_to_client(format!("Hi: {}. You have joined the chat together with {} others.\n", &username, (self.clients.lock().unwrap().len() - 1) as i32).as_str());
    self.broadcast(&format!("{} has joined the chat\n", username));

    loop {
      let mut message = String::new();
      match BufReader::new(&self.stream).read_line(&mut message) {
        Ok(0) => {
          self.disconnect();
          break;
        }
        Ok(_) => {
          message = message.trim().to_string();
          message = format!("{}: {}\n", username, message);
          self.broadcast(&message);
        }
        Err(_) => {
          self.disconnect();
          break;
        }
      }
    }
  }
  pub fn disconnect(&self){
    let username = &self.username.clone().unwrap();
    self.clients.lock().unwrap().remove(username);
  
    let message = format!("{} has left the chat\n", username);
    self.broadcast(&message);
  }

  pub fn send_message_to_client(&mut self, msg: &str){
    let message = format!("{}",msg);
    self.stream.write(&message.as_bytes()).unwrap();
  }
}

