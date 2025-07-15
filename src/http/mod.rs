pub mod router;

#[derive(Debug)]
pub struct Request{
    pub method:String,
    pub path:String,
    pub body:String,
}

#[derive(Debug)]
pub struct Response{
    pub status_code:u16,
    pub status_text:&'static str,
    pub body:String,
}

impl Request{
    pub fn from_raw(raw:&str)->Self{
        let lines:Vec<&str>=raw.lines().collect();
        let first_line=lines.get(0).unwrap_or(&"");
        let mut parts=first_line.split_whitespace();

        let method=parts.next().unwrap_or("").to_string();
        let path=parts.next().unwrap_or("").to_string();
        let body=parts.next().unwrap_or("").to_string();

        Request{
            method, path, body
        }
    }
}

impl Response{
    pub fn to_bytes(&self)->Vec<u8>{
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
            self.status_code,
            self.status_text,
            self.body.len(),
            self.body
        )
        .as_bytes()
        .to_vec()
    }

    pub fn ok(body:&str)->Self{
        Response {
            status_code:200,
            status_text:"OK",
            body:body.to_string(),
        }
    }

    pub fn not_found()->Self{
        Response {
            status_code:404,
            status_text:"NOT FOUND",
            body:"Route not found".to_string(),
        }
    }
}