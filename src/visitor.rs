//! `SpecVisitor` trait and `walk_spec` for walking `OpenAPI` specs.

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
        for (method, op) in item.operations() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fixtures::FULL_SPEC_YAML;

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

    #[test]
    fn visitor_receives_correct_path_strings() {
        struct PathCollector {
            paths: Vec<String>,
        }
        impl SpecVisitor for PathCollector {
            fn visit_path(&mut self, path: &str, _: &PathItem) {
                self.paths.push(path.to_string());
            }
        }
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut collector = PathCollector { paths: Vec::new() };
        walk_spec(&spec, &mut collector);
        assert!(collector.paths.contains(&"/pets".to_string()));
        assert!(collector.paths.contains(&"/pets/{petId}".to_string()));
    }

    #[test]
    fn visitor_receives_method_and_path_for_operations() {
        struct OpCollector {
            entries: Vec<(String, String)>,
        }
        impl SpecVisitor for OpCollector {
            fn visit_operation(&mut self, method: &str, path: &str, _: &Operation) {
                self.entries.push((method.to_string(), path.to_string()));
            }
        }
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut collector = OpCollector {
            entries: Vec::new(),
        };
        walk_spec(&spec, &mut collector);
        assert!(collector.entries.contains(&("get".to_string(), "/pets".to_string())));
        assert!(collector.entries.contains(&("post".to_string(), "/pets".to_string())));
        assert!(
            collector
                .entries
                .contains(&("get".to_string(), "/pets/{petId}".to_string()))
        );
        assert!(
            collector
                .entries
                .contains(&("delete".to_string(), "/pets/{petId}".to_string()))
        );
    }

    #[test]
    fn visitor_receives_parameter_details() {
        struct ParamCollector {
            names: Vec<String>,
            locations: Vec<String>,
        }
        impl SpecVisitor for ParamCollector {
            fn visit_parameter(&mut self, param: &Parameter) {
                self.names.push(param.name.clone());
                self.locations.push(param.location.clone());
            }
        }
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut collector = ParamCollector {
            names: Vec::new(),
            locations: Vec::new(),
        };
        walk_spec(&spec, &mut collector);
        assert!(collector.names.contains(&"limit".to_string()));
        assert!(collector.names.contains(&"petId".to_string()));
        assert!(collector.locations.contains(&"query".to_string()));
        assert!(collector.locations.contains(&"path".to_string()));
    }

    #[test]
    fn visitor_schema_receives_type_info() {
        struct SchemaTypeCollector {
            types: Vec<(String, Option<String>)>,
        }
        impl SpecVisitor for SchemaTypeCollector {
            fn visit_schema(&mut self, name: &str, schema: &Schema) {
                self.types
                    .push((name.to_string(), schema.schema_type.clone()));
            }
        }
        let spec: OpenApiSpec = serde_yaml_ng::from_str(FULL_SPEC_YAML).unwrap();
        let mut collector = SchemaTypeCollector { types: Vec::new() };
        walk_spec(&spec, &mut collector);
        assert!(collector.types.contains(&("Pet".to_string(), Some("object".to_string()))));
        assert!(
            collector
                .types
                .contains(&("PetStatus".to_string(), Some("string".to_string())))
        );
    }

    #[test]
    fn visitor_spec_without_components() {
        struct SchemaCounter {
            count: usize,
        }
        impl SpecVisitor for SchemaCounter {
            fn visit_schema(&mut self, _: &str, _: &Schema) {
                self.count += 1;
            }
        }
        let yaml = "info:\n  title: NoComp\n  version: '1'\npaths:\n  /test:\n    get:\n      responses:\n        '200':\n          description: OK";
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let mut counter = SchemaCounter { count: 0 };
        walk_spec(&spec, &mut counter);
        assert_eq!(counter.count, 0);
    }

    #[test]
    fn visitor_with_all_http_methods() {
        struct MethodCollector {
            methods: Vec<String>,
        }
        impl SpecVisitor for MethodCollector {
            fn visit_operation(&mut self, method: &str, _: &str, _: &Operation) {
                self.methods.push(method.to_string());
            }
        }
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
        let mut collector = MethodCollector {
            methods: Vec::new(),
        };
        walk_spec(&spec, &mut collector);
        assert_eq!(collector.methods.len(), 5);
        assert!(collector.methods.contains(&"get".to_string()));
        assert!(collector.methods.contains(&"post".to_string()));
        assert!(collector.methods.contains(&"put".to_string()));
        assert!(collector.methods.contains(&"delete".to_string()));
        assert!(collector.methods.contains(&"patch".to_string()));
    }

    #[test]
    fn visitor_call_order_path_before_operations() {
        #[derive(Debug, PartialEq)]
        enum Event {
            Path(String),
            Op(String, String),
        }
        struct OrderTracker {
            events: Vec<Event>,
        }
        impl SpecVisitor for OrderTracker {
            fn visit_path(&mut self, path: &str, _: &PathItem) {
                self.events.push(Event::Path(path.to_string()));
            }
            fn visit_operation(&mut self, method: &str, path: &str, _: &Operation) {
                self.events
                    .push(Event::Op(method.to_string(), path.to_string()));
            }
        }
        let yaml = r#"
info:
  title: Order
  version: "1.0.0"
paths:
  /a:
    get:
      responses:
        "200":
          description: OK
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let mut tracker = OrderTracker {
            events: Vec::new(),
        };
        walk_spec(&spec, &mut tracker);
        assert_eq!(tracker.events[0], Event::Path("/a".to_string()));
        assert_eq!(
            tracker.events[1],
            Event::Op("get".to_string(), "/a".to_string())
        );
    }

    #[test]
    fn visitor_path_level_and_operation_level_params() {
        struct DetailedParamCollector {
            params: Vec<(String, String)>,
        }
        impl SpecVisitor for DetailedParamCollector {
            fn visit_parameter(&mut self, param: &Parameter) {
                self.params
                    .push((param.name.clone(), param.location.clone()));
            }
        }
        let yaml = r#"
info:
  title: MixedParams
  version: "1.0.0"
paths:
  /items/{id}:
    parameters:
      - name: id
        in: path
        required: true
    get:
      parameters:
        - name: filter
          in: query
      responses:
        "200":
          description: OK
"#;
        let spec: OpenApiSpec = serde_yaml_ng::from_str(yaml).unwrap();
        let mut collector = DetailedParamCollector {
            params: Vec::new(),
        };
        walk_spec(&spec, &mut collector);
        assert_eq!(collector.params.len(), 2);
        assert!(collector.params.contains(&("filter".to_string(), "query".to_string())));
        assert!(collector.params.contains(&("id".to_string(), "path".to_string())));
    }
}
