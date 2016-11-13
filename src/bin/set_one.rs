#![allow(dead_code)]

extern crate rustc_serialize;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::base64::{ ToBase64, MIME };
use rustc_serialize::hex::{ FromHex, ToHex };

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

fn challenge1() {
    let str1 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
        .from_hex()
        .unwrap();
    // should produce: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
    println!("{}", str1.to_base64(MIME));
}

fn challenge2() {
    let str1= "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
    let str2= "686974207468652062756c6c277320657965".from_hex().unwrap();
    // 746865206b696420646f6e277420706c6179
    println!("{}", fixed_xor(str1, str2).to_hex());
}

fn challenge3() {
    let str1 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
    for x in 0..255 {
        let candidate = fixed_xor_one_char(&str1, x);
        if score_for_spaces(&candidate) > 5 {
            // Cooking MC's like a pound of bacon
            println!("{}", String::from_utf8_lossy(&candidate));
        }
    }
}

fn challenge4() {
    // let stdin = io::stdin();
    let f = File::open("/Users/jfowler/code/cryptopals/inputfiles/4.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let input1 = line.unwrap();
        let input = input1.from_hex().unwrap();
        for x in 0..255 {
            let candidate = fixed_xor_one_char(&input, x);
            if score_for_spaces(&candidate) > 4 {
                // 7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
                // Now that the party is jumping
                // println!("{}", input1);
                println!("{}", String::from_utf8_lossy(&candidate));
            }
        }
    }
}

fn challenge5() {
    let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    let mut out = Vec::new();

    // 0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
    // a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f
    for i in 0..input.len() {
        out.push(input[i] ^ key[i % key.len()]);
    }

    println!("{}", out.to_hex());
}

fn main() {
    challenge1();
    challenge2();
    challenge3();
    challenge4();
    challenge5();
}
