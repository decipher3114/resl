---
title: Control Flow
description: Conditional expressions and iteration in RESL
---

# 🔀 Control Flow

Control flow allows you to make decisions and iterate over data in your configurations. RESL provides conditional expressions and powerful collection transformations.

## 🔄 Conditional Expressions

RESL uses the ternary operator `? condition : then_value | else_value` for conditional logic.

### Basic Conditionals

```resl
{
    age = 25;
    status = ? (age >= 18) : "adult" | "minor";

    is_weekend = true;
    work_mode = ? is_weekend : "relaxed" | "busy";

    score = 85;
    grade = ? (score >= 90) : "A" | "B";

    ["status": status, "mode": work_mode, "grade": grade]
}
```

### Nested Conditionals

Chain multiple conditions for complex logic:

```resl
{
    score = 85;

    grade = ? (score >= 90) : "A"
          | ? (score >= 80) : "B"
          | ? (score >= 70) : "C"
          | ? (score >= 60) : "D"
          | "F";

    temperature = 75;

    clothing = ? (temperature > 80) : "shorts"
             | ? (temperature > 60) : "pants"
             | "jacket";

    ["grade": grade, "clothing": clothing]
}
```

### Complex Conditions

Use logical operators in conditions:

```resl
{
    age = 25;
    has_license = true;
    has_car = false;

    can_drive = ? (age >= 16 && has_license) : true | false;
    transportation = ? (can_drive && has_car) : "drive"
                  | ? can_drive : "rideshare"
                  | "public_transport";

    time_of_day = "evening";
    is_weekend = true;

    activity = ? (is_weekend && time_of_day == "evening") : "social"
             | ? (is_weekend) : "relaxation"
             | "work";

    ["can_drive": can_drive, "transport": transportation, "activity": activity]
}
```

### Environment-Based Configuration

```resl
{
    environment = "production";
    debug_flag = false;

    log_level = ? (environment == "development") : "debug"
              | ? (environment == "staging") : "info"
              | "error";

    database_host = ? (environment == "production") : "prod-db.example.com"
                  | ? (environment == "staging") : "staging-db.example.com"
                  | "localhost";

    ssl_enabled = ? (environment == "production" || environment == "staging") : true | false;

    config = [
        "log_level": log_level,
        "database": ["host": database_host, "ssl": ssl_enabled],
        "debug": ? debug_flag : true | false
    ];

    config
}
```

## 🎭 For-Each Transformations

Transform collections using the `>` operator with `(index, element) : expression` syntax for lists and `(key, value) : expression` syntax for maps.

### Basic List Transformations

```resl
{
    numbers = [1, 2, 3, 4, 5];

    doubled = numbers > (i, n) : n * 2;

    indexed = numbers > (i, n) : ["pos": i, "val": n];

    evens = numbers > (i, n) : ? (n % 2 == 0) : n | null;

    ["doubled": doubled, "indexed": indexed, "evens": evens]
}
```

### Map Transformations

```resl
{
    users = [
        "alice": ["age": 30, "role": "admin"],
        "bob": ["age": 25, "role": "user"],
        "charlie": ["age": 35, "role": "user"]
    ];

    user_list = users > (name, info) : [
        "username": name,
        "age": info["age"],
        "role": info["role"],
        "is_admin": ? (info["role"] == "admin") : true | false
    ];

    ages = users > (name, info) : info["age"];

    ["users": user_list, "ages": ages]
}
```

### Complex Transformations

```resl
{
    services = ["auth", "api", "worker", "web"];
    base_port = 8080;
    environment = "production";

    service_configs = services > (i, name) : [
        "name": name,
        "image": concat("myapp/", name, ":latest"),
        "port": base_port + i,
        "replicas": ? (name == "web") : 3 | 1,
        "resources": [
            "cpu": ? (name == "worker") : "2000m" | "500m",
            "memory": ? (name == "worker") : "2Gi" | "512Mi"
        ],
        "env": [
            "SERVICE_NAME": name,
            "ENVIRONMENT": environment,
            "PORT": to_str(base_port + i)
        ]
    ];

    ["services": service_configs]
}
```

### Data Processing Patterns

