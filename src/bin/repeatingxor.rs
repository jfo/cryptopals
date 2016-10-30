extern crate rustc_serialize;
use rustc_serialize::hex::*;

fn main() {
    let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    let mut out = Vec::new();

    for i in 0..input.len() {
        out.push(input[i] ^ key[i % key.len()]);
    }

    println!("{}", out.to_hex());
}
