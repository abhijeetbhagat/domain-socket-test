extern crate unix_socket;

use unix_socket::UnixStream;
use std::io::prelude::*;

fn main() {
    match UnixStream::connect("suckersocket"){
        Ok(mut s) => {
            s.write_all(b"rust rocks!");
        },
        Err(msg) => println!("{}", msg)
    }
    //stream.write_all(b"rust rocks!");
    //let mut response = String::new();
    //stream.read_to_string(&mut response).unwrap();
    //println!("{}", response);
}
