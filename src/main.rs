use std::{env, fmt::format, fs, path::Path};

use json::JsonValue;

fn main() {
    let home = env::var("HOME").expect("$HOME env variable not set");

    let path = Path::new(&home).join(".cargo/.crates2.json");
    let string = fs::read_to_string(path).expect("Failed to read .crates2.json");

    let json = json::parse(&string).expect("Failed to parse json");

    let mut command = String::new();

    if let JsonValue::Object(map) = &json["installs"] {
        for (name, value) in map.iter() {
            if let JsonValue::Array(features) = &value["features"] {
                if !features.is_empty() {
                    command.push_str("-F");
                }
            }
        }
    } else {
        panic!()
    }
}
