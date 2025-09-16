---
title: Blocks
description: Block expressions with variable bindings and scoping in RESL
---

# 🏗️ Blocks

Blocks are expressions that create scoped environments for variable bindings and computation. They are enclosed in curly braces `{}` and must contain at least one binding followed by a return expression.

## 📝 Basic Syntax

```resl
{
    variable = value;
    another_var = expression;

    return_expression
}
```

**Requirements:**

1. **At least one binding** (`variable = value;`)
2. **A return expression** (the last expression without semicolon)

## 🔒 Variable Scoping

Variables are scoped to their block and accessible in nested blocks:

```resl
{
    outer_var = "accessible everywhere";

    config = {
        inner_var = "only accessible inside";
        result = concat(outer_var, " and ", inner_var);

        result
    };

    config
}
```

## 🎯 Common Use Cases

### Configuration Building

```resl
{
    app_name = "my-service";
    base_port = 8080;

    service_config = {
        name = app_name;
        image = concat("registry/", name, ":latest");
        port = base_port;

        ["name": name, "image": image, "port": port]
    };

    ["service": service_config]
}
```

### Environment-Specific Settings

```resl
{
    env = "production";

    config = ? (env == "production") : {
        host = "prod-server.com";
        ssl = true;

        ["host": host, "ssl": ssl]
    } | {
        host = "localhost";
        ssl = false;

        ["host": host, "ssl": ssl]
    };

    config
}
```
