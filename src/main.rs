mod http;
mod handlers;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use http::{Request, Response};
use http::router::route_request;

fn stream_handler(listener:TcpListener){
    for stream in listener.incoming(){
        match stream{
            Ok (mut stream)=>{
                std::thread::spawn(move || {
                    handle_connection(&mut stream);
                });
            }
            Err(error)=>eprintln!("Connection error: {}", error),
        }
    }
}

fn handle_connection(stream: &mut TcpStream){
    let mut buffer=[0; 1024];

    match stream.read(&mut buffer){
        Ok(_)=>{
            if let Ok(req_str)=std::str::from_utf8(&buffer){
                let request=Request::from_raw(req_str);
                let response=route_request(request);
                
                stream.write_all(response.to_bytes().as_slice()).unwrap();
            }
        }
        Err(error)=>eprintln!("Failed to read: {}", error),
    }
}

fn main(){
    let addr="127.0.0.1:7878";
    let listener:TcpListener= TcpListener::bind(addr)
        .expect("Failed to bind to address");

    println!("Payment gateway server listening on http:{}", addr);

    stream_handler(listener);

}