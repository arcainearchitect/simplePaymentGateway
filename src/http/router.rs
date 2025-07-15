use crate::http::{Request, Response};
use crate::handlers::{handle_index, handle_pay};

pub fn route_request(req:Request)->Response{
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/")=>handle_index(req),
        ("POST", "/pay")=>handle_pay(req),
        _=>Response::not_found(),
    }
}