---
title: Operations
description: Operators for combining and transforming values in RESL
---

# ⚡ Operations

Operations allow you to combine, compare, and transform values using familiar operators. RESL (Runtime Evaluated Serialization Language) provides both binary operations (two operands) and unary operations (single operand).

## ➕ Binary Operations

Binary operations work with exactly two operands and support various types of value manipulation.

### Arithmetic Operations

Perform mathematical calculations on numbers:

```resl
{
    sum = 5 + 3;
    difference = 10 - 4;
    product = 6 * 7;
    quotient = 15 / 3;
    remainder = 17 % 5;

    result = (10 + 5) * 2 - 3;

    ["sum": sum, "product": product, "result": result]
}
```

### String Concatenation

The `+` operator also works with strings:

```resl
{
    greeting = "Hello" + " " + "World";
    path = "/api" + "/" + "users";

    name = "Alice";
    message = "Welcome, " + name + "!";

    ["greeting": greeting, "message": message]
}
```

### Comparison Operations

Compare values and return boolean results:

```resl
{
    is_equal = 5 == 5;
    not_equal = 3 != 7;

    greater = 10 > 5;
    less = 3 < 8;
    greater_equal = 5 >= 5;
    less_equal = 4 <= 9;

    name_match = "Alice" == "Alice";

    ["equal": is_equal, "greater": greater, "name_match": name_match]
}
```

### Logical Operations

Combine boolean values with logical operators:

```resl
{
    both_true = true && true;
    one_false = true && false;

    at_least_one = true || false;
    both_false = false || false;

    age = 25;
    has_license = true;
    can_drive = (age >= 18) && has_license;

    ["both_true": both_true, "can_drive": can_drive]
}
```

## ☝️ Unary Operations

Unary operations work on a single operand.

### Arithmetic Negation

Use `-` to negate numbers:

```resl
{
    positive = 42;
    negative = -positive;
    double_negative = -(-5);

    offset = -10;
    position = 100 + offset;

    ["negative": negative, "position": position]
}
```

### Logical Negation

Use `!` to negate boolean values:

```resl
{
    is_enabled = true;
    is_disabled = !is_enabled;

    debug_mode = false;
    production = !debug_mode;

    age = 16;
    is_adult = age >= 18;
    is_minor = !is_adult;

    ["disabled": is_disabled, "minor": is_minor]
}
```

## 🔄 Operator Precedence

RESL follows standard mathematical precedence rules:

### Precedence Order (highest to lowest)

1. **Parentheses** `()`
2. **Unary operators** `-`, `!`
3. **Multiplication/Division/Modulo** `*`, `/`, `%`
4. **Addition/Subtraction** `+`, `-`
5. **Comparison** `<`, `<=`, `>`, `>=`
6. **Equality** `==`, `!=`
7. **Logical AND** `&&`
8. **Logical OR** `||`

### Examples

```resl
{
    result1 = 2 + 3 * 4;
    result2 = 10 - 6 / 2;

    result3 = (2 + 3) * 4;
    result4 = (10 - 6) / 2;

    x = 5;
    y = 10;
    complex = x > 3 && y < 15 || x == 0;

    ["result1": result1, "result3": result3, "complex": complex]
}
```

## 🎯 Practical Examples

### Configuration Calculations

```resl
{
    base_port = 8080;
    instance_count = 3;

    ports = [
        base_port,
        base_port + 1,
        base_port + 2
    ];

    total_memory = instance_count * 512;

    ["ports": ports, "total_memory_mb": total_memory]
}
```

### Environment Logic

```resl
{
    environment = "production";
    debug_flag = false;
    is_development = environment == "development";
    is_production = environment == "production";
    debug_enabled = debug_flag || is_development;
    ssl_required = is_production && !debug_enabled;

    config = [
        "debug": debug_enabled,
        "ssl": ssl_required,
        "log_level": ? debug_enabled : "debug" | "error"
    ];

    config
}
```

### String Building

```resl
{
    app_name = "my-service";
    version = "1.2.0";
    environment = "prod";

    image_base = "registry.com/" + app_name;
    image_tag = version + "-" + environment;
    full_image = image_base + ":" + image_tag;

    base_url = "https://" + environment + ".example.com";
    api_endpoint = base_url + "/api/v1";

    [
        "image": full_image,
        "api_url": api_endpoint
    ]
}
```

### Numeric Calculations

```resl
{
    cpu_cores = 4;
    memory_gb = 16;
    replicas = 3;

    total_cpu = cpu_cores * replicas;
    total_memory = memory_gb * replicas;
    memory_mb = total_memory * 1024;

    cpu_limit = cpu_cores * 0.8;
    memory_limit = memory_gb * 0.9;

    resources = [
        "total_cpu": total_cpu,
        "total_memory_gb": total_memory,
        "total_memory_mb": memory_mb,
        "cpu_limit": cpu_limit,
        "memory_limit_gb": memory_limit
    ];

    resources
}
```

## 🔍 Type Behavior

### Arithmetic Operations

- **Number + Number**: Mathematical addition
- **String + String**: String concatenation
- **Mixed types**: May produce errors or unexpected results

### Comparison Operations

- **Same types**: Natural comparison
- **Different types**: Generally `false` for equality, may error for ordering
- **Null comparisons**: `null == null` is `true`, `null != anything_else` is `true`

### Logical Operations

- **Truthy values**: Non-zero numbers, non-empty strings, `true`
- **Falsy values**: `0`, `""` (empty string), `false`, `null`

```resl
{
    num_add = 5 + 3;
    str_add = "5" + "3";

    same_type = 5 == 5;
    diff_type = 5 == "5";
    null_compare = null == null;

    ["num_add": num_add, "str_add": str_add, "same_type": same_type]
}
```
