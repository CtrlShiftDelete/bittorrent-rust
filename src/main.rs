use std::env;
// use serde_derive::{Serialize, Deserialize};
use serde::{Serialize, Deserialize};
use std::fs;
//use std::collections::HashMap;
use serde_bencode;
mod decode;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Info {
    length: usize,
    name: String,
    #[serde(rename="piece length")]
    piece_length: usize,
    //pieces: Vec<u8>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];

        // Determine the type of encoded value
        let (decoded_query, _) = decode::decode_query(encoded_value);
        println!("{}",decoded_query.to_string());
    } else if command == "info" {
        let contents = fs::read(&args[2]).expect("Should have been able to read the file");
        let torrent_data: Torrent = serde_bencode::from_bytes(&contents).expect("could not parse");
        println!("Tracker URL: {}", torrent_data.announce);
        println!("Length: {}", torrent_data.info.length);
        //println!("{}", torrent_data);
    } else {
        println!("unknown command: {}", args[1])
    }
}