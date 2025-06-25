# MCP Security Configuration

This document outlines the security measures implemented for the MCP (Model Context Protocol) servers in the dots-notifier project.

## Security Principles

### 1. Principle of Least Privilege
- MCP servers run with minimal required permissions
- Filesystem access limited to project directory only
- GitHub API access scoped to repository context

### 2. Defense in Depth
- Multiple layers of security controls
- Local-only communication (no remote network access)
- Process isolation between MCP servers

### 3. Secure by Default
- Conservative default configurations
- Optional features disabled by default
- Explicit permission grants required

## Access Controls

### Filesystem Server
```json
{
  "filesystem": {
    "allowed_directories": ["${workspaceFolder}"],
    "read_only": false,
    "max_file_size": "10MB",
    "excluded_patterns": [
      "target/*",
      "node_modules/*", 
      ".git/*",
      "*.log"
    ]
  }
}
```

### GitHub Server
```json
{
  "github": {
    "scope": "repository",
    "permissions": ["read"],
    "rate_limiting": true,
    "max_requests_per_hour": 100
  }
}
```

### Rust Analyzer
```json
{
  "rust_analyzer": {
    "workspace_only": true,
    "network_access": false,
    "cargo_check": true
  }
}
```

## Network Security

### Local Communication Only
- All MCP servers communicate via stdio
- No network sockets or external connections
- Process-to-process communication only

### API Token Management
- GitHub tokens stored in environment variables
- No tokens in configuration files
- Token scope limited to repository access

### Data Privacy
- No data transmission to external services
- Local processing only
- No telemetry or analytics

## Process Security

### Sandboxing
```bash
# Example systemd service with security restrictions
[Service]
Type=exec
ExecStart=/usr/bin/npx mcp-server-filesystem /workspace
User=mcp-user
Group=mcp-group

# Security restrictions
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/workspace
CapabilityBoundingSet=
AmbientCapabilities=
SystemCallFilter=@system-service
SystemCallErrorNumber=EPERM
```

### Resource Limits
```json
{
  "resource_limits": {
    "max_memory": "256MB",
    "max_cpu_percent": 10,
    "max_open_files": 100,
    "timeout_seconds": 30
  }
}
```

## Input Validation

### File Path Validation
- Canonical path resolution
- Directory traversal protection
- Symlink validation
- Size limits enforcement

### GitHub API Validation
- Input sanitization
- Rate limiting
- Response validation
- Error handling

### Command Validation
- Allowed command whitelist
- Argument validation
- Output sanitization
- Shell injection prevention

## Audit and Monitoring

### Logging
```json
{
  "logging": {
    "level": "INFO",
    "destinations": ["file", "syslog"],
    "retention_days": 30,
    "sensitive_data_redaction": true
  }
}
```

### Monitoring Metrics
- Request counts and rates
- Error rates and types
- Resource usage (CPU, memory, disk)
- Response times

### Alerting
- Unusual access patterns
- Resource exhaustion
- Error rate spikes
- Security policy violations

## Threat Model

### Threats Mitigated
1. **Unauthorized file system access** - Directory restrictions
2. **Code injection** - Input validation and sandboxing  
3. **Information disclosure** - Scope limitations
4. **Resource exhaustion** - Rate limiting and quotas
5. **Privilege escalation** - Process isolation

### Residual Risks
1. **Supply chain attacks** - NPM package dependencies
2. **Local privilege escalation** - Host system vulnerabilities
3. **Configuration errors** - Human error in setup

### Risk Mitigation
- Regular dependency updates
- Security scanning of packages
- Configuration validation
- Peer review of changes

## Compliance

### Security Standards
- Follows OWASP secure coding practices
- Implements defense-in-depth strategy
- Regular security assessments

### Data Protection
- No personal data collection
- Local data processing only
- User consent for repository access

### Access Management
- Role-based access control
- Regular access reviews
- Audit trail maintenance

## Security Checklist

### Initial Setup
- [ ] Verify MCP server signatures
- [ ] Configure filesystem restrictions
- [ ] Set up GitHub token with minimal scope
- [ ] Enable logging and monitoring
- [ ] Test security controls

### Regular Maintenance
- [ ] Update MCP server packages
- [ ] Rotate GitHub tokens
- [ ] Review access logs
- [ ] Validate configuration
- [ ] Security assessment

### Incident Response
- [ ] Disable compromised servers
- [ ] Revoke API tokens
- [ ] Analyze logs for impact
- [ ] Implement fixes
- [ ] Document lessons learned

## Security Contacts

For security issues or questions:
- Create a private security advisory on GitHub
- Contact project maintainers directly
- Follow responsible disclosure practices

## References

- [MCP Security Best Practices](https://modelcontextprotocol.io/security)
- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [GitHub API Security](https://docs.github.com/en/rest/overview/other-authentication-methods)