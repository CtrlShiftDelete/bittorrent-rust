use std::env;

// use serde_bencode;
use serde_json;

fn decode_string(encoded_value: &str) -> serde_json::Value {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        return serde_json::Value::String(string.to_string());
}

fn decode_integer(encoded_value: &str) -> serde_json::Value {
    let num_str = &encoded_value[1..encoded_value.len()-1];
    let num = num_str.parse::<i64>().unwrap();
    return serde_json::Value::Number(num.into());
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "decode" {
        let encoded_value = &args[2];
        //let mut decoded_value: Value;
        if encoded_value.chars().next().unwrap().is_digit(10) {
            let decoded_value = decode_string(encoded_value);
            println!("{}", decoded_value.to_string()); 
        } else if encoded_value.chars().next().unwrap()=='i' {
            let decoded_value = decode_integer(encoded_value);
            println!("{}", decoded_value.to_string());
        } else {
            panic!("Unhandled encoded value: {}", encoded_value)
        }
        //let decoded_value = decode_bencoded_value(encoded_value);
        // println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}