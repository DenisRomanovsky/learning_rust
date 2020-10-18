mod http;
mod server; 
mod website_handler;

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

fn main() {
    // let server = Server::new("127.0.0.1:8080"); // Will not work
    // Because "some value" is a type &str -- immutable --, which points on heap and stores only length of bytes to take.
    // While String is a more complex struct with lots of features.
    // let string_slice = &string[10..] -- returns &str slice
    // & - take value from this pointer

    let address_string = String::from("127.0.0.1:8080");
    let server = Server::new(address_string);

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    println!("Public path: {}", public_path);
    server.run(WebsiteHandler::new(public_path));

     // to run the code with ENV variable: PUBLIC_PATH=$(pwd)/public cargo run
}
