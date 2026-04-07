use std::path::Path;

use crate::error::SpecError;
use crate::types::OpenApiSpec;

/// Trait for loading `OpenAPI` specs from various sources.
///
/// Implementors must be thread-safe (`Send + Sync`).
pub trait SpecLoader: Send + Sync {
    /// Load an `OpenAPI` spec from the given path.
    ///
    /// # Errors
    ///
    /// Returns [`SpecError`] if the file cannot be read or parsed.
    fn load(&self, path: &Path) -> Result<OpenApiSpec, SpecError>;
}

/// Loads specs from filesystem, auto-detecting JSON or YAML format.
#[derive(Debug, Default, Clone, Copy)]
pub struct FileSpecLoader;

impl SpecLoader for FileSpecLoader {
    fn load(&self, path: &Path) -> Result<OpenApiSpec, SpecError> {
        load_spec(path)
    }
}

/// Load an `OpenAPI` spec from a file, auto-detecting format by extension.
#[must_use = "this returns a Result that should be checked"]
pub fn load_spec(path: impl AsRef<Path>) -> Result<OpenApiSpec, SpecError> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path).map_err(|source| SpecError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    load_spec_from_str(&content, path)
}

/// Load an `OpenAPI` spec from a string, using the path extension to determine format.
#[must_use = "this returns a Result that should be checked"]
pub fn load_spec_from_str(content: &str, path: impl AsRef<Path>) -> Result<OpenApiSpec, SpecError> {
    let path = path.as_ref();
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    match ext {
        "json" => serde_json::from_str(content).map_err(|source| SpecError::ParseJson {
            path: path.to_path_buf(),
            source,
        }),
        "yaml" | "yml" => {
            serde_yaml_ng::from_str(content).map_err(|source| SpecError::ParseYaml {
                path: path.to_path_buf(),
                source,
            })
        }
        _ => serde_json::from_str(content).or_else(|json_error| {
            serde_yaml_ng::from_str(content).map_err(|yaml_error| {
                SpecError::ParseUnknownFormat {
                    path: path.to_path_buf(),
                    json_error,
                    yaml_error,
                }
            })
        }),
    }
}

/// In-memory spec loader for testing. Returns a pre-configured spec or error.
///
/// ```
/// # use sekkei::{MockSpecLoader, SpecLoader, OpenApiSpec};
/// # use std::path::Path;
/// let spec: OpenApiSpec = serde_json::from_str(
///     r#"{"info":{"title":"T","version":"1"},"paths":{}}"#
/// ).unwrap();
/// let loader = MockSpecLoader::new(spec);
/// assert!(loader.load(Path::new("any.json")).is_ok());
/// ```
#[cfg(any(test, feature = "test-support"))]
#[derive(Debug, Clone)]
pub struct MockSpecLoader {
    result: Result<OpenApiSpec, String>,
}

#[cfg(any(test, feature = "test-support"))]
impl MockSpecLoader {
    /// Create a mock that always returns the given spec.
    #[must_use]
    pub fn new(spec: OpenApiSpec) -> Self {
        Self { result: Ok(spec) }
    }

