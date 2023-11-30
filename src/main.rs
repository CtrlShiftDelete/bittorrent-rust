use std::env;

// use serde_bencode;
use serde_json;
#[allow(dead_code)]
fn decode_string(encoded_value: &str) -> serde_json::Value {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    return serde_json::Value::String(string.to_string());
}

fn decode_integer(encoded_value: &str) -> serde_json::Value {
    let num_str = &encoded_value[1..encoded_value.len() - 1];
    let num = num_str.parse::<i64>().unwrap();
    return serde_json::Value::Number(num.into());
}

fn decode_list(mut encoded_value: &str) -> serde_json::Value {
    // decode l5:helloi52ee
    let mut res = Vec::new();
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[1..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    res.push(serde_json::Value::String(string.to_string()));
    
    encoded_value = &encoded_value[colon_index+1+number as usize..];
    //let colon_index2 = encoded_value.find(':');
    let num_str = &encoded_value[1..encoded_value.len() - 2];
    let num = num_str.parse::<i64>().unwrap();
    res.push(serde_json::Value::Number(serde_json::Number::from(num)));
    return serde_json::Value::Array(res);
    // return serde_json::Value::String(num.to_string());
    // return serde_json::Value::Number(num.into());
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];

        // Determine the type of encoded value
        let first_char = encoded_value.chars().next().unwrap();
        println!("{}", first_char);
        match first_char {
            'i' => {
                // Decode integer
                let decoded_value = decode_integer(encoded_value);
                println!("{}", decoded_value.to_string());
            }
            'l' => {
                // Decode list
                let decoded_value = decode_list(encoded_value);
                println!("{}", decoded_value.to_string());
            }
            _ => {
                // Decode string
                let decoded_value = decode_string(encoded_value);
                println!("{}", decoded_value.to_string());
            }
        }
    } else {
        println!("unknown command: {}", args[1])
    }
}
