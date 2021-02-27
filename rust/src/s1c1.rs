// Convert hex to base64
// https://cryptopals.com/sets/1/challenges/1
use crate::test;

extern crate base64;

use std::u8;
use self::base64::{encode};

fn hex_to_base64(hex: &str) -> String {
    let mut bytes = Vec::new(); // Create a mutable Vector of bytes

    for i in 0..(hex.len()/2) { 
        // Get a two character chunk and convert it to integer
        let res = u8::from_str_radix(&hex[2*i..2*i+2], 16);
        // Push to the mutable vector bytes
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => {println!("Got an error decoding hex: {}", e)}
        }
    }


    encode(&bytes) // Convert Vec<u8> to base64 encoded string
}

pub fn run() {
    const INPUT: &str =  "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    const EXPECTED: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let result: String = hex_to_base64(INPUT);

    test::run("Set 1, challenge 1",EXPECTED, &result)
}