# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in RESL, please report it responsibly:

1. **Do not** open a public issue
2. Email: [maintainer email] (replace with actual email)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will respond within 48 hours and provide updates on the resolution timeline.

## Security Considerations

RESL is designed as a configuration language and should not be used to execute untrusted code. While RESL expressions are evaluated, they operate in a controlled environment without system access.

### Safe Usage

- ✅ Parse configuration files from trusted sources
- ✅ Use for application configuration
- ✅ Generate configurations dynamically

### Avoid

- ❌ Executing RESL from untrusted user input
- ❌ Using RESL as a general-purpose scripting language
- ❌ Processing RESL files from untrusted sources without validation
