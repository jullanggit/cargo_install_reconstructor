use std::{collections::HashMap, env, fs, path::Path};

use serde::Deserialize;

// The contents of the .cargo/.crates2.json file
#[derive(Deserialize)]
struct Crates2Json {
    installs: HashMap<String, Install>,
}
// Maybe this can be inlined
#[derive(Deserialize)]
struct Install {
    features: Vec<String>,
    no_default_features: bool,
}

fn main() {
    let home = env::var("HOME").expect("$HOME env variable not set");

    let path = Path::new(&home).join(".cargo/.crates2.json");
    let string = fs::read_to_string(path).expect("Failed to read .crates2.json");

    let json: Crates2Json = serde_json::from_str(&string).expect("Failed to deserialize json");

    let mut items = String::new();

    for (name, install) in json.installs {
        // no-default-features
        if install.no_default_features {
            items.push_str("--no-default-features");
            items.push(' ');
        }

        // features
        if !install.features.is_empty() {
            items.push_str("-F ");
        }
        for feature in install.features {
            items.push_str(&feature);
            items.push(' ');
        }

        // git
        let parts: Vec<_> = name.split_whitespace().collect();

        let name = parts[0];
        let (source, url) = parts[2].split_once('+').unwrap();
        if source == "(git" {
            items.push_str("--git ");

            let url = url.split_once('#').unwrap().0;
            items.push_str(url);
            items.push(' ');
        }

        // name
        items.push_str(name);

        items.push('\n');
    }
    println!("{items}");
}
