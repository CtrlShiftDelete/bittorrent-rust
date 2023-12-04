use std::env;
//use std::collections::HashMap;
// use serde_bencode;
mod decode;


fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];

        // Determine the type of encoded value
        let (decoded_query, _) = decode::decode_query(encoded_value);
        println!("{}",decoded_query.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}