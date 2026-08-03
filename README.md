# sekkei (設計)

Canonical **OpenAPI 3.0** serde types, multi-format loading, and `$ref` resolution.

The typed front door of the pleme-io code-generation pipeline: `sekkei` owns
the OpenAPI document model that `takumi` lowers into a typed IR, which the
`*-forge` generators then render.

```
sekkei  ->  takumi  ->  openapi-forge / iac-forge  ->  emitted SDKs + IaC
(types)     (typed IR)   (renderers)
```

## Usage

```toml
[dependencies]
sekkei = "0.1"
```

Loads JSON and YAML, and resolves `$ref` so consumers see a fully-linked
document rather than a graph of pointers.

## License

MIT — see [LICENSE](./LICENSE).
