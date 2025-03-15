use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRES: &str = "localhost:1234";

fn main(){
    // connection
    println!("Connecting to {}", ECHO_SERVER_ADDRES);
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRES){
        println!("connected to echo server {}:{}",
            stream.local_addr().unwrap().ip(), 
            stream.local_addr().unwrap().port(), 
        );

        // Write hello world msg
        let message = "Hello World!";
        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();
        println!("sent: {}", message);

        // read the result
        let mut buf = [0; 1024];
        let len = stream.read(&mut buf).unwrap();
        
        let message = String::from_utf8_lossy(&buf);
        println!("received: {}, len: {}", message, len);

    } else {
        println!("Failed to connect to echo server {}", ECHO_SERVER_ADDRES);
    }

}