```resl
{
    raw_data = [
        ["name": "Alice", "score": 85, "department": "engineering"],
        ["name": "Bob", "score": 92, "department": "marketing"],
        ["name": "Charlie", "score": 78, "department": "engineering"],
        ["name": "Diana", "score": 96, "department": "sales"]
    ];

    employee_reports = raw_data > (i, employee) : [
        "id": i + 1,
        "name": employee["name"],
        "department": employee["department"],
        "score": employee["score"],
        "grade": ? (employee["score"] >= 90) : "excellent"
               | ? (employee["score"] >= 80) : "good"
               | ? (employee["score"] >= 70) : "satisfactory"
               | "needs_improvement",
        "bonus_eligible": ? (employee["score"] >= 85) : true | false
    ];

    ["reports": employee_reports]
}
```

## 🎯 Combining Conditionals and Transformations

### Dynamic List Building

```resl
{
    features = ["auth", "logging", "metrics", "cache", "search"];
    environment = "production";
    budget_tier = "premium";

    enabled_features = features > (i, feature) :
        ? (feature == "auth") : feature
        | ? (feature == "logging") : feature
        | ? (feature == "metrics" && environment == "production") : feature
        | ? (feature == "cache" && budget_tier == "premium") : feature
        | ? (feature == "search" && budget_tier == "premium") : feature
        | null;

    ["enabled": enabled_features]
}
```

### Configuration Matrix

```resl
{
    environments = ["dev", "staging", "prod"];

    env_configs = environments > (i, env) : [
        "name": env,
        "database": [
            "host": concat(env, "-db.example.com"),
            "port": 5432,
            "ssl": ? (env == "prod") : true | false,
            "pool_size": ? (env == "prod") : 20 | ? (env == "staging") : 10 | 5
        ],
        "logging": [
            "level": ? (env == "dev") : "debug" | ? (env == "staging") : "info" | "error",
            "format": ? (env == "prod") : "json" | "text"
        ],
        "resources": [
            "cpu": ? (env == "prod") : "2000m" | ? (env == "staging") : "1000m" | "500m",
            "memory": ? (env == "prod") : "4Gi" | ? (env == "staging") : "2Gi" | "1Gi"
        ]
    ];

    ["environments": env_configs]
}
```

## 🔍 Advanced Patterns

### Validation and Defaults

```resl
{
    user_input = ["port": 0, "name": "", "ssl": null];

    validated_config = [
        "port": ? (user_input["port"] > 0) : user_input["port"] | 8080,
        "name": ? (length(user_input["name"]) > 0) : user_input["name"] | "default-service",
        "ssl": ? (user_input["ssl"] != null) : user_input["ssl"] | false
    ];

    ["config": validated_config]
}
```

### Feature Flags and Rollouts

```resl
{
    user_segments = ["new_users", "premium_users", "beta_testers"];

    feature_rollouts = user_segments > (i, segment) : [
        "segment": segment,
        "features": [
            "new_ui": ? (segment == "beta_testers") : true | false,
            "advanced_search": ? (segment == "premium_users" || segment == "beta_testers") : true | false,
            "mobile_app": ? (segment != "new_users") : true | false
        ]
    ];

    ["rollouts": feature_rollouts]
}
```

### Resource Scaling

```resl
{
    load_levels = ["low", "medium", "high", "peak"];

    scaling_configs = load_levels > (i, level) : [
        "load_level": level,
        "replicas": ? (level == "peak") : 10
                  | ? (level == "high") : 6
                  | ? (level == "medium") : 3
                  | 1,
        "resources": [
            "cpu": ? (level == "peak" || level == "high") : "2000m" | "1000m",
            "memory": ? (level == "peak") : "4Gi"
                    | ? (level == "high") : "2Gi"
                    | "1Gi"
        ],
        "autoscaling": [
            "enabled": ? (level == "peak" || level == "high") : true | false,
            "min_replicas": ? (level == "peak") : 5 | ? (level == "high") : 3 | 1,
            "max_replicas": ? (level == "peak") : 20 | ? (level == "high") : 10 | 5
        ]
    ];

    ["scaling": scaling_configs]
}
```
