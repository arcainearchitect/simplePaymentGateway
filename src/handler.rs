use std::{io::{Read,Write}, net::{TcpListener, TcpStream}};

pub fn handle_connection(mut stream:TcpStream){
    let mut buffer=[0; 1024];

    match stream.read(&mut buffer){
        Ok(bytes_read)=>{
            if bytes_read==0{
                return ;    // empty request
            }
            
            // Convert raw bytes into string
            let request=String::from_utf8_lossy(&buffer[..bytes_read]);

            // Debug print
            println!("Received request:\n{}", request);

            // Parse basic request
            if request.starts_with("POST /pay"){
                let body="Payment received successfully.";
                let response=format!(
                    "HTTP/1.1 200 Ok\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap();
            } else {
                let body="Not fount.";
                let response=format!(
                    "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap();
            }
            //     let response="HTTP/1.1 200 Ok\r\nContent-Type: text/plain\r\nPayment received successfully.";
            //     stream.write_all(response.as_bytes()).unwrap();
            // } else {
            //     let response="HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\nNot found.";
            //     stream.write_all(response.as_bytes()).unwrap();
            // }
        }
        Err(conn_err0r)=>eprintln!("Failed to read from connection: {}", conn_err0r),
    }

    stream.flush().unwrap();
}