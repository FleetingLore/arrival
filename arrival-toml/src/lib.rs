use arrival_core::Runtime;
use arrival_serde::SerdeRuntime;
use std::fs;

pub fn from_file(path: &str) -> Result<Runtime, TomlError> {
    let content = fs::read_to_string(path).map_err(TomlError::Io)?;
    from_str(&content)
}

pub fn from_str(content: &str) -> Result<Runtime, TomlError> {
    let serde_runtime: SerdeRuntime = toml::from_str(content).map_err(TomlError::Parse)?;
    Ok(serde_runtime.to_runtime())
}

#[derive(Debug)]
pub enum TomlError {
    Io(std::io::Error),
    Parse(toml::de::Error),
}

impl std::fmt::Display for TomlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlError::Io(e) => write!(f, "IO error: {}", e),
            TomlError::Parse(e) => write!(f, "TOML parse error: {}", e),
        }
    }
}

impl std::error::Error for TomlError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let toml = r#"
            [[nodes]]
            path = { nodes = ["root"] }
            next = { nodes = ["root", "child"] }

            [[nodes]]
            path = { nodes = ["root", "child"] }
            result = { String = "hello" }
        "#;

        let runtime = from_str(toml).unwrap();
        assert!(runtime.iter_nodes().count() == 2);
    }
}
