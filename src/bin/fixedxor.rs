extern crate rustc_serialize;
use rustc_serialize::hex::{ FromHex, ToHex };

fn fixed_xor(x: Vec<u8>, y: Vec<u8>) -> Vec<u8>{
    assert_eq!(x.len(), y.len());

    let mut acc = Vec::new();
    for (xe, ye) in x.iter().zip(y) {
        acc.push(xe ^ ye);
    }

    acc
}

fn main() {
    let str1= "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
    let str2= "686974207468652062756c6c277320657965".from_hex().unwrap();
    println!("{:?}", fixed_xor(str1, str2).to_hex());
}
