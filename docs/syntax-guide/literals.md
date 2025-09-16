---
title: Literals
description: Basic data types in RESL
---

# 📊 Literals

Literals are the fundamental building blocks of RESL expressions, representing basic data types that store individual values.

## 🔢 Numbers

RESL supports both integers and floating-point numbers with intuitive syntax.

### Integers

Whole numbers without decimal points:

```resl
42
-17
0
```

### Floating-Point Numbers

Numbers with decimal points:

```resl
3.14159
-2.5
0.0
42.0
```

### Usage Examples

```resl
{
    port = 8080;
    timeout = 30.5;
    offset = -10;

    ["server_port": port, "timeout_seconds": timeout, "time_offset": offset]
}
```

## 📝 Strings

Strings represent text values and must be enclosed in double quotes.

### Basic Strings

```resl
"Hello, World!"
"Configuration value"
"app-name"
""
```

### Escape Sequences

RESL supports standard escape sequences for special characters:

```resl
"Line 1\nLine 2"
"Say \"Hello\""
"Path: C:\\temp"
"Tab\tseparated"
```

### Common Escape Sequences

- `\"` - Double quote
- `\\` - Backslash
- `\n` - Newline

### Usage Examples

```resl
{
    app_name = "My Application";
    description = "A powerful\nconfiguration tool";
    file_path = "C:\\configs\\app.resl";
    message = "Status: \"ready\"";

    ["app": app_name, "desc": description, "path": file_path, "msg": message]
}
```

## ✅ Booleans

Boolean values represent true/false states:

```resl
true
false
```

### Usage Examples

```resl
{
    debug_mode = true;
    production = false;
    ssl_enabled = true;

    server_config = [
        "debug": debug_mode,
        "production": production,
        "ssl": ssl_enabled
    ];

    server_config
}
```

### In Conditionals

Booleans are commonly used with conditional expressions:

```resl
{
    is_production = true;
    log_level = ? is_production : "error" | "debug";

    ["environment": "production", "logging": log_level]
}
```

## ⚫ Null

The `null` literal represents the absence of a value:

```resl
null
```

### Usage Examples

```resl
{
    optional_field = null;
    default_value = null;

    config = [
        "required_field": "value",
        "optional_field": optional_field,
        "computed_field": ? (default_value == null) : "fallback" | default_value
    ];

    config
}
```

### Null Checking

```resl
{
    user_id = null;
    guest_mode = ? (user_id == null) : true | false;

    ["guest": guest_mode]
}
```

## 🎯 Type Behavior

### Automatic Type Recognition

RESL automatically recognizes literal types:

```resl
{
    number_int = 42;
    number_float = 42.0;
    text = "42";
    flag = true;
    empty = null;

    ["types": [number_int, number_float, text, flag, empty]]
}
```

### Type Checking

Use the `type_of()` built-in function to check types:

```resl
{
    value = 42;
    value_type = type_of(value);

    text = "hello";
    text_type = type_of(text);

    ["value_type": value_type, "text_type": text_type]
}
```

## 🔍 Common Patterns

### Environment Configuration

```resl
{
    environment = "development";
    debug = ? (environment == "development") : true | false;
    port = ? (environment == "production") : 80 | 3000;

    ["env": environment, "debug": debug, "port": port]
}
```

### Feature Flags

```resl
{
    feature_flags = [
        "new_ui": true,
        "beta_features": false,
        "analytics": true
    ];

    feature_flags
}
```

### Default Values

```resl
{
    user_config = null;
    timeout = ? (user_config == null) : 30 | user_config;

    ["timeout": timeout]
}
```
