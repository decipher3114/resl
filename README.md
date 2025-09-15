# RESL

**RESL** = **R**untime **E**valuated **S**erialization **L**anguage

A modern configuration and serialization language with variables, expressions, and dynamic runtime evaluation.

```resl
{
    name = "My Application";
    version = "1.0.0";

    database = [
        "host": "localhost",
        "port": 5432,
        "ssl": true
    ];

    features = ["auth", "logging", "metrics"];

    ["name": name, "version": version, "database": database, "features": features]
}
```

## Documentation

- **[📚 Complete Documentation](https://decipher3114.github.io/resl/)** - Language guide, syntax, and examples
- **[🔧 Contributing Guide](https://github.com/decipher3114/resl/blob/main/CONTRIBUTING.md)** - How to contribute to RESL

## License

MIT OR Apache-2.0
