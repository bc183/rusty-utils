use serde_json::{Error, Value};
use serde_yaml;
use std::io;
use toml;

pub struct FormatCommand {
    pub input: String,
    pub is_file: Option<bool>,
}

impl FormatCommand {
    pub fn new(input: String) -> Self {
        let mut command = FormatCommand {
            input,
            is_file: None,
        };
        command.set_is_file();
        command
    }

    pub fn format_json(&self, contents: &str) -> Result<String, io::Error> {
        // Parse the JSON
        let json_value: Result<Value, Error> = serde_json::from_str(contents);
        match json_value {
            Ok(parsed_json) => {
                let formatted_json = serde_json::to_string_pretty(&parsed_json)?;
                Ok(formatted_json)
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid JSON file",
                ))
            }
        }
    }

    pub fn format_yaml(&self, contents: &str) -> Result<String, io::Error> {
        // Parse the YAML
        let yaml_value: Result<serde_yaml::Value, serde_yaml::Error> =
            serde_yaml::from_str(contents);
        match yaml_value {
            Ok(parsed_yaml) => {
                let formatted_yaml = serde_yaml::to_string(&parsed_yaml)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                Ok(formatted_yaml)
            }
            Err(e) => {
                eprintln!("Error parsing YAML: {}", e);
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid YAML file",
                ))
            }
        }
    }

    pub fn format_toml(&self, contents: &str) -> Result<String, io::Error> {
        // Parse the TOML
        let toml_value: Result<toml::Value, toml::de::Error> = toml::from_str(contents);
        match toml_value {
            Ok(parsed_toml) => {
                let formatted_toml = toml::to_string_pretty(&parsed_toml)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                Ok(formatted_toml)
            }
            Err(e) => {
                eprintln!("Error parsing TOML: {}", e);
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid TOML file",
                ))
            }
        }
    }

    fn set_is_file(&mut self) {
        // check if the given input is a file path
        self.is_file = Some(std::path::Path::new(&self.input).is_file());
    }

    fn get_file_type(&self) -> Result<String, io::Error> {
        let is_file = self.is_file.unwrap_or(false);
        if is_file {
            let file_extension = std::path::Path::new(&self.input)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
            return Ok(file_extension.to_string());
        } else {
            if serde_json::from_str::<serde_json::Value>(&self.input).is_ok() {
                return Ok("json".to_string());
            } else if toml::from_str::<toml::Value>(&self.input).is_ok() {
                return Ok("toml".to_string());
            } else if serde_yaml::from_str::<serde_yaml::Value>(&self.input).is_ok() {
                return Ok("yaml".to_string());
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unsupported input type",
                ));
            }
        }
    }

    pub fn format(&self) -> Result<(), io::Error> {
        let file_type = self.get_file_type()?;
        let is_file = self.is_file.unwrap_or(false);
        let contents = if is_file {
            std::fs::read_to_string(&self.input)?
        } else {
            self.input.clone()
        };

        let formatted_contents = match file_type.as_str() {
            "json" => self.format_json(&contents)?,
            "toml" => self.format_toml(&contents)?,
            "yaml" => self.format_yaml(&contents)?,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unsupported file type",
                ));
            }
        };

        if is_file {
            std::fs::write(&self.input, formatted_contents)?;
        } else {
            println!("{}", formatted_contents);
        }
        Ok(())
    }
}
