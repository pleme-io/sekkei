//! OpenAPI 3.1 / JSON-Schema-2020-12 conformance corpus.
//!
//! Every case here is a construct that a 3.1 document can contain and a 3.0-only
//! parser gets wrong. The corpus is **inlined**, not read from a file on disk:
//! a gate whose fixture can go missing degrades to passing vacuously, which is
//! the exact failure mode already on record in this fleet (`iac-forge`'s CI ran
//! zero of 678 tests for 54 days because a `nix flake check` found no `checks`
//! output). An inlined corpus cannot go missing.
//!
//! Measured against Discord's real 3.1 spec (1.18 MB, 150 paths, 242 operations,
//! 539 schemas) on 2026-08-18: every construct below appears there, and the
//! counts each case stands in for are noted per test.

use sekkei::{OpenApiSpec, load_spec_from_str};

fn parse(body: &str) -> OpenApiSpec {
    let doc = format!(
        r#"{{"openapi":"3.1.0","info":{{"title":"corpus","version":"1"}},
            "paths":{{}},"components":{{"schemas":{body}}}}}"#
    );
    load_spec_from_str(&doc, std::path::Path::new("corpus.json"))
        .unwrap_or_else(|e| panic!("corpus must parse: {e}"))
}

fn schema(body: &str, name: &str) -> sekkei::Schema {
    parse(body)
        .components
        .unwrap()
        .schemas
        .remove(name)
        .unwrap()
}

