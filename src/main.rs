use std::str;
use rand::distributions::{Alphanumeric, DistString, Distribution, Standard};
use rand::Rng;

fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    let b64vec: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().collect();

    // First stage, get a binary string
    let mut binstr = input.as_ref()
                      .iter()
                      .map(|x| format!("{:08b}", x))
                      .fold("".to_string(), |cur: String, nxt: String| cur + &nxt);

    // Padd with zeroes, to make the word divisible by 6
    let padd1 = binstr.len() % 6;
    if padd1 > 0 {
        binstr.push_str(&"0".repeat(6 - padd1));
    }
    assert!(binstr.len() % 6 == 0);

    // Second stage, get the base64 encoded string
    let mut out: String = binstr
        .as_bytes()
        .chunks(6)
        .map(|s| str::from_utf8(s).unwrap())
        .map(|n| u32::from_str_radix(n, 2).unwrap())
        .map(|c| b64vec[usize::try_from(c).unwrap()])
        .collect();

    // Padd with = sign, to make the word divisible by 4
    let padd2 = out.len() % 4;
    if padd2 > 0 {
        out.push_str(&"=".repeat(4 - padd2));
    }
    assert!(out.len() % 4 == 0);
    return out;
}
        
fn compare<T: AsRef<[u8]>>(input: T) {
        let us = base64_encode(&input);
        let them = base64::encode(&input);
        if us != them {
            panic!("{} vs {}", us, them);
        }
        println!("bsae64: {}\n\n", us);
}

fn alphanumeric_fuzz() {
    let mut rng = rand::thread_rng();
    let input = Alphanumeric.sample_string(&mut rand::thread_rng(), rng.gen_range(1..64));
    compare(input);
}

fn binary_fuzz() {
    let mut rng = rand::thread_rng();
    let input: Vec<u8> = Standard.sample_iter(&mut rand::thread_rng())
                                 .take(rng.gen_range(1..1000))
                                 .collect();
    compare(input);
}
fn main() {
    loop {
        binary_fuzz();
    }
}
