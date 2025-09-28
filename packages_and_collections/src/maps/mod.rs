
use std::{collections::HashMap, thread::scope};

pub fn create_print_map() {
    println!("Creating map: ");
    let mut socres: HashMap<String, u32> = HashMap::new();
    socres.insert(String::from("Purple"), 42);
    socres.insert(String::from("Orange"), 21);

    println!("Printing map...");

    for (key, value) in socres {
        println!("Key: {key} | Value: {value}");
    }
}