/// 3.1 spells nullability by putting `"null"` in a `type` array. A parser whose
/// `type` is a bare string fails hard here — this is the single construct that
/// blocked Discord's spec at 911 sites.
#[test]
fn type_array_normalizes_to_base_plus_nullable() {
    let s = schema(r#"{"A":{"type":["string","null"]}}"#, "A");
    assert_eq!(s.schema_type.as_deref(), Some("string"), "base type");
    assert!(s.nullable, "null in the type array means nullable");
    assert_eq!(s.type_union, ["string", "null"], "verbatim list is kept");
    assert!(!s.is_multi_type(), "one non-null type is not a multi-type");
}

/// 3.0's spelling of the same idea must land in the same field, so a consumer
/// reads one representation regardless of which dialect wrote the document.
#[test]
fn nullable_true_and_type_array_agree() {
    let a = schema(r#"{"A":{"type":"string","nullable":true}}"#, "A");
    let b = schema(r#"{"A":{"type":["string","null"]}}"#, "A");
    assert_eq!(
        (a.schema_type.as_deref(), a.nullable),
        (Some("string"), true)
    );
    assert_eq!(
        (b.schema_type.as_deref(), b.nullable),
        (Some("string"), true)
    );
}

/// Several non-null types cannot be represented by one base type. The parser
/// must refuse to pick one rather than hand a consumer a silent guess.
#[test]
fn multi_type_reports_none_rather_than_guessing() {
    let s = schema(r#"{"A":{"type":["string","integer"]}}"#, "A");
    assert!(s.schema_type.is_none(), "no base type may be invented");
    assert!(s.is_multi_type());
    assert_eq!(s.type_union, ["string", "integer"]);
    assert!(!s.nullable);
}

/// `const` is how 3.1 pins a single value, and it is what makes a `oneOf` of
/// consts an enum and a shared const-valued property a discriminator. Discord
/// uses it 659 times; dropping it silently erases both signals.
#[test]
fn const_is_preserved() {
    let s = schema(r#"{"A":{"oneOf":[{"const":0},{"const":1}]}}"#, "A");
    assert_eq!(s.one_of.len(), 2);
    assert!(s.one_of.iter().all(sekkei::Schema::is_const));
    assert_eq!(s.one_of[1].const_value, Some(serde_json::json!(1)));
}

/// A vendor extension can change what a construct *means*: Discord marks its
/// `anyOf` sites `x-discord-union: oneOf` to say they are exclusive. Dropping
/// the extension yields a confidently wrong reading, so it must survive.
#[test]
fn vendor_extension_survives() {
    let s = schema(
        r#"{"A":{"anyOf":[{"type":"string"},{"type":"integer"}],"x-discord-union":"oneOf"}}"#,
        "A",
    );
    assert_eq!(
        s.extension("x-discord-union").and_then(|v| v.as_str()),
        Some("oneOf")
    );
    assert_eq!(s.any_of.len(), 2);
}

/// The drift surface. `unmodelled_keywords` reports spec keywords with no field
/// yet — so an upstream adopting a keyword we do not model turns a gate red and
/// names it, instead of the keyword being ignored. Vendor `x-*` keys are
/// deliberately excluded: carrying those unmodelled is the intent.
#[test]
fn unmodelled_keywords_reports_gaps_but_not_vendor_extensions() {
    let s = schema(
        r#"{"A":{"type":"object","x-vendor":1,"unevaluatedProperties":false}}"#,
        "A",
    );
    let gaps: Vec<&str> = s.unmodelled_keywords().collect();
    assert_eq!(gaps, ["unevaluatedProperties"], "a real gap is reported");
    assert!(
        s.extension("x-vendor").is_some(),
        "vendor key still readable"
    );
}

/// A modelled keyword must NOT show up as drift — otherwise the gate cries wolf
/// and gets muted. This is the other half of the test above.
#[test]
fn modelled_keywords_are_not_reported_as_drift() {
    let s = schema(
        r#"{"A":{"type":"array","items":{"type":"string"},"maxItems":5,"minItems":1,
                 "uniqueItems":true,"maxProperties":3,"minProperties":1,
                 "pattern":"^a$","contentEncoding":"base64","const":"x",
                 "discriminator":{"propertyName":"kind"}}}"#,
        "A",
    );
    assert_eq!(s.unmodelled_keywords().count(), 0, "no false drift");
    assert_eq!(s.max_items, Some(5));
    assert_eq!(s.min_items, Some(1));
    assert!(s.unique_items);
    assert_eq!(s.max_properties, Some(3));
    assert_eq!(s.pattern.as_deref(), Some("^a$"));
    assert_eq!(s.content_encoding.as_deref(), Some("base64"));
    assert_eq!(
        s.discriminator.as_ref().map(|d| d.property_name.as_str()),
        Some("kind")
    );
}

/// 2020-12 allows a bare boolean wherever a schema is expected.
/// `additionalProperties: false` is the common spelling and it hard-failed a
/// struct-only parser. It must not then read as "a map with an empty value
/// type" — `false` *forbids* extra properties, so a map-detector keying off
/// `additional_properties.is_some()` would invert the meaning.
#[test]
fn boolean_schema_parses_and_does_not_read_as_a_map() {
    let s = schema(
        r#"{"A":{"type":"object","properties":{"x":{"type":"string"}},
                 "additionalProperties":false}}"#,
        "A",
    );
    assert_eq!(s.additional_properties_allowed, Some(false));
    assert!(
        s.additional_properties.is_none(),
        "a permission is not a value schema"
    );

    let m = schema(
        r#"{"A":{"type":"object","additionalProperties":{"type":"integer"}}}"#,
        "A",
    );
    assert_eq!(m.additional_properties_allowed, None);
    assert_eq!(
        m.additional_properties
            .as_ref()
            .unwrap()
            .schema_type
            .as_deref(),
        Some("integer"),
        "a real map keeps its value schema"
    );
}

/// Which dialect a document declares is the one fact a reader needs to
/// interpret `type` and nullability. It used to be dropped entirely.
#[test]
fn openapi_version_is_recorded() {
    assert_eq!(parse("{}").openapi.as_deref(), Some("3.1.0"));
}

/// An OAuth2 scheme's flows and scopes are what an SDK needs to emit a typed
/// scope surface. They were dropped, which made every scope invisible.
#[test]
fn oauth2_flows_and_scopes_are_captured() {
    let doc = r#"{"openapi":"3.1.0","info":{"title":"t","version":"1"},"paths":{},
      "components":{"securitySchemes":{"OAuth2":{"type":"oauth2","flows":{
        "authorizationCode":{"authorizationUrl":"https://a","tokenUrl":"https://t",
          "scopes":{"identify":"d1","guilds":"d2"}},
        "clientCredentials":{"tokenUrl":"https://t","scopes":{"bot":"d3"}}}}}}}"#;
    let spec = load_spec_from_str(doc, std::path::Path::new("c.json")).unwrap();
    let flows = spec.components.unwrap().security_schemes["OAuth2"]
        .flows
        .clone()
        .expect("flows captured");
    assert_eq!(
        flows.declared().map(|(n, _)| n).collect::<Vec<_>>(),
        ["clientCredentials", "authorizationCode"]
    );
    // Scopes are unioned across flows: an SDK wants the vocabulary once.
    assert_eq!(
        flows.all_scopes().into_iter().collect::<Vec<_>>(),
        ["bot", "guilds", "identify"]
    );
}

/// Round-trip fidelity: re-serializing must reproduce the dialect that was
/// written, not silently rewrite a 3.1 document into 3.0 spelling (or claim
/// nullability twice by emitting both spellings at once).
#[test]
fn round_trip_preserves_each_dialect() {
    for src in [
        r#"{"type":["string","null"]}"#,
        r#"{"type":"string","nullable":true}"#,
        r#"{"type":"object","additionalProperties":false}"#,
        r#"{"const":7}"#,
    ] {
        let s: sekkei::Schema = serde_json::from_str(src).unwrap();
        let back = serde_json::to_value(&s).unwrap();
        let want: serde_json::Value = serde_json::from_str(src).unwrap();
        assert_eq!(back, want, "round-trip must be byte-faithful for {src}");
    }
}
