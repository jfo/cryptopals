extern crate rustc_serialize;
use std::io::{self, Read};
use rustc_serialize::hex::*;
use rustc_serialize::base64::*;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    print!("{}", buffer.from_hex().unwrap().to_base64(MIME));
}
