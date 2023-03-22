use serde_json;
use std::fs::read_to_string;
use std::io::{self, Write};
use colored::Colorize;

fn main() {
    let contents = read_to_string("dictionary.json").unwrap();

    let content_byte = contents.as_bytes();

    let json: serde_json::Value = serde_json::from_slice(content_byte).unwrap();

    let mut quit = false;

    while !quit {
        let mut not_found = true;

        print!("{}", "Search: ".blue().bold().italic());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error: READ LINE");

        let mut search = String::new();

        match input.trim() {
            "qq" => quit = true,
            _ => search = input
        }

        for (k, v) in json.as_object().unwrap() {
            if k.as_str() == search.trim() {
                println!("{}", v.as_str().unwrap());
                not_found = false;
            }
        } 

        if not_found { println!("{}", "Not Found".red().bold().italic()); }
    }
}
