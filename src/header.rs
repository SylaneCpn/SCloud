use sscanf::*;

#[derive(Debug)]
pub struct Header {
    pub failure : bool,
    pub request_method : String,
    pub request_uri : String,
    pub connection : String,
    pub upgrade : String,
    pub length : usize 
}

impl Header {
    pub fn new()->Self {
        Self {failure : false , request_method : String::new(), request_uri : String::new(), connection : String::from("keep-alive"),upgrade : String::new(), length : 0}
    }
}

pub fn process_header(header_struct : &mut Header, content : &Vec<String>) {

    

    for line in content.iter() {
        if header_struct.request_method.is_empty() { //first line
            if let Ok((method,uri)) = sscanf!(line,"{} {} HTTP/1.1",String,String) {
                header_struct.request_method = method;
                header_struct.request_uri = uri;
            }
            else {
               
                header_struct.failure = true;
                break;
            }
        }

        else if let Ok(connection) = sscanf!(line,"Connetion:{}",String) {
            header_struct.connection = connection;
        }

        else if let Ok(lenght) = sscanf!(line,"Content-Length:{usize}") {
            header_struct.length = lenght;
        }

        else if let Ok(upgrade) = sscanf!(line,"Upgrade:{}",String) {
            header_struct.upgrade = upgrade;  
        }

    }
}