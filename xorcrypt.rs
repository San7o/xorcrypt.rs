//usr/bin/rustc "$0" -o xorcrypt && ./xorcrypt "$@"; rm xorcrypt; exit $?
//
// SPDX-License-Identifier: MIT
//
// xorcrypt.rs
// ============
//
// Encrypt / Decrypt files by XORing every byte with a key string,
// equivalent to a Vigenere cipher. Bytes with 0 are not XORed as that
// could leak the key.
//
// Author:  Giovanni Santini
// Mail:    giovanni.santini@proton.me
// License: MIT
//
//
// Usage
// -----
// 
// Just run:
// 
//    ./xorcrypt.rs input.txt key output.txt
// 
// No need to have cargo, no need to build the binary yourself
//

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Usage: {} <in-filename> <password> <out-filename>",
               args[0]);
    }

    let input = File::open(&args[1]).expect("opening input file");
    let input_buf = BufReader::new(input);

    let pass: Vec<char> = args[2].chars().collect();
    let mut pass_i : usize = 0;
    
    let output = File::create(&args[3]).expect("creating output file");
    let mut output_buf = BufWriter::new(output);

    for b in input_buf.bytes() {
        let byte = b.unwrap();
        let mut enc_byte;
        
        // Skip empty bytes to avoid leaking the key when there are
        // multiple 0 bytes one after the other
        enc_byte = if byte == 0 { 0 } else { byte ^ pass[pass_i] as u8 };
        enc_byte = if enc_byte == 0 { byte } else { enc_byte };
        
        let _ = output_buf.write(&[enc_byte]);
        pass_i = (pass_i + 1) % pass.len();
    }

    println!("done")
}
