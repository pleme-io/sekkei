//! Typed error variants for spec loading and parsing.

use std::path::PathBuf;

/// Errors that can occur when loading or parsing an `OpenAPI` spec.
#[derive(Debug, thiserror::Error)]
pub enum SpecError {
    /// Failed to read a spec file from disk.
    #[error("failed to read spec file: {path}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse a JSON spec.
    #[error("failed to parse JSON spec: {path}")]
    ParseJson {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    /// Failed to parse a YAML spec.
    #[error("failed to parse YAML spec: {path}")]
    ParseYaml {
        path: PathBuf,
        #[source]
        source: serde_yaml_ng::Error,
    },

    /// Failed to parse a spec with unknown extension (tried both JSON and YAML).
    #[error("failed to parse spec (tried JSON and YAML): {path}")]
    ParseUnknownFormat {
        path: PathBuf,
        json_error: serde_json::Error,
        yaml_error: serde_yaml_ng::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_error_display() {
        let err = SpecError::ReadFile {
            path: PathBuf::from("/tmp/missing.yaml"),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
        };
        let msg = err.to_string();
        assert!(msg.contains("failed to read spec file"));
        assert!(msg.contains("/tmp/missing.yaml"));
    }

    #[test]
    fn read_file_error_source_chain() {
        use std::error::Error;
        let err = SpecError::ReadFile {
            path: PathBuf::from("spec.yaml"),
            source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
        };
        let source = err.source().unwrap();
        assert!(source.to_string().contains("denied"));
    }

    #[test]
    fn parse_json_error_display() {
        let json_err = serde_json::from_str::<serde_json::Value>("{{bad}}")
            .unwrap_err();
        let err = SpecError::ParseJson {
            path: PathBuf::from("bad.json"),
            source: json_err,
        };
        let msg = err.to_string();
        assert!(msg.contains("failed to parse JSON spec"));
        assert!(msg.contains("bad.json"));
    }

    #[test]
    fn parse_yaml_error_display() {
        let yaml_err = serde_yaml_ng::from_str::<serde_json::Value>(":\n  :\n    :")
            .unwrap_err();
        let err = SpecError::ParseYaml {
            path: PathBuf::from("bad.yaml"),
            source: yaml_err,
        };
        let msg = err.to_string();
        assert!(msg.contains("failed to parse YAML spec"));
        assert!(msg.contains("bad.yaml"));
    }

    #[test]
    fn parse_unknown_format_error_display() {
        let json_err = serde_json::from_str::<serde_json::Value>("not json")
            .unwrap_err();
        let yaml_err = serde_yaml_ng::from_str::<serde_json::Value>(":\n  :\n    :")
            .unwrap_err();
        let err = SpecError::ParseUnknownFormat {
            path: PathBuf::from("mystery.txt"),
            json_error: json_err,
            yaml_error: yaml_err,
        };
        let msg = err.to_string();
        assert!(msg.contains("tried JSON and YAML"));
        assert!(msg.contains("mystery.txt"));
    }

    #[test]
    fn spec_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SpecError>();
    }

    #[test]
    fn spec_error_debug_format() {
        let err = SpecError::ReadFile {
            path: PathBuf::from("test.yaml"),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "missing"),
        };
        let debug = format!("{err:?}");
        assert!(debug.contains("ReadFile"));
        assert!(debug.contains("test.yaml"));
    }
}
