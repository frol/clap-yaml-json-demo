use clap::Parser;
use color_eyre::eyre::WrapErr;

#[derive(Parser)]
#[clap(author, version, about)]
/// This is superior CLI for YAML and JSON convertion!
enum Command {
    /// Convert from YAML to JSON
    YamlToJson { filename: ExistingPath, },
    /// Convert from JSON to YAML
    JsonToYaml { filename: ExistingPath },
}

#[derive(Debug, Clone)]
struct ExistingPath {
    inner: std::path::PathBuf,
}

impl std::str::FromStr for ExistingPath {
    type Err = color_eyre::eyre::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let inner = std::path::PathBuf::from_str(value)?;
        if !inner.exists() {
            color_eyre::eyre::bail!("The file does not exist");
        }
        Ok(Self { inner })
    }
}

impl std::convert::AsRef<std::path::Path> for ExistingPath {
    fn as_ref(&self) -> &std::path::Path {
        self.inner.as_ref()
    }
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let command = Command::parse();

    match command {
        Command::YamlToJson { filename } => {
            let yaml_value: serde_yaml::Value =
                serde_yaml::from_reader(std::fs::File::open(&filename).wrap_err_with(|| {
                    format!("Failed to open {filename:?} file to parse YAML content")
                })?)
                .wrap_err_with(|| format!("Failed to parse {filename:?} file as YAML"))?;
            println!("{}", serde_json::to_string_pretty(&yaml_value)?);
        }
        Command::JsonToYaml { filename } => {
            let json_value: serde_json::Value =
                serde_json::from_reader(std::fs::File::open(&filename).wrap_err_with(|| {
                    format!("Failed to open {filename:?} file to parse JSON content")
                })?)
                .wrap_err_with(|| format!("Failed to parse {filename:?} file as JSON"))?;
            println!("{}", serde_yaml::to_string(&json_value)?);
        }
    }

    Ok(())
}
