use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // Start the server
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let host = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&host).expect("Could not start listener");
    println!("Rustkernel is running on {}", host);

    for stream in listener.incoming() {
        let mut stream = stream.expect("Could not iterate over stream");
        let mut buffer = [0; 8192];
        stream.read(&mut buffer).expect("Error reading stream");
        // // Convert the utf8 to a string
        let req = String::from_utf8_lossy(&buffer);
        println!("\n{}\n", req);
        let res = "HTTP/2.0 200 OK";
        stream.write_all(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
