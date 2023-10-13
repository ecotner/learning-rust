use std::net::TcpListener;
use std::io::Read;

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

        // listens to a TCP socket
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
                            let request = String::from_utf8_lossy(&buffer);
                            println!("Received a request: {}", request);
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