mod logging;
mod parser;
use clap::Parser;
use logging::{err, info};
use parser::sections::skelett_template::SkelettTemplate;
use parser::variables::variable_resolver;
use parser::{args::Args, sections::types::VariableType};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::{self};

fn main() {
    let args = Args::parse();

    // Template file should exist for processing
    assert!(
        Path::new(args.template.as_str()).is_file(),
        "{}",
        err(&format!("Template file {} doesn't exist", args.template))
    );

    println!("{}", info(&format!("Template file: {}", args.template)));

    let toml_string = fs::read_to_string(args.template).expect(&err("Couldn't read template file"));

    let toml: SkelettTemplate =
        toml::from_str(&toml_string).expect(&err("Couldn't parse template file"));

    match toml.variables {
        Some(variables_map) => {
            let mut cache: HashMap<VariableType, String> = HashMap::default();
            for (key, val) in &variables_map {
                let var_result = variable_resolver(&variables_map, &val, 10, &mut cache);
                // println!("== GOT == ");
                println!(
                    "Calling with variable with key: {key}, got {:?}",
                    var_result
                );
            }
        }
        None => {}
    }
}
