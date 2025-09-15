# Best Practices

Essential guidelines for writing clean, maintainable RESL configurations.

## 1. Use Descriptive Variable Names

Make configurations self-documenting with clear, meaningful names.

```resl
{
    database_host = "prod-db.company.com";
    max_connection_count = 100;
    cache_timeout_seconds = 3600;
    enable_ssl_encryption = true;
}
```

## 2. Group Related Variables

Organize related configurations into logical blocks for better readability.

```resl
{
    database = [
        "host": "prod-db.company.com",
        "port": 5432,
        "max_connections": 100
    ];

    cache = [
        "provider": "redis",
        "timeout": 3600,
        "max_size": "512MB"
    ];

    logging = [
        "level": "info",
        "format": "json",
        "output": "/var/log/app.log"
    ];
}
```

## 3. Leverage Computed Values

Reduce repetition and maintain consistency with calculated values.

```resl
{
    base_url = "https://api.company.com";
    api_version = "v2";

    endpoints = [
        "users": concat(base_url, "/", api_version, "/users"),
        "orders": concat(base_url, "/", api_version, "/orders"),
        "products": concat(base_url, "/", api_version, "/products")
    ];
}
```

## 4. Use Conditionals for Environment-Specific Configurations

Handle different environments cleanly with conditional logic.

```resl
{
    environment = "production";

    database_host = if environment == "production" then
        "prod-db.company.com"
    else
        "dev-db.company.com";

    debug_enabled = environment != "production";
    log_level = if environment == "production" then "error" else "debug";
}
```

## 5. Structure Complex Configurations with Functions

Break down complex logic using functions and transformations.

```resl
{
    create_server_config = |name, port, ssl| [
        "name": name,
        "port": port,
        "ssl_enabled": ssl,
        "health_check": concat("http://", name, ":", to_str(port), "/health")
    ];

    servers = [
        create_server_config("web-server", 8080, true),
        create_server_config("api-server", 9000, true),
        create_server_config("admin-server", 8081, false)
    ];
}
```

## 6. Keep Expressions Readable

Break complex logic into smaller, understandable parts.

```resl
{
    user_name = "john.doe";
    user_domain = "company.com";

    email_address = concat(user_name, "@", user_domain);

    is_valid_user = length(user_name) > 0 && length(user_domain) > 0;

    user_config = if is_valid_user then [
        "email": email_address,
        "active": true,
        "created": "2024-01-15"
    ] else [
        "error": "Invalid user configuration"
    ];
}
```
