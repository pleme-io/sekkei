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
