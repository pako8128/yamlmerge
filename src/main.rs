use std::{env, fs::File, io::Read, path::Path, process};

use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE: {} [FILES...]", args[0]);
        process::exit(1);
    }

    let mut root = match parse_yaml(&args[1]) {
        Ok(yaml) => yaml,
        Err(s) => {
            eprintln!("{}: {}", args[1], s);
            process::exit(2);
        }
    };

    for p in args.iter().skip(2) {
        let addition = match parse_yaml(p) {
            Ok(yaml) => yaml,
            Err(s) => {
                eprintln!("skipping {}: {}", p, s);
                continue;
            }
        };
        merge(&mut root, addition).unwrap();
    }

    let mut result = String::new();
    YamlEmitter::new(&mut result).dump(&root).unwrap();
    println!("{}", result);
}

fn merge(root: &mut Yaml, addition: Yaml) -> Result<(), String> {
    if let Yaml::Hash(ref mut root) = root {
        if let Yaml::Hash(map) = addition {
            for (k, v) in map {
                if v.as_hash().is_some() {
                    if let Some(outer) = root.get_mut(&k) {
                        merge(outer, v)?;
                    } else {
                        root.insert(k, v);
                    }
                } else {
                    root.insert(k, v);
                }
            }
        }
    }

    Ok(())
}

fn parse_yaml<P: AsRef<Path>>(path: P) -> Result<Yaml, String> {
    let mut f = File::open(path).map_err(|_| String::from("could not open file"))?;
    let mut s = String::new();
    f.read_to_string(&mut s)
        .map_err(|_| String::from("could not read file"))?;

    let mut yaml =
        YamlLoader::load_from_str(&s).map_err(|_| String::from("could not parse file"))?;
    let yaml = yaml.pop().ok_or(String::from("no document found"))?;
    Ok(yaml)
}