    /// Create a mock that always returns the given error message.
    #[must_use]
    pub fn failing(msg: impl Into<String>) -> Self {
        Self {
            result: Err(msg.into()),
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl SpecLoader for MockSpecLoader {
    fn load(&self, path: &Path) -> Result<OpenApiSpec, SpecError> {
        match &self.result {
            Ok(spec) => Ok(spec.clone()),
            Err(msg) => Err(SpecError::ReadFile {
                path: path.to_path_buf(),
                source: std::io::Error::new(std::io::ErrorKind::Other, msg.clone()),
            }),
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

    #[test]
    fn load_nonexistent_file_returns_read_file_error() {
        let result = load_spec(Path::new("/does/not/exist.yaml"));
        match result.unwrap_err() {
            SpecError::ReadFile { path, source } => {
                assert_eq!(path, Path::new("/does/not/exist.yaml"));
                assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
            }
            other => panic!("expected ReadFile, got: {other:?}"),
        }
    }

    #[test]
    fn load_invalid_json_file_returns_parse_json_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, "not json at all").unwrap();

        match load_spec(&path).unwrap_err() {
            SpecError::ParseJson {
                path: err_path,
                source: _,
            } => {
                assert_eq!(err_path, path);
            }
            other => panic!("expected ParseJson, got: {other:?}"),
        }
    }

    #[test]
    fn load_invalid_yaml_file_returns_parse_yaml_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.yaml");
        std::fs::write(&path, ":\n  :\n    : [[[").unwrap();

        match load_spec(&path).unwrap_err() {
            SpecError::ParseYaml {
                path: err_path,
                source: _,
            } => {
                assert_eq!(err_path, path);
            }
            other => panic!("expected ParseYaml, got: {other:?}"),
        }
    }

    #[test]
    fn load_invalid_unknown_ext_returns_parse_unknown_format_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.txt");
        std::fs::write(&path, "definitely not valid {{{{ anything").unwrap();

        match load_spec(&path).unwrap_err() {
            SpecError::ParseUnknownFormat {
                path: err_path,
                json_error: _,
                yaml_error: _,
            } => {
                assert_eq!(err_path, path);
            }
            other => panic!("expected ParseUnknownFormat, got: {other:?}"),
        }
    }

    #[test]
    fn load_spec_from_str_invalid_json_extension() {
        let result = load_spec_from_str("{{bad}}", Path::new("spec.json"));
        assert!(matches!(result, Err(SpecError::ParseJson { .. })));
    }

    #[test]
    fn load_spec_from_str_invalid_yaml_extension() {
        let result = load_spec_from_str(":\n  :\n    : [[[", Path::new("spec.yaml"));
        assert!(matches!(result, Err(SpecError::ParseYaml { .. })));
    }

    #[test]
    fn load_spec_from_str_invalid_yml_extension() {
        let result = load_spec_from_str(":\n  :\n    : [[[", Path::new("spec.yml"));
        assert!(matches!(result, Err(SpecError::ParseYaml { .. })));
    }

    #[test]
    fn load_spec_from_str_no_extension() {
        let spec = load_spec_from_str(
            r#"{"info":{"title":"NoExt","version":"1.0"},"paths":{}}"#,
            Path::new("spec"),
        )
        .unwrap();
        assert_eq!(spec.info.title, "NoExt");
    }

    #[test]
    fn load_spec_from_str_empty_content_json() {
        let result = load_spec_from_str("", Path::new("empty.json"));
        assert!(result.is_err());
    }

    #[test]
    fn load_spec_from_str_empty_content_yaml() {
        let result = load_spec_from_str("", Path::new("empty.yaml"));
        assert!(result.is_err());
    }

    // ── MockSpecLoader tests ────────────────────────────────────

    #[test]
    fn mock_spec_loader_returns_spec() {
        let spec: OpenApiSpec =
            serde_json::from_str(r#"{"info":{"title":"Mock","version":"1.0"},"paths":{}}"#)
                .unwrap();
        let loader = MockSpecLoader::new(spec);
        let result = loader.load(Path::new("any.yaml")).unwrap();
        assert_eq!(result.info.title, "Mock");
    }

    #[test]
    fn mock_spec_loader_returns_error() {
        let loader = MockSpecLoader::failing("test failure");
        let err = loader.load(Path::new("fail.yaml")).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("fail.yaml"));
    }

    #[test]
    fn mock_spec_loader_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MockSpecLoader>();
    }

    #[test]
    fn mock_spec_loader_implements_spec_loader_trait() {
        let spec: OpenApiSpec =
            serde_json::from_str(r#"{"info":{"title":"T","version":"1"},"paths":{}}"#).unwrap();
        let loader: Box<dyn SpecLoader> = Box::new(MockSpecLoader::new(spec));
        assert!(loader.load(Path::new("test.json")).is_ok());
    }
}
