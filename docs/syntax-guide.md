---
title: Syntax Guide
description: Complete language syntax and features
---

# 📖 Syntax Guide

Learn RESL's syntax for expressions, data types, variables, functions, and control flow.

::: info 💭 Why No Comments?
RESL intentionally excludes comment syntax. This design choice reinforces our core principle of **smaller file sizes** to save bandwidth and storage. Instead of comments, RESL encourages self-documenting code through meaningful variable names and clear structure. This keeps configurations concise while maintaining readability.
:::

## 📊 Basic Data Types

### 🔢 Numbers

RESL supports both integers and floating-point numbers:

```resl
42
-17
3.14159
-2.5
0
```

### 📝 Strings

Strings use double quotes and support escape sequences:

```resl
"Hello, World!"
"Line 1\nLine 2"
"Say \"Hello\""
"Path: C:\\temp"
```

### ✅ Booleans

```resl
true
false
```

### ⚫ Null

Represents the absence of a value:

```resl
null
```

## 📦 Collections

### 📋 Lists

Lists hold ordered sequences of values:

```resl
[1, 2, 3]
["apple", "banana", "cherry"]
[1, "hello", true, null]
[]
```

### 🗺️ Maps

Maps store key-value pairs using the `["key": value]` syntax:

```resl
["name": "Alice", "age": 30]
["x": 10, "y": 20, "point": ["x": 5, "y": 7]]
```

## 🔧 Variables and Assignment

Define variables using the assignment operator `=`:

```resl
{
    name = "Alice";
    age = 30;
    is_admin = true;

    greeting = concat("Hello, ", name);

    ["user": name, "message": greeting]
}
```

Variables are scoped to their containing block and can reference any variable defined earlier in the same scope.

## ⚡ Binary Operations

RESL supports binary operations with exactly two operands:

### ➕ Arithmetic

```resl
5 + 3
10 - 4
6 * 7
15 / 3
17 % 5
```

### ⚖️ Comparison

```resl
5 == 5
3 != 7
10 > 5
(3 < 8)
5 >= 5
4 <= 9
```

### 🔗 Logical

```resl
true && false
true || false
```

## ☝️ Unary Operations

Unary operations work on a single operand:

```resl
-x
!true
```

## 📦 Block Expressions

Blocks group multiple statements and return the value of the last expression:

```resl
{
    x = 10;
    y = 20;
    z = x + y;
    z * 2
}
```

Blocks create their own scope for variables. Child blocks can access parent variables, but parent blocks cannot access child variables:

```resl
{
    outer = "parent";

    result = {
        inner = "child";
        concat(outer, " -> ", inner)
    };

    result
}
```

## 🔧 Functions

### 📋 Function Declaration

Functions are declared using the `|param1, param2| body` syntax:

```resl
{
    add = |x, y| x + y;

    calculate = |a, b| {
        sum = a + b;
        product = a * b;
        ["sum": sum, "product": product]
    };

    result1 = add(5, 3);
    result2 = calculate(4, 6);

    [result1, result2]
}
```

### 📞 Function Calls

Call functions by name with parentheses:

```resl
{
    multiply = |x, y| x * y;
    square = |n| multiply(n, n);

    square(7)
}
```

## 🔀 Conditional Expressions

Use the ternary operator `? condition : then_value | else_value`:

```resl
{
    age = 25;
    status = ? (age >= 18) : "adult" | "minor";

    x = 10;
    result = ? (x > 5) : "big" | "small";

    score = 85;
    grade = ? (score >= 90) : "A"
          | ? (score >= 80) : "B"
          | ? (score >= 70) : "C"
          | "F";

    [status, result, grade]
}
```

## 🎯 Collection Access and Slicing

### 📌 Index Access

Access elements using square brackets:

```resl
{
    numbers = [10, 20, 30, 40];
    data = ["name": "Alice", "age": 30];

    first = numbers[0];
    name = data["name"];

    [first, name]
}
```

### ✂️ Range Slicing

Extract ranges from lists using `[start:end]` syntax:

```resl
{
    numbers = [0, 1, 2, 3, 4, 5];

    slice1 = numbers[1:4];
    slice2 = numbers[2:];
    slice3 = numbers[:3];

    [slice1, slice2, slice3]
}
```

## 🔄 Collection Transformations

### 🎭 For-Each with `>`

Transform collections using the `>` operator with `(index, element) : expression` syntax (for lists) and `(key, value) : expression` syntax (for map):

```resl
{
    numbers = [1, 2, 3, 4];

    doubled = numbers > (i, n) : n * 2;

    indexed = numbers > (i, n) : ["pos": i, "val": n];

    users = [["name": "Alice"], ["name": "Bob"]];
    greetings = users > (i, user) : concat("Hello, ", user["name"]);

    [doubled, indexed, greetings]
}
```

