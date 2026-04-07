//! `OpenAPI` 3.0.3 serde types — canonical definitions for the pleme-io ecosystem.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// ── Root ───────────────────────────────────────────────────────────────────

/// Root `OpenAPI` 3.0 specification object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub info: Info,
    #[serde(default)]
    pub paths: BTreeMap<String, PathItem>,
    #[serde(default)]
    pub components: Option<Components>,
    #[serde(default)]
    pub servers: Vec<Server>,
    #[serde(default)]
    pub security: Vec<BTreeMap<String, Vec<String>>>,
}

// ── Info ───────────────────────────────────────────────────────────────────

/// Metadata about the API (title, version, description).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub version: String,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} v{}", self.title, self.version)
    }
}

// ── Paths & Operations ────────────────────────────────────────────────────

/// Describes the operations available on a single path.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PathItem {
    #[serde(default)]
    pub get: Option<Operation>,
    #[serde(default)]
    pub post: Option<Operation>,
    #[serde(default)]
    pub put: Option<Operation>,
    #[serde(default)]
    pub delete: Option<Operation>,
    #[serde(default)]
    pub patch: Option<Operation>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

/// A single API operation on a path (e.g. GET, POST).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    #[serde(default)]
    pub operation_id: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    #[serde(default)]
    pub request_body: Option<RequestBody>,
    #[serde(default)]
    pub responses: BTreeMap<String, Response>,
    #[serde(default)]
    pub security: Vec<BTreeMap<String, Vec<String>>>,
    #[serde(default)]
    pub tags: Vec<String>,
}

// ── Parameters ─────────────────────────────────────────────────────────────

/// Describes a single operation parameter (query, path, header, cookie).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub schema: Option<Schema>,
    /// $ref pointer, e.g. "#/components/parameters/Foo"
    #[serde(rename = "$ref", default)]
    pub ref_path: Option<String>,
}

// ── Request / Response Bodies ──────────────────────────────────────────────

/// Describes a request body with content type mappings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub content: BTreeMap<String, MediaType>,
    #[serde(default)]
    pub description: Option<String>,
    /// $ref pointer
    #[serde(rename = "$ref", default)]
    pub ref_path: Option<String>,
}

/// A media type (e.g. `application/json`) with an optional schema.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaType {
    #[serde(default)]
    pub schema: Option<Schema>,
}

/// Describes a single response from an API operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub content: Option<BTreeMap<String, MediaType>>,
    /// $ref pointer
    #[serde(rename = "$ref", default)]
    pub ref_path: Option<String>,
}

// ── Schema ─────────────────────────────────────────────────────────────────

/// JSON Schema object used to describe data types in `OpenAPI` specs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "type", default)]
    pub schema_type: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub properties: BTreeMap<String, Schema>,
    #[serde(default)]
    pub items: Option<Box<Schema>>,
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(rename = "enum", default)]
    pub enum_values: Option<Vec<serde_json::Value>>,
    /// $ref pointer, e.g. "#/components/schemas/Foo"
    #[serde(rename = "$ref", default)]
    pub ref_path: Option<String>,
    #[serde(rename = "allOf", default)]
    pub all_of: Vec<Schema>,
    #[serde(rename = "oneOf", default)]
    pub one_of: Vec<Schema>,
    #[serde(rename = "anyOf", default)]
    pub any_of: Vec<Schema>,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    pub minimum: Option<f64>,
    #[serde(default)]
    pub maximum: Option<f64>,
    #[serde(rename = "minLength", default)]
    pub min_length: Option<u64>,
    #[serde(rename = "maxLength", default)]
    pub max_length: Option<u64>,
    #[serde(default)]
    pub nullable: bool,
    #[serde(default)]
    pub additional_properties: Option<Box<Schema>>,
    #[serde(default)]
    pub title: Option<String>,
}

// ── Components ─────────────────────────────────────────────────────────────

/// Holds reusable schema, parameter, request body, and response definitions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    #[serde(default)]
    pub schemas: BTreeMap<String, Schema>,
    #[serde(default)]
    pub security_schemes: BTreeMap<String, SecurityScheme>,
    #[serde(default)]
    pub parameters: BTreeMap<String, Parameter>,
    #[serde(default)]
    pub request_bodies: BTreeMap<String, RequestBody>,
    #[serde(default)]
    pub responses: BTreeMap<String, Response>,
}

/// Defines a security scheme that can be used by the operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScheme {
    #[serde(rename = "type")]
    pub scheme_type: String,
    #[serde(default)]
    pub scheme: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    /// For apiKey type: "header", "query", or "cookie"
    #[serde(rename = "in", default)]
    pub location: Option<String>,
    /// For apiKey type: the header/query parameter name
    #[serde(default)]
    pub name: Option<String>,
}

// ── Server ─────────────────────────────────────────────────────────────────

/// An object representing a server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl std::fmt::Display for OpenApiSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OpenAPI spec: {} ({} paths, {} operations)",
            self.info,
            self.paths.len(),
            self.operation_count()
        )
    }
}

// ── PathItem helpers ──────────────────────────────────────────────────

const HTTP_METHODS: [&str; 5] = ["get", "post", "put", "delete", "patch"];

impl PathItem {
    /// Iterate over all defined operations in this path item as `(method, &Operation)` pairs.
    ///
    /// Methods are yielded in standard order: GET, POST, PUT, DELETE, PATCH.
    pub fn operations(&self) -> impl Iterator<Item = (&'static str, &Operation)> {
        HTTP_METHODS
            .into_iter()
            .zip([
                self.get.as_ref(),
                self.post.as_ref(),
                self.put.as_ref(),
                self.delete.as_ref(),
                self.patch.as_ref(),
            ])
            .filter_map(|(method, op)| op.map(|o| (method, o)))
    }
}

// ── Schema helpers ────────────────────────────────────────────────────

impl Schema {
    /// Check if this schema is a `$ref`.
    #[must_use]
    pub fn is_ref(&self) -> bool {
        self.ref_path.is_some()
    }

    /// Check if this is an array type.
    #[must_use]
    pub fn is_array(&self) -> bool {
        self.schema_type.as_deref() == Some("array")
    }

    /// Check if this is an object type.
    #[must_use]
    pub fn is_object(&self) -> bool {
        self.schema_type.as_deref() == Some("object")
    }

    /// Check if this is a primitive type (string, integer, number, boolean).
    #[must_use]
    pub fn is_primitive(&self) -> bool {
        matches!(
            self.schema_type.as_deref(),
            Some("string" | "integer" | "number" | "boolean")
        )
    }

