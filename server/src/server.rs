use std::net::TcpListener;
use std::io::Read;
use crate::http::Request; // crate keyword refers to the "root" of the module
// use std::convert::TryFrom;

// a struct that holds data about the server
pub struct Server {
    addr: String,
}

// implementation block that holds the functionality for the Server struct.
// unlike classes, these "method-like" objects are defined separately from
// the data contained in the struct
impl Server {
    pub fn new(addr: String) -> Self {
        // Server { addr: addr }
        Self { addr } // shorthand for above line
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        // listens to a TCP socket; if there's an error, end the program
        let listener = TcpListener::bind(&self.addr).unwrap();

        // continuously reads TCP connections
        loop {
            match listener.accept() { // blocks until connection is established
                Ok((mut stream, _)) => {
                    // if the connection is successful, read the raw data from the HTTP request into a buffer
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // convert the [u8] buffer to a string and print
                            let request_str = String::from_utf8_lossy(&buffer);
                            println!("Received a request: {}", request_str);
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request {}", e)
                            }
                        },
                        // if there's an error, log it, but don't panic
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                },
                // if there's an error, log it, but don't panic
                Err(e) => println!("Failed to establish a connection: {}", e),
            }           
        }
    }
}