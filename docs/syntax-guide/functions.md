---
title: Functions
description: Function declarations, calls, and built-in utilities in RESL
---

# 🔧 Functions

Functions enable code reuse and modular configuration by encapsulating reusable logic. RESL supports both user-defined functions and a rich set of built-in utilities.

## 📋 Function Declaration

Functions are declared using the pipe syntax `|param1, param2| body`:

### Basic Functions

```resl
{
    add = |x, y| x + y;

    square = |n| n * n;

    get_version = || "1.2.0";

    result1 = add(5, 3);
    result2 = square(7);
    version = get_version();

    ["sum": result1, "square": result2, "version": version]
}
```

### Multi-Statement Functions

Use block expressions for complex function bodies:

```resl
{
    calculate_stats = |numbers| {
        total = length(numbers);
        sum_result = numbers > (i, n) : n;
        average = sum_result / total;

        ["count": total, "sum": sum_result, "avg": average]
    };

    data = [10, 20, 30, 40];
    stats = calculate_stats(data);

    stats
}
```

### Configuration Builders

```resl
{
    build_service_config = |name, port, replicas| [
        "name": name,
        "image": concat("app/", name, ":latest"),
        "port": port,
        "replicas": replicas,
        "env": [
            "SERVICE_NAME": name,
            "PORT": to_str(port)
        ]
    ];

    auth_service = build_service_config("auth", 8080, 3);
    api_service = build_service_config("api", 8081, 2);

    ["services": [auth_service, api_service]]
}
```

## 📞 Function Calls

Call functions by name with parentheses:

### Basic Calls

```resl
{
    multiply = |x, y| x * y;
    greet = |name| concat("Hello, ", name, "!");

    product = multiply(6, 7);
    greeting = greet("Alice");

    ["product": product, "greeting": greeting]
}
```

### Nested Calls

```resl
{
    double = |x| x * 2;
    add_ten = |x| x + 10;

    result = double(add_ten(5));

    ["result": result]
}
```

### Higher-Order Functions

Functions can accept other functions as parameters:

```resl
{
    apply_twice = |func, value| func(func(value));
    increment = |x| x + 1;

    result = apply_twice(increment, 5);

    ["result": result]
}
```

## 🛠️ Built-in Functions

RESL provides several built-in functions for common operations.

### 📝 String Functions

#### `concat(args...)`

Concatenates string values together. Non-string arguments are ignored.

```resl
{
    greeting = concat("Hello", " ", "World");

    message = concat("Count: ", 42, " items");

    api_path = concat("/api/", "v1", "/", "users");

    ["greeting": greeting, "path": api_path]
}
```

#### `to_str(value)`

Converts any value to its string representation.

```resl
{
    num_str = to_str(42);
    float_str = to_str(3.14);

    bool_str = to_str(true);

    null_str = to_str(null);

    message = concat("Port: ", to_str(8080));

    ["number": num_str, "message": message]
}
```

### 📦 Collection Functions

#### `length(collection)`

Returns the length of strings, lists, or maps.

```resl
{
    text_len = length("Hello");

    items = ["a", "b", "c", "d"];
    item_count = length(items);

    user = ["name": "Alice", "age": 30, "city": "NYC"];
    field_count = length(user);

    ["text": text_len, "items": item_count, "fields": field_count]
}
```

#### `push(list, value)`

Adds a value to the end of a list and returns the new list.

```resl
{
    original = [1, 2, 3];
    with_four = push(original, 4);

    names = ["Alice", "Bob"];
    with_charlie = push(names, "Charlie");

    numbers = [1];
    more_numbers = push(push(numbers, 2), 3);

    ["with_four": with_four, "names": with_charlie]
}
```

#### `insert(collection, key, value)`

Inserts a value into a collection at the specified key/index.

**For Maps:**

```resl
{
    user = ["name": "Alice"];
    with_age = insert(user, "age", 30);
    with_city = insert(with_age, "city", "NYC");

    ["user": with_city]
}
```

**For Lists (supports negative indices):**

```resl
{
    numbers = [1, 3, 4];

    with_zero = insert(numbers, 0, 0);

    fixed = insert(numbers, 1, 2);

    with_five = insert(numbers, -1, 5);

    ["fixed": fixed, "with_five": with_five]
}
```

### 🔧 Utility Functions

#### `type_of(value)`

Returns the type of a value as a string.

```resl
{
    int_type = type_of(42);
    float_type = type_of(3.14);
    str_type = type_of("hello");
    bool_type = type_of(true);
    null_type = type_of(null);

    list_type = type_of([1, 2, 3]);
    map_type = type_of(["a": 1]);

    types_info = [
        "integer": int_type,
        "list": list_type,
        "map": map_type
    ];

    types_info
}
```

#### `debug(value)`

Prints the value to stdout and returns the value unchanged. Useful for debugging.

```resl
{
    value = 42;

    result = debug(value);

    calculation = debug(5 + 3) * 2;

    ["result": result, "calculation": calculation]
}
```

## 🚀 Advanced Function Patterns

### Configuration Factories

```resl
{
    create_database_config = |env, host, port| [
        "host": host,
        "port": port,
        "ssl": ? (env == "production") : true | false,
        "pool_size": ? (env == "production") : 20 | 5,
        "timeout": ? (env == "production") : 30 | 10
    ];

    prod_db = create_database_config("production", "prod-db.com", 5432);
    dev_db = create_database_config("development", "localhost", 5432);

    ["production": prod_db, "development": dev_db]
}
```

### Validation Functions

```resl
{
    validate_port = |port| ? (port > 0 && port < 65536) : port | 8080;
    validate_name = |name| ? (length(name) > 0) : name | "default";

    create_service = |name, port| [
        "name": validate_name(name),
        "port": validate_port(port),
        "status": "configured"
    ];

    service1 = create_service("api", 8080);
    service2 = create_service("", 99999);

    ["service1": service1, "service2": service2]
}
```

### Data Transformation

```resl
{
    transform_user = |raw_user| [
        "id": raw_user["id"],
        "name": raw_user["name"],
        "email": raw_user["email"],
        "display_name": concat(raw_user["name"], " <", raw_user["email"], ">"),
        "is_active": ? (raw_user["status"] == "active") : true | false
    ];

    raw_users = [
        ["id": 1, "name": "Alice", "email": "alice@example.com", "status": "active"],
        ["id": 2, "name": "Bob", "email": "bob@example.com", "status": "inactive"]
    ];

    users = raw_users > (i, user) : transform_user(user);

    ["users": users]
}
```

### Environment Builders

```resl
{
    build_env_vars = |app_name, env, config| {
        base_vars = [
            "APP_NAME": app_name,
            "ENVIRONMENT": env,
            "LOG_LEVEL": ? (env == "production") : "error" | "debug"
        ];

        base_vars
    };

    auth_env = build_env_vars("auth-service", "production", ["DB_POOL": "20"]);
    api_env = build_env_vars("api-service", "development", ["DEBUG": "true"]);

    ["auth": auth_env, "api": api_env]
}
```