    /// Check if this schema has enum values.
    #[must_use]
    pub fn is_enum(&self) -> bool {
        self.enum_values.as_ref().is_some_and(|v| !v.is_empty())
    }

    /// Get the ref name if this is a `$ref` schema.
    #[must_use]
    pub fn ref_name(&self) -> Option<&str> {
        self.ref_path.as_deref().map(ref_name)
    }
}

// ── Operation helpers ─────────────────────────────────────────────────

impl Operation {
    /// Get the JSON body schema from `request_body`, if any.
    #[must_use]
    pub fn json_body_schema(&self) -> Option<&Schema> {
        self.request_body
            .as_ref()?
            .content
            .get("application/json")?
            .schema
            .as_ref()
    }

    /// Get the success response schema (200 or 201).
    #[must_use]
    pub fn success_response_schema(&self) -> Option<&Schema> {
        let resp = self
            .responses
            .get("200")
            .or_else(|| self.responses.get("201"))?;
        resp.content
            .as_ref()?
            .get("application/json")?
            .schema
            .as_ref()
    }
}

// ── $ref resolution helpers ────────────────────────────────────────────────

/// Extract the final component name from a JSON pointer.
///
/// ```text
/// "#/components/schemas/Pet" → "Pet"
/// "#/components/parameters/LimitParam" → "LimitParam"
/// ```
#[must_use]
pub fn ref_name(ref_path: &str) -> &str {
    ref_path.rsplit('/').next().unwrap_or(ref_path)
}

impl OpenApiSpec {
    /// Look up a schema by `$ref` pointer like `#/components/schemas/Foo`.
    #[must_use]
    pub fn resolve_schema_ref(&self, ref_path: &str) -> Option<&Schema> {
        let name = ref_name(ref_path);
        self.components.as_ref()?.schemas.get(name)
    }

    /// Look up a parameter by `$ref` pointer like `#/components/parameters/Foo`.
    #[must_use]
    pub fn resolve_parameter_ref(&self, ref_path: &str) -> Option<&Parameter> {
        let name = ref_name(ref_path);
        self.components.as_ref()?.parameters.get(name)
    }

    /// Look up a request body by `$ref` pointer.
    #[must_use]
    pub fn resolve_request_body_ref(&self, ref_path: &str) -> Option<&RequestBody> {
        let name = ref_name(ref_path);
        self.components.as_ref()?.request_bodies.get(name)
    }

    /// Look up a response by `$ref` pointer.
    #[must_use]
    pub fn resolve_response_ref(&self, ref_path: &str) -> Option<&Response> {
        let name = ref_name(ref_path);
        self.components.as_ref()?.responses.get(name)
    }
}

/// Enumerate all operations across all paths.
/// Returns `(method, path, &Operation)` triples.
///
/// Prefer [`OpenApiSpec::all_operations`] for new code.
#[must_use]
pub fn all_operations(spec: &OpenApiSpec) -> Vec<(String, String, &Operation)> {
    spec.all_operations().collect()
}

impl OpenApiSpec {
    /// Iterate over every operation in the spec as `(method, path, &Operation)` triples.
    pub fn all_operations(&self) -> impl Iterator<Item = (String, String, &Operation)> {
        self.paths.iter().flat_map(|(path, item)| {
            let path = path.clone();
            item.operations()
                .map(move |(method, op)| (method.to_string(), path.clone(), op))
        })
    }

    /// Returns the number of named schemas in `components.schemas`.
    #[must_use]
    pub fn schema_count(&self) -> usize {
        self.components
            .as_ref()
            .map_or(0, |c| c.schemas.len())
    }

    /// Returns the total number of operations across all paths.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.paths.values().map(|item| item.operations().count()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fixtures::{FULL_SPEC_YAML, MINIMAL_SPEC_YAML};