## 🛠️ Built-in Functions

RESL provides several built-in functions for common operations:

### 📝 String Functions

#### 🔗 `concat(args...)`

Concatenates string values together. Only string arguments are used; non-strings are ignored.

```resl
concat("Hello", " ", "World")
```

_Returns: `"Hello World"`_

#### 🔤 `to_str(value)`

Converts any value to its string representation.

```resl
to_str(42)
```

_Returns: `"42"`_

### 📦 Collection Functions

#### 📏 `length(collection)`

Returns the length of strings, lists, or maps. Returns the character count for strings.

```resl
length("Hello")
```

_Returns: `5`_

```resl
length([1, 2, 3])
```

_Returns: `3`_

#### ➕ `push(list, value)`

Adds a value to the end of a list and returns the new list.

```resl
push([1, 2], 3)
```

_Returns: `[1, 2, 3]`_

#### 📝 `insert(collection, key, value)`

Inserts a value into a collection at the specified key/index.

For maps:

```resl
insert(["a": 1], "b", 2)
```

_Returns: `["a": 1, "b": 2]`_

For lists (supports negative indices):

```resl
insert([1, 3], 1, 2)
```

_Returns: `[1, 2, 3]`_

### 🔧 Utility Functions

#### 🏷️ `type_of(value)`

Returns the type of a value as a string.

```resl
type_of(42)
```

_Returns: `"integer"`_

```resl
type_of([1, 2, 3])
```

_Returns: `"list"`_

Possible return values: `"null"`, `"boolean"`, `"integer"`, `"float"`, `"string"`, `"list"`, `"map"`

#### 🐛 `debug(value)`

Prints the value to stdout and returns the value unchanged. Useful for debugging.

```resl
debug("Hello")
```

_Prints: `Hello` and returns: `"Hello"`_

### 💡 Usage Examples

```resl
{
    name = "Alice";
    age = 30;

    greeting = concat("Hello, ", name, "!");
    info = concat(name, " is ", to_str(age), " years old");

    items = ["apple", "banana"];
    fruits = push(items, "cherry");

    person = ["name": name];
    profile = insert(person, "age", age);

    [
        "greeting": greeting,
        "info": info,
        "fruits": fruits,
        "profile": profile,
        "fruit_count": length(fruits)
    ]
}
```

## 🚀 Advanced Patterns

### 📋 Configuration Templates

```resl
{
    base_port = 8080;
    host = "localhost";
    version = "v1.2.0";

    env = "production";
    db_host = ? (env == "production") : "prod-db.example.com" | "localhost";
    log_level = ? (env == "production") : "error" | "debug";

    services = ["auth", "api", "worker"];

    deployments = services > (i, name) : [
        "name": name,
        "image": concat("app/", name, ":", version),
        "port": base_port + i,
        "env": [
            "DB_HOST": db_host,
            "LOG_LEVEL": log_level,
            "SERVICE_NAME": name
        ]
    ];

    ["services": deployments, "environment": env]
}
```

### 🧮 Computed Configurations

```resl
{
    servers = [
        ["name": "web1", "cpu": 2, "memory": 4],
        ["name": "web2", "cpu": 4, "memory": 8],
        ["name": "db1", "cpu": 8, "memory": 16]
    ];

    total_cpu = sum(servers > (i, s) : s["cpu"]);
    total_memory = sum(servers > (i, s) : s["memory"]);
    avg_cpu = total_cpu / len(servers);

    allocations = servers > (i, server) : [
        "server": server["name"],
        "cpu_percent": (server["cpu"] * 100) / total_cpu,
        "memory_percent": (server["memory"] * 100) / total_memory,
        "tier": ? (server["cpu"] >= 8) : "high"
              | ? (server["cpu"] >= 4) : "medium"
              | "low"
    ];

    [
        "summary": ["total_cpu": total_cpu, "total_memory": total_memory, "avg_cpu": avg_cpu],
        "allocations": allocations
    ]
}
```

## 🎯 Top-Level Flexibility

Unlike JSON, RESL allows any expression at the top level:

```resl
"Hello, World!"

[1, 2, 3]

(5 + 3) * 2

? (true) : "yes" | "no"

{
    x = 10;
    y = 20;
    x + y
}
```

This flexibility makes RESL suitable for a wide range of use cases beyond traditional configuration files.

## 💎 Best Practices

1. **Use descriptive variable names** to make configurations self-documenting
2. **Group related variables** in logical blocks
3. **Leverage computed values** to reduce repetition and maintain consistency
4. **Use conditionals** for environment-specific configurations
5. **Structure complex configurations** using functions and transformations
6. **Comment your intentions** using meaningful variable names rather than comments

RESL's syntax is designed to be both powerful and intuitive, enabling you to create maintainable, expressive configuration files that grow with your needs.
