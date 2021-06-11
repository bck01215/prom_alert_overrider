extern crate yaml_rust;
use regex::Regex;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use structopt::StructOpt;
use yaml_rust::{Yaml, YamlLoader};
/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    overrides: std::path::PathBuf,
    /// The path to the file to rewrite to
    #[structopt(parse(from_os_str))]
    rewrite: std::path::PathBuf,
}

fn get_yaml_data() -> yaml_rust::Yaml {
    let args = Cli::from_args();
    let yaml_data = std::fs::read_to_string(args.overrides).expect("ERROR: Could not read yaml");
    let data = &(YamlLoader::load_from_str(&yaml_data).unwrap()[0]);
    let t = data.as_hash().unwrap().to_owned();
    t[&Yaml::from_str("overrides")].to_owned()
}

fn get_replacement_text(value: &str) -> std::string::String {
    let data = get_yaml_data();
    let x = &(data.as_hash().unwrap())[&Yaml::from_str(value)];
    let mut overrides = x.as_hash().unwrap()[&Yaml::from_str("regex")]
        .as_vec()
        .unwrap()
        .to_owned();
    let first = overrides.pop().unwrap();
    let mut text = first.as_str().unwrap().to_string();
    for i in overrides {
        text.push_str("\n\t\t\t\t\t\tor ");
        text.push_str(i.as_str().unwrap())
    }
    text
}
fn remake_block(block: &str) -> String {
    let re = Regex::new(r"\{\{[ ]{0,20}overrides\.(.*)\}\}").unwrap();
    let mut text = block.to_owned();
    for cap in re.captures_iter(block) {
        let replacement = get_replacement_text(&cap[1].replace(" ", ""));
        text = text.replace(&cap[0], &replacement);
    }
    text
}
fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let block = read_to_string(&args.rewrite).unwrap();
    let new_block = remake_block(&block);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&args.rewrite)?;
    file.write(new_block.as_bytes())?;

    Ok(())
}
