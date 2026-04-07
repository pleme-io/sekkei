use std::path::Path;

use anyhow::{Context, Result};

use crate::types::OpenApiSpec;

/// Trait for loading `OpenAPI` specs from various sources.
pub trait SpecLoader: Send + Sync {
    fn load(&self, path: &Path) -> Result<OpenApiSpec>;
}

/// Loads specs from filesystem, auto-detecting JSON or YAML format.
pub struct FileSpecLoader;

impl SpecLoader for FileSpecLoader {
    fn load(&self, path: &Path) -> Result<OpenApiSpec> {
        load_spec(path)
    }
}

/// Load an `OpenAPI` spec from a file, auto-detecting format by extension.
pub fn load_spec(path: &Path) -> Result<OpenApiSpec> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read spec file: {}", path.display()))?;
    load_spec_from_str(&content, path)
}

/// Load an `OpenAPI` spec from a string, using the path extension to determine format.
pub fn load_spec_from_str(content: &str, path: &Path) -> Result<OpenApiSpec> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    match ext {
        "json" => serde_json::from_str(content)
            .with_context(|| format!("failed to parse JSON spec: {}", path.display())),
        "yaml" | "yml" => serde_yaml_ng::from_str(content)
            .with_context(|| format!("failed to parse YAML spec: {}", path.display())),
        _ => {
            // Try JSON first, then YAML
            serde_json::from_str(content)
                .or_else(|_| serde_yaml_ng::from_str(content))
                .with_context(|| {
                    format!(
                        "failed to parse spec (tried JSON and YAML): {}",
                        path.display()
                    )
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    const YAML_SPEC: &str = r#"
info:
  title: YAML API
  version: "1.0.0"
paths: {}
"#;

    const JSON_SPEC: &str = r#"{
  "info": { "title": "JSON API", "version": "2.0.0" },
  "paths": {}
}"#;

    #[test]
    fn load_yaml_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("spec.yaml");
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(YAML_SPEC.as_bytes()).unwrap();

        let spec = load_spec(&path).unwrap();
        assert_eq!(spec.info.title, "YAML API");
        assert_eq!(spec.info.version, "1.0.0");
    }

    #[test]
    fn load_yml_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("spec.yml");
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(YAML_SPEC.as_bytes()).unwrap();

        let spec = load_spec(&path).unwrap();
        assert_eq!(spec.info.title, "YAML API");
    }

    #[test]
    fn load_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("spec.json");
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(JSON_SPEC.as_bytes()).unwrap();

        let spec = load_spec(&path).unwrap();
        assert_eq!(spec.info.title, "JSON API");
        assert_eq!(spec.info.version, "2.0.0");
    }

    #[test]
    fn load_unknown_extension_json() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("spec.txt");
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(JSON_SPEC.as_bytes()).unwrap();

        let spec = load_spec(&path).unwrap();
        assert_eq!(spec.info.title, "JSON API");
    }

    #[test]
    fn load_unknown_extension_yaml() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("spec.txt");
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(YAML_SPEC.as_bytes()).unwrap();

        let spec = load_spec(&path).unwrap();
        assert_eq!(spec.info.title, "YAML API");
    }

    #[test]
    fn load_nonexistent_file_errors() {
        let result = load_spec(Path::new("/nonexistent/path/spec.yaml"));
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("failed to read spec file"));
    }

    #[test]
    fn load_invalid_content_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.json");
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(b"not valid json or yaml {{{{").unwrap();

        let result = load_spec(&path);
        assert!(result.is_err());
    }

    #[test]
    fn load_spec_from_str_yaml() {
        let spec = load_spec_from_str(YAML_SPEC, Path::new("test.yaml")).unwrap();
        assert_eq!(spec.info.title, "YAML API");
    }

    #[test]
    fn load_spec_from_str_json() {
        let spec = load_spec_from_str(JSON_SPEC, Path::new("test.json")).unwrap();
        assert_eq!(spec.info.title, "JSON API");
    }

    #[test]
    fn file_spec_loader_trait() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("spec.yaml");
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(YAML_SPEC.as_bytes()).unwrap();

        let loader = FileSpecLoader;
        let spec = loader.load(&path).unwrap();
        assert_eq!(spec.info.title, "YAML API");
    }

    #[test]
    fn file_spec_loader_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<FileSpecLoader>();
    }

    #[test]
    fn load_spec_unknown_extension_tries_both() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("spec.txt");
        std::fs::write(
            &path,
            r#"{"info":{"title":"Test","version":"1.0"},"paths":{}}"#,
        )
        .unwrap();
        let spec = load_spec(&path).unwrap();
        assert_eq!(spec.info.title, "Test");
    }

    #[test]
    fn load_spec_from_str_unknown_extension_json() {
        let spec = load_spec_from_str(
            r#"{"info":{"title":"FromStr","version":"1.0"},"paths":{}}"#,
            Path::new("spec.unknown"),
        )
        .unwrap();
        assert_eq!(spec.info.title, "FromStr");
    }

    #[test]
    fn load_spec_from_str_unknown_extension_yaml() {
        let spec = load_spec_from_str(
            "info:\n  title: YamlStr\n  version: '1'\npaths: {}",
            Path::new("spec.unknown"),
        )
        .unwrap();
        assert_eq!(spec.info.title, "YamlStr");
    }
}
