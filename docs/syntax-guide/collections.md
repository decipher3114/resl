---
title: Collections
description: Lists and maps for organizing data in RESL
---

# 📦 Collections

Collections allow you to organize multiple values into structured data. RESL provides two main collection types: lists for ordered sequences and maps for key-value relationships.

## 📋 Lists

Lists hold ordered sequences of values and are defined using square brackets `[]`.

### Basic Lists

```resl
[1, 2, 3]
["apple", "banana", "cherry"]
[1, "hello", true, null]
[]
```

### Nested Lists

Lists can contain other lists for complex data structures:

```resl
[
    [1, 2, 3],
    ["a", "b", "c"],
    [true, false]
]
```

### Configuration Examples

```resl
{
    ports = [8080, 8081, 8082];
    environments = ["development", "staging", "production"];
    features = ["auth", "logging", "metrics"];

    server_config = [
        "ports": ports,
        "environments": environments,
        "enabled_features": features
    ];

    server_config
}
```

## 🗺️ Maps

Maps store key-value pairs using the `["key": value]` syntax, similar to objects in JSON but with explicit brackets.

### Basic Maps

```resl
["name": "Alice", "age": 30]
["x": 10, "y": 20]
["enabled": true, "timeout": 30]
```

### Nested Maps

Maps can contain other maps and lists:

```resl
[
    "user": ["name": "Alice", "age": 30],
    "settings": ["theme": "dark", "notifications": true],
    "coordinates": ["x": 10, "y": 20, "point": ["x": 5, "y": 7]]
]
```

### Configuration Examples

```resl
{
    database = [
        "host": "localhost",
        "port": 5432,
        "credentials": [
            "username": "admin",
            "password": "secret"
        ]
    ];

    server = [
        "host": "0.0.0.0",
        "port": 8080,
        "ssl": [
            "enabled": true,
            "cert_path": "/etc/ssl/cert.pem"
        ]
    ];

    ["database": database, "server": server]
}
```

## 📌 Index Access

Access elements in collections using square bracket notation.

### List Indexing

Lists use zero-based indexing:

```resl
{
    numbers = [10, 20, 30, 40];
    colors = ["red", "green", "blue"];

    first_number = numbers[0];
    last_color = colors[2];

    ["first": first_number, "last": last_color]
}
```

### Map Access

Maps use string keys for access:

```resl
{
    user = ["name": "Alice", "age": 30, "city": "New York"];

    user_name = user["name"];
    user_age = user["age"];

    ["user_name": user_name, "user_age": user_age]
}
```

### Nested Access

Access nested values by chaining index operations:

```resl
{
    config = [
        "database": [
            "host": "localhost",
            "credentials": ["username": "admin"]
        ]
    ];

    db_host = config["database"]["host"];
    username = config["database"]["credentials"]["username"];

    ["host": db_host, "user": username]
}
```

## ✂️ Range Slicing

Extract ranges from lists using `[start:end]` syntax (lists only).

### Basic Slicing

```resl
{
    numbers = [0, 1, 2, 3, 4, 5];

    middle = numbers[1:4];
    from_two = numbers[2:];
    first_three = numbers[:3];

    ["middle": middle, "from_two": from_two, "first_three": first_three]
}
```

### Practical Examples

```resl
{
    log_entries = ["error", "info", "warn", "debug", "info", "error"];

    recent_logs = log_entries[3:];
    first_half = log_entries[:3];
    middle_section = log_entries[1:4];

    ["recent": recent_logs, "first": first_half, "middle": middle_section]
}
```

## 🔍 Collection Operations

Collection operations like `length()`, `push()`, and `insert()` are covered in detail in the [Functions - Collection Functions](functions#-collection-functions) section.

### Quick Reference

- **`length(collection)`** - Get the size of strings, lists, or maps
- **`push(list, value)`** - Add element to end of list
- **`insert(collection, key, value)`** - Insert value at key/index

See [Functions](functions) for complete documentation with examples.

```

## 🔄 Collection Transformations

Collection transformations using the for-each operator `>` are covered in detail in the [Control Flow - For-Each Transformations](control-flow#-for-each-transformations) section.

### Quick Reference

- **List transformation**: `list > (index, element) : expression`
- **Map transformation**: `map > (key, value) : expression`

See [Control Flow](control-flow) for complete documentation with examples.
```

## 🎯 Common Patterns

### Configuration Arrays

```resl
{
    services = [
        ["name": "auth", "port": 8080],
        ["name": "api", "port": 8081],
        ["name": "web", "port": 8082]
    ];

    service_ports = services > (i, service) : service["port"];

    ["services": services, "ports": service_ports]
}
```

### Environment-Specific Lists

```resl
{
    env = "production";

    allowed_origins = ? (env == "production") :
        ["https://app.example.com", "https://api.example.com"] |
        ["http://localhost:3000", "http://localhost:8080"];

    ["cors_origins": allowed_origins]
}
```

### Dynamic Map Building

```resl
{
    features = ["auth", "logging", "metrics"];

    feature_config = features > (i, feature) : [feature, true];

    config_map = [
        "app_name": "my-app",
        "version": "1.0.0"
    ];

    ["config": config_map, "features": feature_config]
}
```

### Nested Configuration

```resl
{
    environments = ["dev", "staging", "prod"];

    database_configs = environments > (i, env) : [
        "environment": env,
        "config": [
            "host": concat(env, "-db.example.com"),
            "port": 5432,
            "ssl": ? (env == "prod") : true | false
        ]
    ];

    ["databases": database_configs]
}
```
