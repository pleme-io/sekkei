// SpecVisitor trait and walk_spec for walking OpenAPI specs.

use crate::types::{Operation, Parameter, PathItem, Schema};
use crate::OpenApiSpec;

/// Visitor trait for walking `OpenAPI` specs.
/// Implement specific methods to process only what you need.
/// Default implementations are no-ops, enabling selective visiting.
pub trait SpecVisitor {
    /// Called for each path item in the spec.
    fn visit_path(&mut self, _path: &str, _item: &PathItem) {}

    /// Called for each operation (GET, POST, PUT, DELETE, PATCH) in a path.
    fn visit_operation(&mut self, _method: &str, _path: &str, _op: &Operation) {}

    /// Called for each named schema in `components.schemas`.
    fn visit_schema(&mut self, _name: &str, _schema: &Schema) {}

    /// Called for each parameter (both path-level and operation-level).
    fn visit_parameter(&mut self, _param: &Parameter) {}
}

/// Walk a spec, calling visitor methods for each element.
pub fn walk_spec(spec: &OpenApiSpec, visitor: &mut dyn SpecVisitor) {
    for (path, item) in &spec.paths {
        visitor.visit_path(path, item);
        for (method, op) in path_operations(item) {
            visitor.visit_operation(method, path, op);
            for param in &op.parameters {
                visitor.visit_parameter(param);
            }
        }
        for param in &item.parameters {
            visitor.visit_parameter(param);
        }
    }
    if let Some(components) = &spec.components {
        for (name, schema) in &components.schemas {
            visitor.visit_schema(name, schema);
        }
    }
}

