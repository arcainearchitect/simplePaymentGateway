use crate::http::{Request, Response};

pub fn handle_index(_req:Request)->Response{
    Response::ok("Welcome to the Rust payment gateway API")
}

pub fn handle_pay(req:Request)->Response{
    println!("Received payment request with body: {}", req.body);
    Response::ok("Payment processed successfully!")
}