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

fn score_for_spaces(x: &Vec<u8>) -> u8 {
    let mut acc = 0;

    for xe in x {
        if *xe == 32 {
            acc += 1
        }
    }

    acc
}

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
    let mut output = 0;
    let mut score = 0;

    for x in 0..255 {
        let current = score_for_spaces(&fixed_xor_one_char(&input, x));
        if  current > score {
            score = current;
            output = x;
        }
    }

    println!("\n\n{}", String::from_utf8_lossy(&fixed_xor_one_char(&input, output)));
}
