extern crate unix_socket;
use std::thread;
use unix_socket::{UnixStream, UnixListener};
use std::io::Read;

fn handle_client(mut stream: UnixStream) {
    println!("reading stream...");
    let mut response = Vec::new();
    let n : usize = stream.read_to_end(&mut response).unwrap();
    println!("bytes read {}", n);
    println!("{}", std::str::from_utf8(response.as_slice()).unwrap());
}

fn main(){
    let listener = UnixListener::bind("suckersocket").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                /* connection failed */
                break;
            }
        }
    }

    // close the listener socket
    drop(listener);
}
