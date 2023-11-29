// use serde_json;
use std::env;
use std::collections::HashMap;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
// fn decode_string(encoded_value: &str) -> String {
//     // If encoded_value starts with a digit, it's a number
//     if encoded_value.chars().next().unwrap().is_digit(10) {
//         // Example: "5:hello" -> "hello"
//         let colon_index = encoded_value.find(':').unwrap();
//         let number_string = &encoded_value[..colon_index];
//         let number = number_string.parse::<i64>().unwrap();
//         let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
//         return string.to_string();
//     } else {
//         panic!("Unhandled encoded value: {}", encoded_value)
//     }
// }

// fn decode_integer(encoded_value: &str) -> String {
//     let i_index = encoded_value.find('i').unwrap();
//     let e_index = encoded_value.find('e').unwrap();
//     let number_string = &encoded_value[i_index+1..e_index];
//     return number_string.to_string();
// }

//decode l5:helloi52ee
fn decode_list(encoded_value: &str) -> Vec<Box<dyn std::fmt::Debug>> {
    let mut mixed_list: Vec<Box<dyn std::fmt::Debug>> = Vec::new();

    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[1..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index+1..colon_index + 1 + number as usize];
    mixed_list.push(Box::new(string.to_string()));

    let decoded_number_string = &encoded_value[colon_index+2+number as usize..encoded_value.len()-2];
    println!("string = {}", decoded_number_string);
    let decoded_number = decoded_number_string.parse::<i64>().unwrap();
    mixed_list.push(Box::new(decoded_number));

    mixed_list
}

//decode d3:foo3:bar5:helloi52ee
fn decode_dictionary(encoded_value: &str) -> HashMap<String, Box<dyn std::fmt::Debug>> {

    let mut dictionary = HashMap::new();

    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[1..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index+1..colon_index+1+number as usize];
    
    encoded_value = &encoded_value[colon_index+number as usize..]
    let colon_index1 = encoded_value.find(':').unwrap();
    let number_string1 = &encoded_value[..colon_index1];
    let number1 = number_string1.parse::<i64>().unwrap();
    let string1 = &encoded_value[colon_index1+1..colon_index1+1+number1 as usize];

    encoded_value = &encoded_value[colon_index1+number1 as usize..encoded_value.len()-1];
    dictionary.insert(string.to_string(), Box::new(string1.to_string()));
    //decoded_list = decode_list(encoded_value);

    let colon_index2 = encoded_value.find(':').unwrap();
    let number_string2 = &encoded_value[1..colon_index2];
    let number2 = number_string2.parse::<i64>().unwrap();
    let string2 = &encoded_value[colon_index2+1..colon_index2 + 1 + number2 as usize];
    //mixed_list.push(Box::new(string.to_string()));

    //let decoded_number_string = &encoded_value[colon_index2++number2 as usize..encoded_value.len()-2];
    //println!("string = {}", decoded_number_string);
    // let decoded_number = decoded_number_string.parse::<i64>().unwrap();
    dictionary.insert(string2.to_string(), Box::new(52));

    // let colon_index2 = encoded_value.find(':').unwrap();
    // let number2 = 
    // let str2 = &encoded_value[colon_index2+1..]
    // dictionary.insert("foo".to_string(), Box::new("bar".to_string()));
    // dictionary.insert("hello".to_string(), Box::new(52));

    dictionary
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // println!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        // let decoded_value = decode_string(encoded_value);
        // println!("{}", decoded_value.to_string());
        // let decoded_value1 = decode_integer(encoded_value);
        // println!("{}", decoded_value.to_string());
        // let result = decode_list(encoded_value);
        
        // print!("[");
        // for (index, item) in result.iter().enumerate() {
        //     if index > 0 {
        //         print!(", ");
        //     }
        //     print!("{:?}", item);
        // }
        // println!("]");

        let result = decode_dictionary(encoded_value);
        print!("{");
        for (index, (key, value)) in result.iter().enumerate() {
            if index > 0 {
              print!(", ");
            }
            print!("{:?}: {:?}", key, value);
        }
        println!("}");
    } else {
        println!("unknown command: {}", args[1])
    }
}
