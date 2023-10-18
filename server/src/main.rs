// silence some compiler warnings; #![...] annotates code for the compiler
#![allow(dead_code)]

// pull the Request/Server structs from their respective namespaces
use std::env;
use server::Server;
use http::website_handler::WebsiteHandler;

mod server; // use this to go find the file that defines this module
mod http;

fn main() {
    // get public file directory from environment variables
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR")); // determined at compile time
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path); // determined at run time
    println!("public path: {}", public_path);
    // configure server and request handler
    let server = Server::new("127.0.0.1:8080".to_string());
    let handler = WebsiteHandler::new(public_path);
    // start server
    server.run(handler);
}