    #[test]
    fn parse_minimal_yaml() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(MINIMAL_SPEC_YAML).unwrap();
        assert_eq!(spec.info.title, "Test API");
        assert_eq!(spec.info.version, "1.0.0");
        assert!(spec.paths.is_empty());
        assert!(spec.components.is_none());
        assert!(spec.servers.is_empty());
    }

    #[test]
    fn parse_full_yaml() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        assert_eq!(spec.info.title, "Pet Store");
        assert_eq!(
            spec.info.description.as_deref(),
            Some("A sample pet store API")
        );
        assert_eq!(spec.info.version, "2.0.0");
        assert_eq!(spec.servers.len(), 1);
        assert_eq!(spec.servers[0].url, "https://api.petstore.example.com/v2");
        assert_eq!(spec.paths.len(), 2);
        assert!(spec.paths.contains_key("/pets"));
        assert!(spec.paths.contains_key("/pets/{petId}"));
    }

    #[test]
    fn parse_path_item_methods() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let pets = &spec.paths["/pets"];
        assert!(pets.get.is_some());
        assert!(pets.post.is_some());
        assert!(pets.put.is_none());
        assert!(pets.delete.is_none());

        let pet_by_id = &spec.paths["/pets/{petId}"];
        assert!(pet_by_id.get.is_some());
        assert!(pet_by_id.delete.is_some());
        assert!(pet_by_id.post.is_none());
    }

    #[test]
    fn parse_operation_fields() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let list_op = spec.paths["/pets"].get.as_ref().unwrap();
        assert_eq!(list_op.operation_id.as_deref(), Some("listPets"));
        assert_eq!(list_op.summary.as_deref(), Some("List all pets"));
        assert_eq!(list_op.parameters.len(), 1);
        assert_eq!(list_op.parameters[0].name, "limit");
        assert_eq!(list_op.parameters[0].location, "query");
        assert!(!list_op.parameters[0].required);
    }

    #[test]
    fn parse_parameters_with_schema() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let limit_param = &spec.paths["/pets"].get.as_ref().unwrap().parameters[0];
        let schema = limit_param.schema.as_ref().unwrap();
        assert_eq!(schema.schema_type.as_deref(), Some("integer"));
        assert_eq!(schema.format.as_deref(), Some("int64"));
    }

    #[test]
    fn parse_path_level_parameters() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let pet_item = &spec.paths["/pets/{petId}"];
        assert_eq!(pet_item.parameters.len(), 1);
        assert_eq!(pet_item.parameters[0].name, "petId");
        assert_eq!(pet_item.parameters[0].location, "path");
        assert!(pet_item.parameters[0].required);
    }

    #[test]
    fn parse_request_body() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let create_op = spec.paths["/pets"].post.as_ref().unwrap();
        let body = create_op.request_body.as_ref().unwrap();
        assert!(body.required);
        assert!(body.content.contains_key("application/json"));
        let schema = body.content["application/json"].schema.as_ref().unwrap();
        assert_eq!(
            schema.ref_path.as_deref(),
            Some("#/components/schemas/CreatePetRequest")
        );
    }

    #[test]
    fn parse_responses() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let get_op = spec.paths["/pets/{petId}"].get.as_ref().unwrap();
        assert!(get_op.responses.contains_key("200"));
        assert!(get_op.responses.contains_key("404"));
        assert_eq!(
            get_op.responses["404"].description.as_deref(),
            Some("Pet not found")
        );
    }

    #[test]
    fn parse_component_schemas() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let components = spec.components.as_ref().unwrap();
        assert!(components.schemas.contains_key("Pet"));
        assert!(components.schemas.contains_key("PetStatus"));
        assert!(components.schemas.contains_key("CreatePetRequest"));

        let pet = &components.schemas["Pet"];
        assert_eq!(pet.schema_type.as_deref(), Some("object"));
        assert_eq!(pet.required, vec!["id", "name"]);
        assert!(pet.properties.contains_key("id"));
        assert!(pet.properties.contains_key("name"));
        assert!(pet.properties.contains_key("tag"));
        assert!(pet.properties.contains_key("status"));
    }

    #[test]
    fn parse_enum_schema() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let status = &spec.components.as_ref().unwrap().schemas["PetStatus"];
        assert_eq!(status.schema_type.as_deref(), Some("string"));
        let variants = status.enum_values.as_ref().unwrap();
        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0].as_str(), Some("available"));
        assert_eq!(variants[1].as_str(), Some("pending"));
        assert_eq!(variants[2].as_str(), Some("sold"));
    }

    #[test]
    fn parse_security_schemes() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let schemes = &spec.components.as_ref().unwrap().security_schemes;
        assert!(schemes.contains_key("bearerAuth"));
        let bearer = &schemes["bearerAuth"];
        assert_eq!(bearer.scheme_type, "http");
        assert_eq!(bearer.scheme.as_deref(), Some("bearer"));
    }

    #[test]
    fn parse_json_format() {
        let json = r#"{
            "info": { "title": "JSON API", "version": "0.1.0" },
            "paths": {}
        }"#;
        let spec: OpenApiSpec = serde_json::from_str(json).unwrap();
        assert_eq!(spec.info.title, "JSON API");
        assert_eq!(spec.info.version, "0.1.0");
    }

    // -- ref_name tests --

    #[test]
    fn ref_name_extracts_last_segment() {
        assert_eq!(ref_name("#/components/schemas/Pet"), "Pet");
        assert_eq!(
            ref_name("#/components/parameters/LimitParam"),
            "LimitParam"
        );
        assert_eq!(ref_name("#/components/requestBodies/PetBody"), "PetBody");
    }

    #[test]
    fn ref_name_handles_no_slash() {
        assert_eq!(ref_name("Standalone"), "Standalone");
    }

    #[test]
    fn ref_name_handles_trailing_slash() {
        assert_eq!(ref_name("#/components/schemas/"), "");
    }

    // -- resolve_*_ref tests --

    #[test]
    fn resolve_schema_ref_found() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let pet = spec.resolve_schema_ref("#/components/schemas/Pet");
        assert!(pet.is_some());
        assert_eq!(pet.unwrap().schema_type.as_deref(), Some("object"));
    }

    #[test]
    fn resolve_schema_ref_not_found() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        assert!(
            spec.resolve_schema_ref("#/components/schemas/Nonexistent")
                .is_none()
        );
    }

    #[test]
    fn resolve_schema_ref_no_components() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(MINIMAL_SPEC_YAML).unwrap();
        assert!(
            spec.resolve_schema_ref("#/components/schemas/Anything")
                .is_none()
        );
    }

    #[test]
    fn resolve_parameter_ref_found() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let param = spec.resolve_parameter_ref("#/components/parameters/LimitParam");
        assert!(param.is_some());
        assert_eq!(param.unwrap().name, "limit");
    }

    #[test]
    fn resolve_parameter_ref_not_found() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        assert!(
            spec.resolve_parameter_ref("#/components/parameters/Missing")
                .is_none()
        );
    }

    #[test]
    fn resolve_request_body_ref_found() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let body = spec.resolve_request_body_ref("#/components/requestBodies/PetBody");
        assert!(body.is_some());
        assert!(body.unwrap().required);
    }

    #[test]
    fn resolve_response_ref_found() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let resp = spec.resolve_response_ref("#/components/responses/NotFound");
        assert!(resp.is_some());
        assert_eq!(
            resp.unwrap().description.as_deref(),
            Some("The requested resource was not found")
        );
    }

    #[test]
    fn resolve_response_ref_not_found() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        assert!(
            spec.resolve_response_ref("#/components/responses/Gone")
                .is_none()
        );
    }

    // -- Schema edge cases --

    #[test]
    fn parse_schema_with_all_of() {
        let yaml = r##"
info:
  title: AllOf Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    Extended:
      allOf:
        - $ref: "#/components/schemas/Base"
        - type: object
          properties:
            extra:
              type: string
    Base:
      type: object
      required:
        - id
      properties:
        id:
          type: integer
"##;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let extended = &spec.components.as_ref().unwrap().schemas["Extended"];
        assert_eq!(extended.all_of.len(), 2);
        assert_eq!(
            extended.all_of[0].ref_path.as_deref(),
            Some("#/components/schemas/Base")
        );
    }

    #[test]
    fn parse_schema_with_nullable() {
        let yaml = r#"
info:
  title: Nullable Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    NullableField:
      type: object
      properties:
        value:
          type: string
          nullable: true
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schema = &spec.components.as_ref().unwrap().schemas["NullableField"];
        assert!(schema.properties["value"].nullable);
    }

    #[test]
    fn parse_schema_with_additional_properties() {
        let yaml = r#"
info:
  title: AdditionalProps Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    FreeForm:
      type: object
      additionalProperties:
        type: string
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schema = &spec.components.as_ref().unwrap().schemas["FreeForm"];
        assert!(schema.additional_properties.is_some());
        let inner = schema.additional_properties.as_ref().unwrap();
        assert_eq!(inner.schema_type.as_deref(), Some("string"));
    }

    #[test]
    fn parse_schema_array_with_items() {
        let yaml = r#"
info:
  title: Array Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    StringList:
      type: array
      items:
        type: string
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schema = &spec.components.as_ref().unwrap().schemas["StringList"];
        assert_eq!(schema.schema_type.as_deref(), Some("array"));
        let items = schema.items.as_ref().unwrap();
        assert_eq!(items.schema_type.as_deref(), Some("string"));
    }

    #[test]
    fn parse_servers() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        assert_eq!(spec.servers.len(), 1);
        assert_eq!(spec.servers[0].url, "https://api.petstore.example.com/v2");
        assert_eq!(
            spec.servers[0].description.as_deref(),
            Some("Production server")
        );
    }

    #[test]
    fn parse_api_key_security_scheme() {
        let yaml = r#"
info:
  title: ApiKey Test
  version: "1.0.0"
paths: {}
components:
  securitySchemes:
    apiKey:
      type: apiKey
      in: header
      name: X-API-Key
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schemes = &spec.components.as_ref().unwrap().security_schemes;
        let key = &schemes["apiKey"];
        assert_eq!(key.scheme_type, "apiKey");
        assert_eq!(key.location.as_deref(), Some("header"));
        assert_eq!(key.name.as_deref(), Some("X-API-Key"));
    }

    // -- all_operations tests --

    #[test]
    fn all_operations_returns_correct_tuples() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let ops = all_operations(&spec);
        // Full spec has: GET /pets, POST /pets, GET /pets/{petId}, DELETE /pets/{petId}
        assert_eq!(ops.len(), 4);

        let methods_and_paths: Vec<(&str, &str)> =
            ops.iter().map(|(m, p, _)| (m.as_str(), p.as_str())).collect();
        assert!(methods_and_paths.contains(&("get", "/pets")));
        assert!(methods_and_paths.contains(&("post", "/pets")));
        assert!(methods_and_paths.contains(&("get", "/pets/{petId}")));
        assert!(methods_and_paths.contains(&("delete", "/pets/{petId}")));
    }

    #[test]
    fn all_operations_empty_paths() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(MINIMAL_SPEC_YAML).unwrap();
        let ops = all_operations(&spec);
        assert!(ops.is_empty());
    }

    #[test]
    fn all_operations_preserves_operation_ids() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let ops = all_operations(&spec);
        let ids: Vec<Option<&str>> = ops
            .iter()
            .map(|(_, _, op)| op.operation_id.as_deref())
            .collect();
        assert!(ids.contains(&Some("listPets")));
        assert!(ids.contains(&Some("createPet")));
        assert!(ids.contains(&Some("getPet")));
        assert!(ids.contains(&Some("deletePet")));
    }

    // -- global security --

    #[test]
    fn parse_global_security() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        assert_eq!(spec.security.len(), 1);
        assert!(spec.security[0].contains_key("bearerAuth"));
        assert!(spec.security[0]["bearerAuth"].is_empty());
    }

    // -- Schema min/max constraints --

    #[test]
    fn parse_schema_with_min_max() {
        let yaml = r#"
info:
  title: MinMax Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    Bounded:
      type: integer
      minimum: 1
      maximum: 100
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schema = &spec.components.as_ref().unwrap().schemas["Bounded"];
        assert_eq!(schema.minimum, Some(1.0));
        assert_eq!(schema.maximum, Some(100.0));
    }

    #[test]
    fn parse_schema_with_string_length_constraints() {
        let yaml = r#"
info:
  title: StringLength Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    BoundedString:
      type: string
      minLength: 3
      maxLength: 255
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schema = &spec.components.as_ref().unwrap().schemas["BoundedString"];
        assert_eq!(schema.min_length, Some(3));
        assert_eq!(schema.max_length, Some(255));
    }

    #[test]
    fn parse_schema_with_default_value() {
        let yaml = r#"
info:
  title: Default Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    WithDefault:
      type: string
      default: "hello"
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schema = &spec.components.as_ref().unwrap().schemas["WithDefault"];
        assert_eq!(
            schema.default.as_ref().and_then(|v| v.as_str()),
            Some("hello")
        );
    }

    #[test]
    fn parse_schema_with_title() {
        let yaml = r#"
info:
  title: Title Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    Named:
      type: object
      title: A Named Schema
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schema = &spec.components.as_ref().unwrap().schemas["Named"];
        assert_eq!(schema.title.as_deref(), Some("A Named Schema"));
    }

    #[test]
    fn parse_schema_with_one_of() {
        let yaml = r##"
info:
  title: OneOf Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    StringOrInt:
      oneOf:
        - type: string
        - type: integer
"##;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schema = &spec.components.as_ref().unwrap().schemas["StringOrInt"];
        assert_eq!(schema.one_of.len(), 2);
        assert_eq!(schema.one_of[0].schema_type.as_deref(), Some("string"));
        assert_eq!(schema.one_of[1].schema_type.as_deref(), Some("integer"));
    }

    #[test]
    fn parse_schema_with_any_of() {
        let yaml = r##"
info:
  title: AnyOf Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    Flexible:
      anyOf:
        - type: string
        - type: number
"##;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schema = &spec.components.as_ref().unwrap().schemas["Flexible"];
        assert_eq!(schema.any_of.len(), 2);
    }

    // -- Schema ref inside properties --

    #[test]
    fn parse_property_with_ref() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let pet = &spec.components.as_ref().unwrap().schemas["Pet"];
        let status_prop = &pet.properties["status"];
        assert_eq!(
            status_prop.ref_path.as_deref(),
            Some("#/components/schemas/PetStatus")
        );
    }

    // -- Component parameter ref --

    #[test]
    fn parse_component_parameters() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let params = &spec.components.as_ref().unwrap().parameters;
        assert!(params.contains_key("LimitParam"));
        let limit = &params["LimitParam"];
        assert_eq!(limit.name, "limit");
        assert_eq!(limit.location, "query");
        assert!(!limit.required);
    }

    // -- Component request bodies --

    #[test]
    fn parse_component_request_bodies() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let bodies = &spec.components.as_ref().unwrap().request_bodies;
        assert!(bodies.contains_key("PetBody"));
        let pet_body = &bodies["PetBody"];
        assert!(pet_body.required);
        assert!(pet_body.content.contains_key("application/json"));
    }

    // -- Component responses --

    #[test]
    fn parse_component_responses() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let responses = &spec.components.as_ref().unwrap().responses;
        assert!(responses.contains_key("NotFound"));
        assert_eq!(
            responses["NotFound"].description.as_deref(),
            Some("The requested resource was not found")
        );
    }

    // -- Roundtrip serialization --

    #[test]
    fn roundtrip_json_serialization() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let json = serde_json::to_string(&spec).unwrap();
        let roundtrip: OpenApiSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtrip.info.title, spec.info.title);
        assert_eq!(roundtrip.paths.len(), spec.paths.len());
    }

    // -- Schema Default trait --

    #[test]
    fn schema_default_is_empty() {
        let schema = Schema::default();
        assert!(schema.schema_type.is_none());
        assert!(schema.properties.is_empty());
        assert!(schema.required.is_empty());
        assert!(!schema.nullable);
        assert!(schema.all_of.is_empty());
        assert!(schema.one_of.is_empty());
        assert!(schema.any_of.is_empty());
    }

    // -- Components Default trait --

    #[test]
    fn components_default_is_empty() {
        let components = Components::default();
        assert!(components.schemas.is_empty());
        assert!(components.security_schemes.is_empty());
        assert!(components.parameters.is_empty());
        assert!(components.request_bodies.is_empty());
        assert!(components.responses.is_empty());
    }

    // -- Operation tags --

    #[test]
    fn parse_operation_with_tags() {
        let yaml = r#"
info:
  title: Tags Test
  version: "1.0.0"
paths:
  /items:
    get:
      operationId: listItems
      tags:
        - items
        - public
      responses:
        "200":
          description: OK
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let op = spec.paths["/items"].get.as_ref().unwrap();
        assert_eq!(op.tags, vec!["items", "public"]);
    }

    // ── Schema helper tests ─────────────────────────────────────

    #[test]
    fn schema_is_ref() {
        let s = Schema {
            ref_path: Some("#/components/schemas/Pet".to_string()),
            ..Default::default()
        };
        assert!(s.is_ref());
        assert_eq!(s.ref_name(), Some("Pet"));
    }

    #[test]
    fn schema_is_not_ref() {
        let s = Schema::default();
        assert!(!s.is_ref());
        assert_eq!(s.ref_name(), None);
    }

    #[test]
    fn schema_is_array() {
        let s = Schema {
            schema_type: Some("array".to_string()),
            ..Default::default()
        };
        assert!(s.is_array());
        assert!(!s.is_object());
        assert!(!s.is_primitive());
    }

    #[test]
    fn schema_is_object() {
        let s = Schema {
            schema_type: Some("object".to_string()),
            ..Default::default()
        };
        assert!(s.is_object());
        assert!(!s.is_array());
        assert!(!s.is_primitive());
    }

    #[test]
    fn schema_is_primitive_string() {
        let s = Schema {
            schema_type: Some("string".to_string()),
            ..Default::default()
        };
        assert!(s.is_primitive());
        assert!(!s.is_object());
        assert!(!s.is_array());
    }

    #[test]
    fn schema_is_primitive_integer() {
        let s = Schema {
            schema_type: Some("integer".to_string()),
            ..Default::default()
        };
        assert!(s.is_primitive());
    }

    #[test]
    fn schema_is_primitive_number() {
        let s = Schema {
            schema_type: Some("number".to_string()),
            ..Default::default()
        };
        assert!(s.is_primitive());
    }

    #[test]
    fn schema_is_primitive_boolean() {
        let s = Schema {
            schema_type: Some("boolean".to_string()),
            ..Default::default()
        };
        assert!(s.is_primitive());
    }

    #[test]
    fn schema_is_enum() {
        let s = Schema {
            enum_values: Some(vec![serde_json::Value::String("a".to_string())]),
            ..Default::default()
        };
        assert!(s.is_enum());
    }

    #[test]
    fn schema_is_not_enum_empty() {
        let s = Schema {
            enum_values: Some(vec![]),
            ..Default::default()
        };
        assert!(!s.is_enum());
    }

    #[test]
    fn schema_is_not_enum_none() {
        let s = Schema::default();
        assert!(!s.is_enum());
    }

    #[test]
    fn schema_ref_name_nested_path() {
        let s = Schema {
            ref_path: Some("#/components/schemas/deeply/Nested".to_string()),
            ..Default::default()
        };
        assert_eq!(s.ref_name(), Some("Nested"));
    }

    #[test]
    fn schema_no_type_is_not_primitive_or_array_or_object() {
        let s = Schema::default();
        assert!(!s.is_primitive());
        assert!(!s.is_array());
        assert!(!s.is_object());
    }

    // ── Operation helper tests ──────────────────────────────────

    #[test]
    fn operation_json_body_schema() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let post_op = spec.paths["/pets"].post.as_ref().unwrap();
        let body_schema = post_op.json_body_schema();
        assert!(body_schema.is_some());
        assert!(body_schema.unwrap().is_ref());
    }

    #[test]
    fn operation_json_body_schema_none_when_no_body() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let get_op = spec.paths["/pets"].get.as_ref().unwrap();
        assert!(get_op.json_body_schema().is_none());
    }

    #[test]
    fn operation_success_response_200() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let get_op = spec.paths["/pets"].get.as_ref().unwrap();
        let resp_schema = get_op.success_response_schema();
        assert!(resp_schema.is_some());
        assert!(resp_schema.unwrap().is_array());
    }

    #[test]
    fn operation_success_response_201() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let post_op = spec.paths["/pets"].post.as_ref().unwrap();
        let resp_schema = post_op.success_response_schema();
        assert!(resp_schema.is_some());
        assert!(resp_schema.unwrap().is_ref());
    }

    #[test]
    fn operation_success_response_none_when_no_content() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let delete_op = spec.paths["/pets/{petId}"].delete.as_ref().unwrap();
        assert!(delete_op.success_response_schema().is_none());
    }

    // ── all_operations comprehensive ────────────────────────────

    #[test]
    fn all_operations_returns_correct_methods() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let ops = all_operations(&spec);
        let methods: Vec<&str> = ops.iter().map(|(m, _, _)| m.as_str()).collect();
        assert!(methods.contains(&"get"));
        assert!(methods.contains(&"post"));
        assert!(methods.contains(&"delete"));
    }

    #[test]
    fn all_operations_empty_spec() {
        let spec: OpenApiSpec =
            serde_yaml_ng::from_str("info:\n  title: E\n  version: '1'\npaths: {}")
                .unwrap();
        assert!(all_operations(&spec).is_empty());
    }

    // ── Schema helpers on parsed spec ───────────────────────────

    #[test]
    fn parsed_schema_helpers_on_pet() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let pet = &spec.components.as_ref().unwrap().schemas["Pet"];
        assert!(pet.is_object());
        assert!(!pet.is_array());
        assert!(!pet.is_primitive());
        assert!(!pet.is_ref());
        assert!(!pet.is_enum());
    }

    #[test]
    fn parsed_schema_helpers_on_pet_status() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let status = &spec.components.as_ref().unwrap().schemas["PetStatus"];
        assert!(status.is_primitive());
        assert!(status.is_enum());
        assert!(!status.is_object());
    }

    #[test]
    fn parsed_schema_helpers_on_ref_property() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let pet = &spec.components.as_ref().unwrap().schemas["Pet"];
        let status_prop = &pet.properties["status"];
        assert!(status_prop.is_ref());
        assert_eq!(status_prop.ref_name(), Some("PetStatus"));
    }

    // ── YAML ↔ JSON round-trip ────────────────────────────────────

    #[test]
    fn roundtrip_yaml_to_json_to_yaml() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let json = serde_json::to_string_pretty(&spec).unwrap();
        let from_json: OpenApiSpec = serde_json::from_str(&json).unwrap();
        let yaml_again = serde_yaml_ng::to_string(&from_json).unwrap();
        let from_yaml: OpenApiSpec = serde_yaml_ng::from_str(&yaml_again).unwrap();

        assert_eq!(from_yaml.info.title, "Pet Store");
        assert_eq!(from_yaml.info.version, "2.0.0");
        assert_eq!(from_yaml.paths.len(), 2);
        assert_eq!(
            from_yaml.components.as_ref().unwrap().schemas.len(),
            3
        );
    }

    #[test]
    fn roundtrip_preserves_all_operations() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let json = serde_json::to_string(&spec).unwrap();
        let roundtrip: OpenApiSpec = serde_json::from_str(&json).unwrap();
        let orig_ops = all_operations(&spec);
        let rt_ops = all_operations(&roundtrip);
        assert_eq!(orig_ops.len(), rt_ops.len());
        for (orig, rt) in orig_ops.iter().zip(rt_ops.iter()) {
            assert_eq!(orig.0, rt.0);
            assert_eq!(orig.1, rt.1);
            assert_eq!(orig.2.operation_id, rt.2.operation_id);
        }
    }

    #[test]
    fn roundtrip_preserves_schema_fields() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let json = serde_json::to_string(&spec).unwrap();
        let roundtrip: OpenApiSpec = serde_json::from_str(&json).unwrap();

        let orig_pet = &spec.components.as_ref().unwrap().schemas["Pet"];
        let rt_pet = &roundtrip.components.as_ref().unwrap().schemas["Pet"];
        assert_eq!(orig_pet.schema_type, rt_pet.schema_type);
        assert_eq!(orig_pet.required, rt_pet.required);
        assert_eq!(orig_pet.properties.len(), rt_pet.properties.len());
    }

    #[test]
    fn roundtrip_preserves_servers() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let json = serde_json::to_string(&spec).unwrap();
        let roundtrip: OpenApiSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(spec.servers.len(), roundtrip.servers.len());
        assert_eq!(spec.servers[0].url, roundtrip.servers[0].url);
        assert_eq!(spec.servers[0].description, roundtrip.servers[0].description);
    }

    #[test]
    fn roundtrip_preserves_security() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let json = serde_json::to_string(&spec).unwrap();
        let roundtrip: OpenApiSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(spec.security, roundtrip.security);
    }

    // ── Complex schema compositions ──────────────────────────────

    #[test]
    fn parse_deeply_nested_schema() {
        let yaml = r#"
info:
  title: Nested Test
  version: "1.0.0"
paths: {}
components:
  schemas:
    Outer:
      type: object
      properties:
        inner:
          type: object
          properties:
            deep:
              type: array
              items:
                type: string
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let outer = &spec.components.as_ref().unwrap().schemas["Outer"];
        let inner = &outer.properties["inner"];
        assert!(inner.is_object());
        let deep = &inner.properties["deep"];
        assert!(deep.is_array());
        let items = deep.items.as_ref().unwrap();
        assert!(items.is_primitive());
        assert_eq!(items.schema_type.as_deref(), Some("string"));
    }

    #[test]
    fn parse_all_of_with_override_properties() {
        let yaml = r##"
info:
  title: AllOf Override
  version: "1.0.0"
paths: {}
components:
  schemas:
    Base:
      type: object
      properties:
        id:
          type: integer
    Extended:
      allOf:
        - $ref: "#/components/schemas/Base"
        - type: object
          required:
            - name
          properties:
            name:
              type: string
            role:
              type: string
              enum:
                - admin
                - user
"##;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let extended = &spec.components.as_ref().unwrap().schemas["Extended"];
        assert_eq!(extended.all_of.len(), 2);
        let override_part = &extended.all_of[1];
        assert_eq!(override_part.required, vec!["name"]);
        assert!(override_part.properties["role"].is_enum());
    }

    // ── Ref resolution edge cases ────────────────────────────────

    #[test]
    fn resolve_schema_ref_chain() {
        let yaml = r##"
info:
  title: Ref Chain
  version: "1.0.0"
paths: {}
components:
  schemas:
    Alias:
      $ref: "#/components/schemas/Real"
    Real:
      type: string
"##;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let alias = spec.resolve_schema_ref("#/components/schemas/Alias").unwrap();
        assert!(alias.is_ref());
        let real = spec.resolve_schema_ref("#/components/schemas/Real").unwrap();
        assert!(real.is_primitive());
    }

    #[test]
    fn resolve_refs_with_empty_components() {
        let yaml = r#"
info:
  title: Empty Components
  version: "1.0.0"
paths: {}
components:
  schemas: {}
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        assert!(spec.resolve_schema_ref("#/components/schemas/Missing").is_none());
        assert!(spec.resolve_parameter_ref("#/components/parameters/Missing").is_none());
        assert!(spec.resolve_request_body_ref("#/components/requestBodies/Missing").is_none());
        assert!(spec.resolve_response_ref("#/components/responses/Missing").is_none());
    }

    // ── Operation with operation-level security ──────────────────

    #[test]
    fn parse_operation_level_security() {
        let yaml = r#"
info:
  title: OpSecurity
  version: "1.0.0"
paths:
  /secure:
    get:
      operationId: secureEndpoint
      security:
        - apiKey: []
        - oauth2:
            - read
            - write
      responses:
        "200":
          description: OK
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let op = spec.paths["/secure"].get.as_ref().unwrap();
        assert_eq!(op.security.len(), 2);
        assert!(op.security[0].contains_key("apiKey"));
        let oauth_scopes = &op.security[1]["oauth2"];
        assert_eq!(oauth_scopes, &vec!["read".to_string(), "write".to_string()]);
    }

    // ── Multiple content types ───────────────────────────────────

    #[test]
    fn parse_multiple_content_types() {
        let yaml = r#"
info:
  title: MultiContent
  version: "1.0.0"
paths:
  /upload:
    post:
      operationId: upload
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
          multipart/form-data:
            schema:
              type: object
      responses:
        "200":
          description: OK
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let body = spec.paths["/upload"]
            .post
            .as_ref()
            .unwrap()
            .request_body
            .as_ref()
            .unwrap();
        assert_eq!(body.content.len(), 2);
        assert!(body.content.contains_key("application/json"));
        assert!(body.content.contains_key("multipart/form-data"));
    }

    // ── Operation with no operationId ────────────────────────────

    #[test]
    fn parse_operation_without_operation_id() {
        let yaml = r#"
info:
  title: NoOpId
  version: "1.0.0"
paths:
  /items:
    get:
      responses:
        "200":
          description: OK
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let op = spec.paths["/items"].get.as_ref().unwrap();
        assert!(op.operation_id.is_none());
        assert!(op.summary.is_none());
        assert!(op.description.is_none());
        assert!(op.tags.is_empty());
        assert!(op.parameters.is_empty());
        assert!(op.request_body.is_none());
        assert!(op.security.is_empty());
    }

    // ── all_operations with put and patch ────────────────────────

    #[test]
    fn all_operations_includes_put_and_patch() {
        let yaml = r#"
info:
  title: PutPatch
  version: "1.0.0"
paths:
  /items/{id}:
    put:
      operationId: replaceItem
      responses:
        "200":
          description: OK
    patch:
      operationId: updateItem
      responses:
        "200":
          description: OK
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let ops = all_operations(&spec);
        assert_eq!(ops.len(), 2);
        let methods: Vec<&str> = ops.iter().map(|(m, _, _)| m.as_str()).collect();
        assert!(methods.contains(&"put"));
        assert!(methods.contains(&"patch"));
    }

    // ── Schema with multiple numeric constraints ─────────────────

    #[test]
    fn parse_schema_with_all_constraints() {
        let yaml = r#"
info:
  title: Constraints
  version: "1.0.0"
paths: {}
components:
  schemas:
    ConstrainedString:
      type: string
      minLength: 1
      maxLength: 100
      default: "hello"
      title: A constrained string
      description: Must be between 1 and 100 chars
      nullable: true
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let schema = &spec.components.as_ref().unwrap().schemas["ConstrainedString"];
        assert_eq!(schema.min_length, Some(1));
        assert_eq!(schema.max_length, Some(100));
        assert_eq!(
            schema.default.as_ref().and_then(|v| v.as_str()),
            Some("hello")
        );
        assert_eq!(schema.title.as_deref(), Some("A constrained string"));
        assert_eq!(
            schema.description.as_deref(),
            Some("Must be between 1 and 100 chars")
        );
        assert!(schema.nullable);
        assert!(schema.is_primitive());
    }

    // ── ref_name edge cases ──────────────────────────────────────

    #[test]
    fn ref_name_empty_string() {
        assert_eq!(ref_name(""), "");
    }

    #[test]
    fn ref_name_single_slash() {
        assert_eq!(ref_name("/"), "");
    }

    #[test]
    fn ref_name_multiple_trailing_slashes() {
        assert_eq!(ref_name("a/b/c/"), "");
    }

    // ── Parameter with $ref ──────────────────────────────────────

    #[test]
    fn parameter_ref_path_field_deserialized() {
        let json = r##"{
            "name": "limit",
            "in": "query",
            "$ref": "#/components/parameters/LimitParam"
        }"##;
        let param: Parameter = serde_json::from_str(json).unwrap();
        assert_eq!(
            param.ref_path.as_deref(),
            Some("#/components/parameters/LimitParam")
        );
        assert_eq!(param.name, "limit");
    }

    // ── Request body with $ref ───────────────────────────────────

    #[test]
    fn parse_request_body_with_ref() {
        let yaml = r##"
info:
  title: BodyRef
  version: "1.0.0"
paths:
  /items:
    post:
      requestBody:
        $ref: "#/components/requestBodies/ItemBody"
      responses:
        "201":
          description: Created
components:
  requestBodies:
    ItemBody:
      required: true
      content:
        application/json:
          schema:
            type: object
"##;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let body = spec.paths["/items"]
            .post
            .as_ref()
            .unwrap()
            .request_body
            .as_ref()
            .unwrap();
        assert_eq!(
            body.ref_path.as_deref(),
            Some("#/components/requestBodies/ItemBody")
        );
    }

    // ── Response with $ref ───────────────────────────────────────

    #[test]
    fn parse_response_with_ref() {
        let yaml = r##"
info:
  title: RespRef
  version: "1.0.0"
paths:
  /items:
    get:
      responses:
        "404":
          $ref: "#/components/responses/NotFound"
components:
  responses:
    NotFound:
      description: Not found
"##;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let resp = &spec.paths["/items"].get.as_ref().unwrap().responses["404"];
        assert_eq!(
            resp.ref_path.as_deref(),
            Some("#/components/responses/NotFound")
        );
    }

    // ── PartialEq for security requirements ──────────────────────

    #[test]
    fn multiple_servers_parsed() {
        let yaml = r#"
info:
  title: MultiServer
  version: "1.0.0"
paths: {}
servers:
  - url: https://prod.example.com
    description: Production
  - url: https://staging.example.com
    description: Staging
  - url: http://localhost:8080
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        assert_eq!(spec.servers.len(), 3);
        assert_eq!(spec.servers[0].url, "https://prod.example.com");
        assert_eq!(spec.servers[2].url, "http://localhost:8080");
        assert!(spec.servers[2].description.is_none());
    }

    // ── Info description ─────────────────────────────────────────

    #[test]
    fn info_without_description() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(MINIMAL_SPEC_YAML).unwrap();
        assert!(spec.info.description.is_none());
    }

    // ── Schema with additional_properties box ────────────────────

    #[test]
    fn parse_schema_additional_properties_nested() {
        let yaml = r#"
info:
  title: NestedAdditional
  version: "1.0.0"
paths: {}
components:
  schemas:
    Map:
      type: object
      additionalProperties:
        type: array
        items:
          type: integer
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let map = &spec.components.as_ref().unwrap().schemas["Map"];
        let ap = map.additional_properties.as_ref().unwrap();
        assert!(ap.is_array());
        let items = ap.items.as_ref().unwrap();
        assert_eq!(items.schema_type.as_deref(), Some("integer"));
    }

    // ── Operation json_body_schema with non-JSON content ─────────

    #[test]
    fn json_body_schema_none_when_xml_only() {
        let yaml = r#"
info:
  title: XmlBody
  version: "1.0.0"
paths:
  /items:
    post:
      requestBody:
        content:
          application/xml:
            schema:
              type: object
      responses:
        "201":
          description: Created
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let op = spec.paths["/items"].post.as_ref().unwrap();
        assert!(op.json_body_schema().is_none());
    }

    // ── Success response with non-JSON content ───────────────────

    #[test]
    fn success_response_schema_none_when_no_json() {
        let yaml = r#"
info:
  title: NoJsonResp
  version: "1.0.0"
paths:
  /items:
    get:
      responses:
        "200":
          description: OK
          content:
            text/plain:
              schema:
                type: string
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let op = spec.paths["/items"].get.as_ref().unwrap();
        assert!(op.success_response_schema().is_none());
    }

    #[test]
    fn success_response_schema_none_when_only_404() {
        let yaml = r#"
info:
  title: Only404
  version: "1.0.0"
paths:
  /items:
    get:
      responses:
        "404":
          description: Not found
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let op = spec.paths["/items"].get.as_ref().unwrap();
        assert!(op.success_response_schema().is_none());
    }

    // ── PathItem::operations() tests ────────────────────────────

    // ── Display and PartialEq tests ─────────────────────────────

    #[test]
    fn spec_display_format() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let display = spec.to_string();
        assert_eq!(display, "OpenAPI spec: Pet Store v2.0.0 (2 paths, 4 operations)");
    }

    #[test]
    fn spec_display_minimal() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(MINIMAL_SPEC_YAML).unwrap();
        let display = spec.to_string();
        assert_eq!(display, "OpenAPI spec: Test API v1.0.0 (0 paths, 0 operations)");
    }

    #[test]
    fn info_display_format() {
        let info = Info {
            title: "Pet Store".to_string(),
            description: None,
            version: "2.0.0".to_string(),
        };
        assert_eq!(info.to_string(), "Pet Store v2.0.0");
    }

    #[test]
    fn info_partial_eq() {
        let a = Info {
            title: "API".to_string(),
            description: None,
            version: "1.0".to_string(),
        };
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn server_partial_eq() {
        let a = Server {
            url: "https://example.com".to_string(),
            description: Some("prod".to_string()),
        };
        let b = a.clone();
        assert_eq!(a, b);
    }

    // ── Default trait tests ──────────────────────────────────────

    #[test]
    fn path_item_default_has_no_operations() {
        let item = PathItem::default();
        assert!(item.get.is_none());
        assert!(item.post.is_none());
        assert!(item.put.is_none());
        assert!(item.delete.is_none());
        assert!(item.patch.is_none());
        assert!(item.parameters.is_empty());
        assert_eq!(item.operations().count(), 0);
    }

    #[test]
    fn operation_default_has_empty_fields() {
        let op = Operation::default();
        assert!(op.operation_id.is_none());
        assert!(op.summary.is_none());
        assert!(op.description.is_none());
        assert!(op.parameters.is_empty());
        assert!(op.request_body.is_none());
        assert!(op.responses.is_empty());
        assert!(op.security.is_empty());
        assert!(op.tags.is_empty());
    }

    #[test]
    fn request_body_default_is_empty() {
        let body = RequestBody::default();
        assert!(!body.required);
        assert!(body.content.is_empty());
        assert!(body.description.is_none());
        assert!(body.ref_path.is_none());
    }

    #[test]
    fn media_type_default_has_no_schema() {
        let mt = MediaType::default();
        assert!(mt.schema.is_none());
    }

    #[test]
    fn response_default_is_empty() {
        let resp = Response::default();
        assert!(resp.description.is_none());
        assert!(resp.content.is_none());
        assert!(resp.ref_path.is_none());
    }

    #[test]
    fn spec_all_operations_iterator() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let ops: Vec<_> = spec.all_operations().collect();
        assert_eq!(ops.len(), 4);
        let methods_and_paths: Vec<(&str, &str)> =
            ops.iter().map(|(m, p, _)| (m.as_str(), p.as_str())).collect();
        assert!(methods_and_paths.contains(&("get", "/pets")));
        assert!(methods_and_paths.contains(&("post", "/pets")));
        assert!(methods_and_paths.contains(&("get", "/pets/{petId}")));
        assert!(methods_and_paths.contains(&("delete", "/pets/{petId}")));
    }

    #[test]
    fn spec_all_operations_empty() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(MINIMAL_SPEC_YAML).unwrap();
        assert_eq!(spec.all_operations().count(), 0);
    }

    #[test]
    fn spec_schema_count() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        assert_eq!(spec.schema_count(), 3);
    }

    #[test]
    fn spec_schema_count_no_components() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(MINIMAL_SPEC_YAML).unwrap();
        assert_eq!(spec.schema_count(), 0);
    }

    #[test]
    fn spec_operation_count() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        assert_eq!(spec.operation_count(), 4);
    }

    #[test]
    fn spec_operation_count_empty() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(MINIMAL_SPEC_YAML).unwrap();
        assert_eq!(spec.operation_count(), 0);
    }

    #[test]
    fn path_item_operations_returns_defined_methods() {
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let pets = &spec.paths["/pets"];
        let methods: Vec<&str> = pets.operations().map(|(m, _)| m).collect();
        assert_eq!(methods, vec!["get", "post"]);
    }

    #[test]
    fn path_item_operations_empty_when_no_methods() {
        let item = PathItem {
            get: None,
            post: None,
            put: None,
            delete: None,
            patch: None,
            parameters: vec![],
        };
        assert_eq!(item.operations().count(), 0);
    }

    #[test]
    fn path_item_operations_all_five_methods() {
        let yaml = r#"
info:
  title: AllMethods
  version: "1.0.0"
paths:
  /resource:
    get:
      responses:
        "200":
          description: OK
    post:
      responses:
        "201":
          description: Created
    put:
      responses:
        "200":
          description: OK
    delete:
      responses:
        "204":
          description: Deleted
    patch:
      responses:
        "200":
          description: OK
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let item = &spec.paths["/resource"];
        let methods: Vec<&str> = item.operations().map(|(m, _)| m).collect();
        assert_eq!(methods, vec!["get", "post", "put", "delete", "patch"]);
    }
}
