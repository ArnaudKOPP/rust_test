use std::io::{Read};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let thread_id = thread::current().id();
    println!("New handlers : {:?}", thread_id);
    loop {
        let mut data = [0 as u8; 50]; // using 50 byte buffer
        match stream.read(&mut data) {
            Ok(size) => {
                // echo everything!
                if size == 0 {
                    break;
                }
                println!("Received raw : {:?}", &data[0..size]);

                let _s = match str::from_utf8(&data[0..size]) {
                    Ok(v) => println!("Received     : {:?}", v),
                    Err(e) => println!("Invalid UTF8 : {}", e),
                };
                print!("#######################################################################\n");

                //stream.write(&data[0..size]).unwrap();
                //stream.write(String::from(" -> Ack\n").as_bytes()).unwrap();
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                println!("Shutdown stream : {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
