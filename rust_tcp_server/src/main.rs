use std::net::{Shutdown, TCPListener, TCPStream};
//use std::thread;

fn main() {
    let listener = TCPListener::bind("0.0.0.0:3333").unwrap();
    //accept connections and process them, spawning new threads for each one.
    println!("Server listening on {:?}", listener.local_addr());
}
