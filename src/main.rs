use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

pub mod handler;
use handler::handle_connection;

fn main(){
    let addr="127.0.0.1:7878";
    let listener:TcpListener=TcpListener::bind(addr)
        .expect("Failed to bind to address.");

    println!("Server now listening on {}", addr);

    for stream in listener.incoming(){
        match stream{
            Ok(stream)=>{
                // Handle connection in new thread
                thread::spawn(||{
                    handle_connection(stream);
                });
            }
            Err(error)=>eprintln!("Failed to connect to server: {}", error),
        }
    }
}