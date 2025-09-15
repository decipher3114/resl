---
title: Syntax Guide
description: Complete language syntax and features
---

# 📖 Syntax Guide

Learn RESL's syntax for building expressive, maintainable configuration files.

::: info 💭 No Comments in RESL
RESL intentionally excludes comment syntax to maintain smaller file sizes and encourage self-documenting code through meaningful variable names and clear structure.
:::

## Basic Example

```resl
{
    app_name = "my-service";
    port = 8080;
    debug = true;

    database = [
        "host": "localhost",
        "port": 5432,
        "ssl": false
    ];

    endpoints = [
        "health": "/health",
        "api": "/api/v1"
    ];

    final_config = [
        "app": app_name,
        "server": ["port": port, "debug": debug],
        "database": database,
        "routes": endpoints
    ];
}
```

RESL supports all essential configuration needs: numbers, strings, booleans, null values, lists, maps, arithmetic operations, comparisons, conditionals, functions, and transformations.

For comprehensive guidelines on writing clean configurations, see our [Best Practices](best-practices) guide.
