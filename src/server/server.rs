mod client_handler; 

use std::net::{TcpListener, TcpStream};
use log::{info};
use log4rs::config::{Deserializers, load_config_file};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use lazy_static::lazy_static;
use client_handler::ClientHandler;

type ClientList = Arc<Mutex<HashMap<String, TcpStream>>>;


lazy_static! {
  static ref CLIENTS: ClientList = Arc::new(Mutex::new(HashMap::new()));
}

fn main() {
  let listener = init_server();
  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        let peer_addr = stream.peer_addr().unwrap();

        info!("New client connected: {:?}. Waiting for him to set username", peer_addr);

        let mut client_handler = ClientHandler::new(&stream.try_clone().unwrap(), CLIENTS.clone());
        let username = client_handler.get_username();
        CLIENTS.lock().unwrap().insert(username.clone(), stream.try_clone().unwrap());

        info!("{} has set his username: {}", peer_addr, username);


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
fn init_server() -> TcpListener{
  let config_path = "log4rs.yml";
  let config = load_config_file(config_path, Deserializers::default()).unwrap();
  log4rs::init_config(config).unwrap();

  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
  info!("Server listening on port 8080");
  return listener;

}