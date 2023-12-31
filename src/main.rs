use std::env;
use serde_json::json;
mod decode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "decode" {
        // Uncomment this block to pass the first stage
        let mut encoded_value = args[2].bytes().into_iter();
        let decoded_value = decode::decode_bencoded_value(&mut encoded_value).unwrap();
        println!("{}", json!(decoded_value));
    } else if command == "info" {
        let info = decode::get_metafile_info(&args);
        print!("{}", info);
    } else if command == "peers" {
        let info = decode::get_metafile_info(&args);
        let info_hash_encoded: String = unsafe { String::from_utf8_unchecked(info.hash) };
        let response = reqwest::blocking::Client::new()
            .get(info.trackter_url)
            .query(&[
                ("info_hash", info_hash_encoded.as_str()),
                ("peer_id", "00112233445566778899"),
                ("port", "6881"),
                ("uploaded", "0"),
                ("downloaded", "0"),
                ("left", &info.length.to_string()),
                ("compact", "1"),
            ])
            .send()
            .unwrap();

        let body = response.bytes().unwrap();
        let iterator = &mut body.iter().copied();
        let value = decode::decode_bencoded_value(iterator).unwrap();

        let encoded_peers = value.as_object().unwrap()["peers"]
            .as_str().
            expect("peers can be parse to string").
            as_bytes().
            chunks(6);

            let mut peers = Vec::new();
            for encoded_peer in encoded_peers {
                let mut iterator = encoded_peer.iter();
                let first_octet = iterator.next().unwrap();
                let second_octet = iterator.next().unwrap();
                let third_octet = iterator.next().unwrap();
                let fourth_octet = iterator.next().unwrap();
                let first_byte_port = *iterator.next().unwrap() as u16;
                let second_byte_port = *iterator.next().unwrap() as u16;
                let port = (first_byte_port << 8) | second_byte_port;
                let peer_address = format!(
                    "{}.{}.{}.{}:{}",
                    first_octet, second_octet, third_octet, fourth_octet, port
                );
                peers.push(peer_address);
            }
            print!("{:?}", peers);
        } else {
            println!("unknown command: {}", args[1])
        }
    }