use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:1997") {
        Ok(mut stream) => {
            println!("success connect to 1997");
            let msg = b"I am Sorry";
            let expect_resp = b"That's ok";
            for _ in 0..10 {
                stream.write(msg);
            }
            println!("Send hello, awaiting reply");
            // use 9 byte buffer
            let mut data = [0 as u8; 9];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == expect_resp {
                        println!("Reply is ok")
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e)
        }
    }

}
