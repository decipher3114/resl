---
title: Getting Started
description: Getting started with RESL - Runtime Evaluated Serialization Language
---

# 📖 Getting Started

Welcome to **RESL**! This guide will help you get started with RESL and understand how to use it effectively in your projects.

### 🎯 Try Your First Expression

Start with simple expressions:

```resl
5 + 3
```

_Evaluates to: `8`_

```resl
concat("Hello, ", "World!")
```

_Evaluates to: `"Hello, World!"`_

### ⚙️ Basic Configuration Example

Here's a practical configuration using RESL:

```resl
{
    port = 8080;
    host = "localhost";
    database = ["host": host, "port": 5432];
    app_url = concat("http://", host, ":", port);

    [
        "server": ["host": host, "port": port],
        "database": database,
        "url": app_url
    ]
}
```

_Evaluates to:_

```resl
[
  "server": [
    "host": "localhost",
    "port": 8080
  ],
  "database": [
    "host": "localhost",
    "port": 5432
  ],
  "url": "http://localhost:8080"
]
```

## 🔌 Using RESL in Your Applications

RESL integrates with applications in multiple languages:

::: code-group

```rust [Rust]
use resl::evaluate;

let result = evaluate("5 + 3").unwrap();
println!("{}", result); // 8
```

```c [C/C++]
#include "resl.h"

ReslValue* result = resl_evaluate("5 + 3");
printf("%lld\n", result->data.integer); // 8
resl_value_free(result);
```

:::

_See [Language Bindings](../usage/bindings) for complete integration details._

## 🎯 Learning Path

1. **Start Here**: Read [What is RESL](what-is-resl) to understand the fundamentals
2. **See the Benefits**: Check [Why RESL](why-resl) for comparisons with other formats
3. **Learn Syntax**: Work through the [Syntax Guide](../syntax-guide/overview.md) sections in order
4. **Try Tools**: Use the [CLI](../usage/cli-usage) to evaluate and format RESL files
5. **Integrate**: Add RESL to your projects with [Language Bindings](../usage/bindings)

## 🤝 Contributing

Interested in contributing to RESL? We welcome contributions of all kinds! See our [Contributing Guide](https://github.com/decipher3114/resl/blob/main/CONTRIBUTING.md) for:
