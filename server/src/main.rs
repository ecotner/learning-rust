// silence some compiler warnings; #![...] annotates code for the compiler
#![allow(dead_code)]

// pull the Request/Server structs from their respective namespaces
// use http::Request;
// use http::Method;
use server::Server;
use http::website_handler::WebsiteHandler;

mod server; // use this to go find the file that defines this module
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    let handler = WebsiteHandler::new();
    server.run(handler);
}
