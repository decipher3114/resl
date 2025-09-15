---
title: Language Bindings
description: Use RESL from Rust, C, and other languages
---

# 🔗 Language Bindings

Integrate RESL evaluation into applications written in different programming languages.

## 🦀 Rust (Native)

RESL is written in Rust and provides the most complete and performant API through the native Rust library.

### 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
resl = "0.1"
```

### 🚀 Usage

```rust
use resl::{evaluate, format, Value, ParseError};

fn main() -> Result<(), ParseError> {
    // Simple evaluation
    let result = evaluate("5 + 3")?;
    println!("{}", result); // 8

    // Complex configuration
    let config = r#"
        {
            port = 8080;
            host = "localhost";
            database = ["host": host, "port": 5432];
            ["server": ["host": host, "port": port], "database": database]
        }
    "#;

    let result = evaluate(config)?;

    // Work with the result
    match result {
        Value::Map(map) => {
            if let Some(Value::Map(server)) = map.get("server") {
                if let Some(Value::String(host)) = server.get("host") {
                    println!("Server host: {}", host);
                }
            }
        },
        _ => println!("Unexpected result type"),
    }

    // Format RESL code
    let formatted = format(r#"{x=1;y=2;x+y}"#, true)?;
    println!("{}", formatted);

    // Error handling - undefined variables become null, not errors
    let result = evaluate("{ x = undefined_var; x }").unwrap();
    // result is Value::Null

    Ok(())
}
```

### ⚠️ Error Handling

Rust uses `Result<Value, ParseError>` for error handling. RESL evaluation is infallible - there are only parse errors, no runtime errors (undefined variables become `null`):

```rust
use resl::{evaluate, ParseError};

// Safe error handling with ?
fn safe_evaluate(input: &str) -> Result<(), ParseError> {
    let result = evaluate(input)?;
    println!("Success: {}", result);
    Ok(())
}

// Pattern matching for specific error handling
match evaluate("invalid syntax") {
    Ok(value) => println!("Result: {}", value),
    Err(e) => eprintln!("Parse error: {}", e),
}

// Undefined variables become null, not errors
let result = evaluate("{ x = undefined_var; x }").unwrap();
// result is Value::Null
```

### 📊 Value Types

```rust
use resl::Value;

let result = evaluate(r#"["mixed": [1, "hello", true, null]]"#)?;

match result {
    Value::Map(map) => {
        if let Some(Value::List(list)) = map.get("mixed") {
            for item in list {
                match item {
                    Value::Integer(i) => println!("Integer: {}", i),
                    Value::String(s) => println!("String: {}", s),
                    Value::Boolean(b) => println!("Boolean: {}", b),
                    Value::Null => println!("Null value"),
                    _ => println!("Other type"),
                }
            }
        }
    },
    _ => {},
}
```

## ⚙️ C/C++ (FFI)

RESL provides C-compatible bindings through the `resl-ffi` crate with C-specific headers and configuration in `resl-c`.

### 📦 Installation

The FFI library and header files are provided:

- **Windows**: `resl.dll` and `resl.lib`
- **Linux**: `libresl.so`
- **macOS**: `libresl.dylib`

The C header file is available as `resl.h`.

### 📚 C API Reference

Based on the header file, RESL provides these core functions:

#### ⚡ Evaluation Functions

- `ReslValue* resl_evaluate(const char* input)` - Evaluates RESL expression
- `ReslString resl_format(const char* input, bool pretty)` - Formats RESL code
- `ReslString resl_evaluate_and_format(const char* input, bool pretty)` - Evaluates and formats

#### 🧠 Memory Management

- `void resl_value_free(ReslValue* val)` - Frees ReslValue and children recursively
- `void resl_string_free(ReslString s)` - Frees ReslString

#### 📊 Value Types

The `ReslValue` struct uses a tagged union with these types:

- `Null` - Null value
- `String` - UTF-8 string with pointer and length
- `Integer` - 64-bit signed integer
- `Float` - Double precision float
- `Boolean` - Boolean value
- `List` - Array of ReslValue pointers
- `Map` - Array of key-value pairs

### 💻 C Usage

```c
#include "resl.h"
#include <stdio.h>

int main() {
    ReslValue* result = resl_evaluate("{x = 10; y = 20; x + y}");

    if (result && result->tag == Integer) {
        printf("Result: %lld\n", result->payload.integer);
        resl_value_free(result);
    } else {
        printf("Evaluation failed\n");
        return -1;
    }

    return 0;
}
```

### Error Handling

C FFI returns `NULL` pointers to indicate parse errors. RESL evaluation itself never fails - undefined variables become `null` values:

```c
// Parse errors return NULL
ReslValue* result = resl_evaluate("invalid syntax");
if (!result) {
    printf("Error: Parse failed\n");
    return -1;
}

// Undefined variables become null values, not errors
ReslValue* undefined = resl_evaluate("{ x = undefined_var; x }");
if (undefined && undefined->tag == Null) {
    printf("Undefined variable became null\n");
}
resl_value_free(undefined);
```

### 🔧 C++ Usage

```cpp
#include "resl.h"
#include <iostream>

class ReslWrapper {
    ReslValue* value_;
public:
    explicit ReslWrapper(const std::string& source)
        : value_(resl_evaluate(source.c_str())) {}

    ~ReslWrapper() { if (value_) resl_value_free(value_); }

    bool is_valid() const { return value_ != nullptr; }

    long long as_integer() const {
        return (value_ && value_->tag == Integer) ? value_->payload.integer : 0;
    }
};

int main() {
    ReslWrapper result("{x = 42; y = 58; x + y}");

    if (result.is_valid()) {
        std::cout << "Result: " << result.as_integer() << std::endl;
    } else {
        std::cerr << "Evaluation failed" << std::endl;
        return -1;
    }

    return 0;
}
```

### ⚠️ Error Handling

C++ can use RAII and exceptions for cleaner error handling. Only parse errors exist - evaluation never fails:

```cpp
// RAII wrapper automatically handles cleanup
ReslWrapper result("invalid syntax");
if (!result.is_valid()) {
    std::cerr << "Error: Parse failed" << std::endl;
    return -1;
}

// Undefined variables become null, not errors
ReslWrapper undefined("{ x = undefined_var; x }");
if (undefined.is_valid()) {
    // Check for null value
    auto val = static_cast<ReslValue*>(undefined.get());
    if (val->tag == Null) {
        std::cout << "Undefined variable became null" << std::endl;
    }
}
```

## 🚀 Performance Considerations

### 🦀 Rust

- **Fastest**: Direct function calls, no FFI overhead
- **Memory**: Zero-copy for string operations where possible
- **Threading**: Safe for concurrent use with proper synchronization

### ⚙️ C/C++

- **Fast**: Minimal FFI overhead
- **Memory**: Manual memory management required (call `resl_value_free`)
- **Threading**: Thread-safe for evaluation, not for shared values

## 🤝 Future Language Support

We're planning support for additional languages and welcome contributions from the community:

**Planned Languages:**

- **Python**: Native extension or ctypes implementation
- **JavaScript**: WebAssembly for browsers, native modules for Node.js
- **Go**: CGO-based bindings
- **Java**: JNI integration for enterprise applications
- **WebAssembly**: Direct WASM compilation for web environments

The existing FFI layer provides a stable foundation for implementing bindings in any language that supports C interop.

::: tip CONTRIBUTIONS
**Interested in contributing?**  
Check out our [Contributing Guide - Language Bindings](https://github.com/decipher3114/resl/blob/main/CONTRIBUTING.md#language-bindings) for implementation guidance and best practices.
:::
