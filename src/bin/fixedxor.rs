extern crate rustc_serialize;
use std::io;
use std::io::prelude::*;
use rustc_serialize::hex::FromHex;

fn fixed_xor(x: Vec<u8>, y: Vec<u8>) -> Vec<u8>{
    assert_eq!(x.len(), y.len());

    let mut acc = Vec::new();
    for (xe, ye) in x.iter().zip(y) {
        acc.push(xe ^ ye);
    }

    acc
}

fn fixed_xor_one_char(x: &[u8], y: u8) -> Vec<u8> {
    let mut acc = Vec::new();
    for xe in x.iter() {
        acc.push(xe ^ y);
    }

    acc
}

fn score_for_spaces(x: &[u8]) -> u8 {
    let mut acc = 0;

    for xe in x {
        if *xe == 32 {
            acc += 1
        }
    }

    acc
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input1 = line.unwrap();
        let input = input1.from_hex().unwrap();
        for x in 0..255 {
            let candidate = fixed_xor_one_char(&input, x);
            if score_for_spaces(&candidate) > 4 {
                // 7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
                // Now that the party is jumping
                println!("{}", input1);
                println!("{}", String::from_utf8_lossy(&candidate));
            }
        }
    }
}
