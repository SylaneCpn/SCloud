use sscanf::*;

#[derive(Debug)]
pub struct Header {
    pub failure: bool,
    pub request_method: String,
    pub request_uri: String,
    pub connection: String,
    pub upgrade: String,
    pub length: usize,
}

impl Header {
    pub fn new() -> Self {
        Self {
            failure: false,
            request_method: String::new(),
            request_uri: String::new(),
            connection: String::from("keep-alive"),
            upgrade: String::new(),
            length: 0,
        }
    }

    pub fn process_header(&mut self, content: &Vec<String>) {
        for line in content.iter() {
            if self.request_method.is_empty() {
                //first line
                if let Ok((method, uri)) = sscanf!(line, "{} {} HTTP/1.1", String, String) {
                    self.request_method = method;
                    self.request_uri = uri;
                } else {
                    self.failure = true;
                    break;
                }
            } else if let Ok(connection) = sscanf!(line, "Connetion:{}", String) {
                self.connection = connection;
            } else if let Ok(lenght) = sscanf!(line, "Content-Length:{usize}") {
                self.length = lenght;
            } else if let Ok(upgrade) = sscanf!(line, "Upgrade:{}", String) {
                self.upgrade = upgrade;
            }
        }
    }
}
