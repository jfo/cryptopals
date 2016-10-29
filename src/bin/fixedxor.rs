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

fn fixed_xor_one_char(x: &Vec<u8>, y: u8) -> Vec<u8> {

    let mut acc = Vec::new();
    for xe in x.iter() {
        acc.push(xe ^ y);
    }

    acc
}

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();

    for x in 0..255 {
        println!("{}: {}",x, String::from_utf8_lossy(&fixed_xor_one_char(&input, x)));
    }

    println!("\n\n{}", String::from_utf8_lossy(&fixed_xor_one_char(&input, 88)));
}
