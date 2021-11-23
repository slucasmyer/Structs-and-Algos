use std::io::BufRead;
use std::char;
fn caesar_cipher(_num: i32, stringy: String, rot: i32) -> String {
    let mut caesar = String::from("");
    for c in stringy.chars() {
        if c.is_ascii_alphabetic() {
            let mut b = [0; 2];
            let encoded = c.encode_utf16(&mut b);
            if c.is_uppercase() {
                encoded[0] -= 65u16;
                encoded[0] += rot as u16;
                encoded[0] = encoded[0]%26;
                encoded[0] += 65u16;
            } else {
                encoded[0] -= 97u16;
                encoded[0] += rot as u16;
                encoded[0] = encoded[0]%26;
                encoded[0] += 97u16;
            }
            let decoded = char::from_u32(encoded[0] as u32);
            caesar.push(decoded.unwrap());
        } else {
            caesar.push(c)
        }
    }
    caesar
}



fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let mut count = 0i32;
    let mut str_len = 0i32;
    let mut init_str = String::from("");
    let mut rot = 0i32;
    for line in lock.lines() {
        if count == 0 {
            str_len = line.unwrap().parse().unwrap();
        } else if count == 1 {
            init_str = line.unwrap();
        } else if count == 2 {
            rot = line.unwrap().parse().unwrap();
        }
        count += 1;
    }
    println!("{}", caesar_cipher(str_len, init_str, rot));
}