/// Extract all operations from a path item as `(method, &Operation)` pairs.
fn path_operations(item: &PathItem) -> Vec<(&'static str, &Operation)> {
    let mut ops = Vec::new();
    if let Some(op) = &item.get {
        ops.push(("get", op));
    }
    if let Some(op) = &item.post {
        ops.push(("post", op));
    }
    if let Some(op) = &item.put {
        ops.push(("put", op));
    }
    if let Some(op) = &item.delete {
        ops.push(("delete", op));
    }
    if let Some(op) = &item.patch {
        ops.push(("patch", op));
    }
    ops
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A richer spec with components, paths, servers, and security schemes.
    const FULL_SPEC_YAML: &str = r##"
info:
  title: Pet Store
  description: A sample pet store API
  version: "2.0.0"
servers:
  - url: https://api.petstore.example.com/v2
    description: Production server
security:
  - bearerAuth: []
paths:
  /pets:
    get:
      operationId: listPets
      summary: List all pets
      parameters:
        - name: limit
          in: query
          required: false
          schema:
            type: integer
            format: int64
      responses:
        "200":
          description: A list of pets
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: "#/components/schemas/Pet"
    post:
      operationId: createPet
      summary: Create a pet
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/CreatePetRequest"
      responses:
        "201":
          description: Pet created
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
  /pets/{petId}:
    parameters:
      - name: petId
        in: path
        required: true
        schema:
          type: string
    get:
      operationId: getPet
      summary: Get a pet by ID
      responses:
        "200":
          description: A pet
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
        "404":
          description: Pet not found
    delete:
      operationId: deletePet
      summary: Delete a pet
      responses:
        "204":
          description: Pet deleted
components:
  schemas:
    Pet:
      type: object
      required:
        - id
        - name
      properties:
        id:
          type: integer
          format: int64
        name:
          type: string
        tag:
          type: string
        status:
          $ref: "#/components/schemas/PetStatus"
    PetStatus:
      type: string
      enum:
        - available
        - pending
        - sold
    CreatePetRequest:
      type: object
      required:
        - name
      properties:
        name:
          type: string
          description: The pet's name
        tag:
          type: string
          description: Optional tag
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
  parameters:
    LimitParam:
      name: limit
      in: query
      required: false
      schema:
        type: integer
  requestBodies:
    PetBody:
      required: true
      content:
        application/json:
          schema:
            $ref: "#/components/schemas/CreatePetRequest"
  responses:
    NotFound:
      description: The requested resource was not found
"##;

    #[test]
    fn visitor_counts_operations() {
        struct Counter {
            ops: usize,
            schemas: usize,
        }
        impl SpecVisitor for Counter {
            fn visit_operation(&mut self, _: &str, _: &str, _: &Operation) {
                self.ops += 1;
            }
            fn visit_schema(&mut self, _: &str, _: &Schema) {
                self.schemas += 1;
            }
        }
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut counter = Counter {
            ops: 0,
            schemas: 0,
        };
        walk_spec(&spec, &mut counter);
        assert_eq!(counter.ops, 4); // listPets, createPet, getPet, deletePet
        assert_eq!(counter.schemas, 3); // Pet, PetStatus, CreatePetRequest
    }

    #[test]
    fn visitor_counts_paths() {
        struct PathCounter {
            paths: usize,
        }
        impl SpecVisitor for PathCounter {
            fn visit_path(&mut self, _: &str, _: &PathItem) {
                self.paths += 1;
            }
        }
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut counter = PathCounter { paths: 0 };
        walk_spec(&spec, &mut counter);
        assert_eq!(counter.paths, 2); // /pets, /pets/{petId}
    }

    #[test]
    fn visitor_counts_parameters() {
        struct ParamCounter {
            params: usize,
        }
        impl SpecVisitor for ParamCounter {
            fn visit_parameter(&mut self, _: &Parameter) {
                self.params += 1;
            }
        }
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut counter = ParamCounter { params: 0 };
        walk_spec(&spec, &mut counter);
        // 1 operation-level param (limit on GET /pets) + 1 path-level param (petId on /pets/{petId})
        assert_eq!(counter.params, 2);
    }

    #[test]
    fn visitor_collects_operation_ids() {
        struct IdCollector {
            ids: Vec<String>,
        }
        impl SpecVisitor for IdCollector {
            fn visit_operation(&mut self, _: &str, _: &str, op: &Operation) {
                if let Some(id) = &op.operation_id {
                    self.ids.push(id.clone());
                }
            }
        }
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut collector = IdCollector { ids: Vec::new() };
        walk_spec(&spec, &mut collector);
        assert!(collector.ids.contains(&"listPets".to_string()));
        assert!(collector.ids.contains(&"createPet".to_string()));
        assert!(collector.ids.contains(&"getPet".to_string()));
        assert!(collector.ids.contains(&"deletePet".to_string()));
    }

    #[test]
    fn visitor_collects_methods() {
        struct MethodCollector {
            methods: Vec<String>,
        }
        impl SpecVisitor for MethodCollector {
            fn visit_operation(&mut self, method: &str, _: &str, _: &Operation) {
                self.methods.push(method.to_string());
            }
        }
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut collector = MethodCollector {
            methods: Vec::new(),
        };
        walk_spec(&spec, &mut collector);
        assert!(collector.methods.contains(&"get".to_string()));
        assert!(collector.methods.contains(&"post".to_string()));
        assert!(collector.methods.contains(&"delete".to_string()));
    }

    #[test]
    fn visitor_empty_spec() {
        struct Counter {
            total: usize,
        }
        impl SpecVisitor for Counter {
            fn visit_path(&mut self, _: &str, _: &PathItem) {
                self.total += 1;
            }
            fn visit_operation(&mut self, _: &str, _: &str, _: &Operation) {
                self.total += 1;
            }
            fn visit_schema(&mut self, _: &str, _: &Schema) {
                self.total += 1;
            }
            fn visit_parameter(&mut self, _: &Parameter) {
                self.total += 1;
            }
        }
        let spec: OpenApiSpec =
            serde_yaml_ng::from_str("info:\n  title: E\n  version: '1'\npaths: {}").unwrap();
        let mut counter = Counter { total: 0 };
        walk_spec(&spec, &mut counter);
        assert_eq!(counter.total, 0);
    }

    #[test]
    fn visitor_noop_default_implementations() {
        // Verifies that the default no-op implementations don't panic.
        struct NoopVisitor;
        impl SpecVisitor for NoopVisitor {}
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut visitor = NoopVisitor;
        walk_spec(&spec, &mut visitor);
        // If we get here without panic, defaults are safe.
    }

    #[test]
    fn visitor_collects_schema_names() {
        struct SchemaCollector {
            names: Vec<String>,
        }
        impl SpecVisitor for SchemaCollector {
            fn visit_schema(&mut self, name: &str, _: &Schema) {
                self.names.push(name.to_string());
            }
        }
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut collector = SchemaCollector { names: Vec::new() };
        walk_spec(&spec, &mut collector);
        assert!(collector.names.contains(&"Pet".to_string()));
        assert!(collector.names.contains(&"PetStatus".to_string()));
        assert!(collector.names.contains(&"CreatePetRequest".to_string()));
    }
}
