# Rust Chat Application

## Overview
This is a simple client/server chat application written in Rust. The clients connect to the server and can set a username. Then multiple other clients can connect/disconnect and talk with each other on the socket.

## Prerequisites
You will need the following installed on your system:
- Rust programming language (https://www.rust-lang.org/tools/install)
- Cargo package manager (included with Rust)

## TODO List
- ~Disconnect clients and test the broadcast.~
- ~Clients communicating with each other.~
- Server pinging the clients every minute to see if connection is still there, if not remove from chat.
- Connect a database to store usernames with sent messages.
- GUI? | Browser?
- Authentication? 
- ... 

## How to Run
To run the chat application, follow these steps:

### Server
1. Open a terminal window and navigate to the project directory.
2. Run the following command to build and run the server:
```cargo run --bin server```
3. The server will start listening for incoming connections on port 8080.

### Client
1. Open another terminal window and navigate to the project directory.
2. Run the following command to build and run the client:
```cargo run --bin client```
3. The client will be prompt to enter a username.
4. Once you enter a username, the client will connect to the server and you can start chatting.

## Acknowledgements
This chat application is based on the tutorial at [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
