use std::env;
//use std::collections::HashMap;
// use serde_bencode;
use serde_json;
#[allow(dead_code)]
fn decode_query(encoded_value: &str) -> (serde_json::Value, usize) {
    let first_char = encoded_value.chars().next().unwrap();
    match first_char {
        'i' => {
            //decode integer
            let end_index = encoded_value.find('e').unwrap();
            let number_string = &encoded_value[1..end_index];
            let number = number_string.parse::<i64>().unwrap();
            (serde_json::Value::Number(number.into()), end_index)
        }
        '0'..='9' => {
            //decode string
            let colon_index = encoded_value.find(':').unwrap();
            let number_string = &encoded_value[..colon_index];
            let number = number_string.parse::<i64>().unwrap();
            let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
            (serde_json::Value::String(string.to_string()), colon_index + number as usize)
        }
        'l' => {
            //decode list
            let mut result = Vec::new();
            let mut current_char_index = 1;
            while current_char_index < encoded_value.len() {
                let initial_char = encoded_value.chars().nth(current_char_index).unwrap();
                if initial_char == 'e' {
                    break;
                }
                let (decoded_value, next_index) = decode_query(&encoded_value[current_char_index..]);
                result.push(decoded_value);
                current_char_index += next_index + 1;
            }
            (serde_json::Value::Array(result), current_char_index)
        }
        'd' => {
            //decode dictionary
            //let mut result = HashMap::new();
            let mut result = serde_json::Map::new();
            let mut current_char_index = 1;
            while current_char_index < encoded_value.len() {
                let initial_char = encoded_value.chars().nth(current_char_index).unwrap();
                if initial_char == 'e' {
                    break;
                }
                let (decoded_key, next_index) = decode_query(&encoded_value[current_char_index..]);
                current_char_index += next_index+1;
                let (decoded_value, next_index) = decode_query(&encoded_value[current_char_index..]);
                current_char_index += next_index+1;
                result.insert(decoded_key.as_str().unwrap().to_string(), decoded_value);
            }
            (serde_json::Value::Object(result), current_char_index)
        }
        _ => {
            (serde_json::Value::String("Panic: Invalid Input String".to_string()), 0)
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];

        // Determine the type of encoded value
        let (decoded_query, _) = decode_query(encoded_value);
        println!("{}",decoded_query.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
