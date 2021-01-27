use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    println!("server start at 127.0.0.1:7878");
    match listener {
        Ok(n) => {
            for stream in n.incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(|| {
                            handle_client(stream);
                        });
                    }
                    Err(e) => panic!(e),
                }
            }
        }
        Err(e) => {
            println!("failed due to {}", e);
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 16];

    while let Ok(read) = stream.read(&mut buffer) {
        if read == 0 {
            break;
        }

        if let Err(_) = stream.write(&buffer[0..read]) {
            break;
        }
    }
}
