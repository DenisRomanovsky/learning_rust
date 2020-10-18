mod http;
mod server; 
use server::Server;

fn main() {
    // let server = Server::new("127.0.0.1:8080"); // Will not work
    // Because "some value" is a type &str -- immutable --, which points on heap and stores only length of bytes to take.
    // While String is a more complex struct with lots of features.
    // let string_slice = &string[10..] -- returns &str slice
    // & - take value from this pointer

    let address_string = String::from("127.0.0.1:8080");
    let server = Server::new(address_string);
    server.run();
}
