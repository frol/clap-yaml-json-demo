use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
/// This is superior CLI for YAML and JSON convertion!
enum Command {
    /// Convert from YAML to JSON
    YamlToJson { filename: std::path::PathBuf },
    /// Convert from JSON to YAML
    JsonToYaml { filename: std::path::PathBuf },
}

fn main() {
    let command = Command::parse();

    match command {
        Command::YamlToJson { filename } => {
            let yaml_value: serde_yaml::Value = serde_yaml::from_reader(std::fs::File::open(&filename).unwrap()).unwrap();
            println!("{}", serde_json::to_string_pretty(&yaml_value).unwrap());
        }
        Command::JsonToYaml { filename } => {
            let json_value: serde_json::Value = serde_json::from_reader(std::fs::File::open(&filename).unwrap()).unwrap();
            println!("{}", serde_yaml::to_string(&json_value).unwrap());
        }
    }
}
