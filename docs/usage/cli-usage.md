---
title: CLI Usage
description: Command-line tools for formatting, evaluating, and converting RESL files
---

# ⚡ CLI Usage

Command-line tools for formatting, evaluating, and converting RESL files.le: CLI Usage
description: Command-line tools for formatting, evaluation, and conversion

---

# ⚡ CLI Usage

Command-line tools for formatting, evaluating, and converting RESL files.

## 📦 Installation

### 📦 Package Managers

#### Cargo

```bash
cargo install resl-cli
```

_Coming soon!_

### 📀 Binary Releases

_Coming soon! Pre-built binaries will be available from GitHub Releases._

## 🚀 Basic Usage

```bash
resl [OPTIONS] <COMMAND>
```

### ⚙️ Global Options

- `-i, --input <FILE>` - Input file to read from (defaults to stdin)
- `-o, --output <FILE>` - Output file to write to (defaults to stdout)
- `-p, --pretty` - Enable pretty-formatted output

## 🛠️ Commands

### 🎨 `format` - Format RESL Code

Format and pretty-print RESL expressions:

```bash
# Format from file
resl format -i config.resl -o formatted.resl --pretty

# Format from stdin
echo '{ x = 10; y = 20; x + y }' | resl format --pretty

# Format to stdout
resl format -i messy.resl --pretty
```

**Example:**

Input:

```resl
{x=10;y=20;result=x+y;["sum":result,"values":[x,y]]}
```

Output (with `--pretty`):

```resl
{
    x = 10;
    y = 20;
    result = x + y;
    [
        "sum": result,
        "values": [x, y]
    ]
}
```

### ⚡ `evaluate` - Parse and Evaluate

Parse and evaluate RESL expressions to see their final output:

```bash
# Evaluate a file
resl evaluate -i config.resl --pretty

# Evaluate from stdin
echo '5 + 3' | resl evaluate

# Evaluate and save result
resl evaluate -i input.resl -o output.json --pretty
```

**Examples:**

```bash
# Simple arithmetic
$ echo '(10 + 5) * 2' | resl evaluate
30

# Complex configuration
$ resl evaluate -i deployment.resl --pretty
{
    "services": [
        {
            "name": "auth",
            "image": "app/auth:v1.2.0",
            "port": 8080
        },
        {
            "name": "api",
            "image": "app/api:v1.2.0",
            "port": 8081
        }
    ],
    "environment": "production"
}
```

### 📤 `export` - Convert to Other Formats

Export RESL to JSON or TOML:

```bash
# Export to JSON
resl export --to json -i config.resl -o config.json --pretty

# Export to TOML
resl export --to toml -i config.resl -o config.toml --pretty

# Export to stdout
resl export --to json -i config.resl --pretty
```

**Example:**

RESL input (`config.resl`):

```resl
{
    app_name = "MyApp";
    version = "1.0.0";
    servers = [
        ["host": "web1.example.com", "port": 8080],
        ["host": "web2.example.com", "port": 8080]
    ];

    ["application": ["name": app_name, "version": version], "servers": servers]
}
```

JSON output:

```bash
$ resl export --to json -i config.resl --pretty
{
  "application": {
    "name": "MyApp",
    "version": "1.0.0"
  },
  "servers": [
    {
      "host": "web1.example.com",
      "port": 8080
    },
    {
      "host": "web2.example.com",
      "port": 8080
    }
  ]
}
```

TOML output:

```bash
$ resl export --to toml -i config.resl --pretty
[application]
name = "MyApp"
version = "1.0.0"

[[servers]]
host = "web1.example.com"
port = 8080

[[servers]]
host = "web2.example.com"
port = 8080
```

### 📥 `import` - Convert from Other Formats

Import JSON or TOML files and convert them to RESL:

```bash
# Import from JSON
resl import --from json -i config.json -o config.resl --pretty

# Import from TOML
resl import --from toml -i config.toml -o config.resl --pretty

# Import to stdout
resl import --from json -i data.json --pretty
```

**Example:**

JSON input (`data.json`):

```json
{
  "name": "Alice",
  "age": 30,
  "hobbies": ["reading", "hiking"],
  "address": {
    "street": "123 Main St",
    "city": "Boston"
  }
}
```

RESL output:

```bash
$ resl import --from json -i data.json --pretty
[
    "name": "Alice",
    "age": 30,
    "hobbies": [
        "reading",
        "hiking"
    ],
    "address": [
        "street": "123 Main St",
        "city": "Boston"
    ]
]
```

## ❗ Error Messages

RESL provides beautifully formatted error messages that show exactly where syntax errors occur. The error display includes the exact line and column where the problem was found, with helpful context:

<img src="/error.png" alt="Error" style="width: 400px;">

The error formatting makes it easy to spot issues by showing:

- The exact line and column number where the error occurred
- The problematic code with clear visual indicators
- A helpful description of what was expected vs. what was found

## 🚀 Performance Tips

1. **Use files instead of stdin/stdout** for large configurations to avoid memory overhead
2. **Skip `--pretty` formatting** for production builds to reduce output size
3. **Validate syntax first** with `format` before running expensive `evaluate` operations
4. **Cache evaluation results** for configurations that don't change frequently

The RESL CLI is designed to integrate smoothly into existing build pipelines and development workflows, providing both interactive use and automation-friendly batch processing capabilities.
