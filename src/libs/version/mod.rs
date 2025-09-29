mod errors;

use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;
use std::fs;
use std::path::Path;
use toml::Value as TomlValue;

pub use errors::VersionError;

#[derive(Debug)]
enum FileFormat {
    Json,
    Toml,
    Yaml,
}

fn detect_format(file_path: &str) -> Result<FileFormat, VersionError> {
    let path = Path::new(file_path);
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    match extension.to_lowercase().as_str() {
        "json" => Ok(FileFormat::Json),
        "toml" => Ok(FileFormat::Toml),
        "yaml" | "yml" => Ok(FileFormat::Yaml),
        _ => Err(VersionError::UnsupportedFormat(format!(
            "Unsupported file extension: '{}'. Supported: json, toml, yaml, yml",
            extension
        ))),
    }
}

fn read_json(value: &JsonValue, param_path: &str) -> Option<String> {
    let parts: Vec<&str> = param_path.split('.').collect();
    let mut current = value;

    for part in parts {
        current = current.get(part)?;
    }

    match current {
        JsonValue::String(s) => Some(s.clone()),
        JsonValue::Number(n) => Some(n.to_string()),
        JsonValue::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn read_toml(value: &TomlValue, param_path: &str) -> Option<String> {
    let parts: Vec<&str> = param_path.split('.').collect();
    let mut current = value;

    for part in parts {
        current = match current {
            TomlValue::Table(table) => table.get(part)?,
            _ => return None,
        };
    }

    match current {
        TomlValue::String(s) => Some(s.clone()),
        TomlValue::Integer(n) => Some(n.to_string()),
        TomlValue::Float(f) => Some(f.to_string()),
        TomlValue::Boolean(b) => Some(b.to_string()),
        _ => None,
    }
}

fn read_yaml(value: &YamlValue, param_path: &str) -> Option<String> {
    let parts: Vec<&str> = param_path.split('.').collect();
    let mut current = value;

    for part in parts {
        current = current.get(part)?;
    }

    match current {
        YamlValue::String(s) => Some(s.clone()),
        YamlValue::Number(n) => Some(n.to_string()),
        YamlValue::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

pub fn read_version(file_path: &str, param_name: &str) -> Result<String, VersionError> {
    let format = detect_format(file_path)?;

    let content = fs::read_to_string(file_path)?;

    let result = match format {
        FileFormat::Json => {
            let value: JsonValue = serde_json::from_str(&content)
                .map_err(|e| VersionError::ParseError(format!("JSON parse error: {}", e)))?;
            read_json(&value, param_name)
        }

        FileFormat::Toml => {
            let value: TomlValue = toml::de::from_str(&content)
                .map_err(|e| VersionError::ParseError(format!("TOML parse error: {}", e)))?;
            read_toml(&value, param_name)
        }

        FileFormat::Yaml => {
            let value: YamlValue = serde_yaml_ng::from_str(&content)
                .map_err(|e| VersionError::ParseError(format!("YAML parse error: {}", e)))?;
            read_yaml(&value, param_name)
        }
    };

    result.ok_or_else(|| VersionError::ParamNotFound(param_name.to_string()))
}

#[cfg(test)]
mod tests;
