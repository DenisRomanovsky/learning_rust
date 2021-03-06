use std::net::TcpListener;
use crate::http::{Response, StatusCode, Request, ParseError}; // create - the root of the package
use std::convert::TryFrom;
use std::io::{Read}; // Include inner implementation. Read is a trate

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response{
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            //addr: addr 
            addr
        }   
    }

    // Takes ownership of the whole struct !!!
    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("Listening on {}", self.addr);

        loop {
            /*
            This solution is OK. But we can use match to make it prettier.
            let res = listener.accept();
            
            println!("Request accepted.");

            if res.is_err(){
                println!("Request is erroneous.");
                continue;
            }

            let (stream, addr) = res.unwrap();
            */

            //////////////////////////////////////////////////////////////////////////////////////////

            // match - matches result with all patterns inside an enum. Or can be used as a switch.
            // .accept() may return either OK or Err.
            match listener.accept(){
                // We need stream but ommit addr
                Ok((mut stream, _)) => {
                   println!("Request accepted");
                   let mut buffer = [0; 1024]; // Array of 1024 zeros. Rust can't handle uninitialized arrays.
                   match stream.read(&mut buffer) {
                       Ok(_) => {
                           println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                           // Request::try_from(&buffer as &[u8]);  Compiler expects a slice, not array.
                           let response = match Request::try_from(&buffer[..]) {
                               Ok(request) => handler.handle_request(&request),
                               Err(e) => handler.handle_bad_request(&e)
                            
                           };

                           if let Err(e) = response.send(&mut stream){
                               println!("failed to send reponse {}", e);
                           }
                       },
                       Err(e) => println!("Failed to read from connection {}", e),
                   }
                }
                Err(e) => println!("Error: {}", e),
                // We can add _ => {} as a default.
            }
        }
    }
}

