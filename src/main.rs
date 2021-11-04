use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // Start the server
    let listener;
    let port;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        listener = TcpListener::bind("127.0.0.1:0").unwrap();
        println!("Choosing random available port\nUse argument to specify port e.g. ech 8000");
        port = listener.local_addr().unwrap().port().to_string();
    } else {
        port = args[1].to_string();
        let host = format!("127.0.0.1:{}", port);
        listener = TcpListener::bind(&host).expect("Could not start listener");
    }
    println!("ech is up and running on 127.0.0.1:{}...", port);

    for stream in listener.incoming() {
        let mut stream = stream.expect("Could not iterate over stream");
        let mut buffer = [0; 8192];
        stream.read(&mut buffer).expect("Error reading stream");
        let req = String::from_utf8_lossy(&buffer);
        println!("{}", &req);
        let res = format!("HTTP/1.1 200 OK\r\n\r\n{}", req);
        stream
            .write_all(res.trim_end_matches(char::from(0)).as_bytes())
            .expect("Couldn't write stream");
        stream.flush().expect("Couldn't flush stream");
    }
}
