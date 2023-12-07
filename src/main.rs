extern crate serde_bytes;
use sha1::{Digest, Sha1};
use std::env;
use hex;
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
    pieces: serde_bytes::ByteBuf,
}

fn calculate_info_hash(info: &Info) -> String {
    let mut hasher = Sha1::new();
    //convert info data to bytes first
    let info_bytes = serde_bencode::to_bytes(info).expect("Could not convert to bytes");
    hasher.update(info_bytes);
    let hash_result = hasher.finalize();
    //convert hash result to hexadecimal string
    let hex_string: String = hash_result.iter().map(|byte| format!("{:02x}",byte)).collect();
    hex_string.to_string()
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
        let info_hash = calculate_info_hash(&torrent_data.info);
        println!("Info Hash: {}", info_hash);

        println!("Piece Length: {}", torrent_data.info.piece_length);
        // let pieces_string = String::from_utf8_lossy(&torrent_data.info.pieces);
        // println!("Pieces: {}", pieces_string);
        // println!("Pieces: {}", type_of(&torrent_data.info.pieces));
        // println!("Pieces Hash: {}", calculate_pieces_hash(&torrent_data.info.pieces));
        // let piece_hashes = calculate_piece_hashes(&torrent_data.info);
        println!("Piece Hashes:");
        torrent_data
        .info
        .pieces
        .chunks(20)
        .for_each(|hash| println!("{}", hex::encode(hash)));

    } else {
        println!("unknown command: {}", args[1])
    }
}