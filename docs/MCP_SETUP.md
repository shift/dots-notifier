# MCP (Model Context Protocol) Configuration for GitHub Copilot

This document describes the MCP server configuration for enabling GitHub Copilot functionality in the `dots-notifier` project.

## Overview

The Model Context Protocol (MCP) allows GitHub Copilot to connect to various data sources and tools to provide better context-aware assistance. This configuration includes servers for:

- **Filesystem access** - Project structure and file content understanding
- **Git integration** - Version control context and history
- **Rust toolchain** - Language server integration for Rust-specific assistance

## Configuration Files

### `.mcp/config.json`
Main MCP server configuration defining:
- Filesystem server with allowed directories
- Git server for repository context
- Rust analyzer for language-specific features

### `.vscode/settings.json`
VS Code workspace settings for:
- GitHub Copilot enablement
- MCP server integration
- Rust analyzer configuration
- Nix language support

### `package.json`
Node.js dependencies for MCP servers:
- `@modelcontextprotocol/server-filesystem`
- `@modelcontextprotocol/server-git`

## Setup Instructions

### Prerequisites
- Node.js 18+ installed
- Rust toolchain with rust-analyzer
- VS Code with GitHub Copilot extension
- Git repository access

### Installation

1. **Install MCP servers:**
   ```bash
   npm install
   ```

2. **Verify Rust analyzer:**
   ```bash
   rust-analyzer --version
   ```

3. **Open in VS Code:**
   ```bash
   code dots-notifier.code-workspace
   ```

### Configuration Verification

1. **Test filesystem server:**
   ```bash
   npx @modelcontextprotocol/server-filesystem --help
   ```

2. **Test git server:**
   ```bash
   npx @modelcontextprotocol/server-git --help
   ```

3. **Check rust-analyzer:**
   ```bash
   rust-analyzer --stdio < /dev/null
   ```

## Security Considerations

### Filesystem Access
- Limited to project directory: `${workspaceFolder}`
- No access to system files outside project
- Read-only access for most operations

### Git Access
- Repository-scoped access only
- No write permissions to remote repositories
- Local repository operations only

### Network Security
- MCP servers run locally
- No external network access required
- Communication over local IPC/stdio

## Usage with GitHub Copilot

### Enabled Features
- **Code completion** with project context
- **Documentation generation** from existing patterns
- **Rust-specific suggestions** using language server
- **Git-aware refactoring** based on repository history
- **Nix configuration assistance** for system modules

### Example Copilot Prompts
- "Generate a systemd service configuration for this D-Bus service"
- "Create a Nix module option for notification group configuration"
- "Add error handling to the D-Bus proxy implementation"
- "Write tests for the notification delivery functionality"

## Troubleshooting

### MCP Server Issues
```bash
# Check server availability
npm list @modelcontextprotocol/server-filesystem
npm list @modelcontextprotocol/server-git

# Reinstall if needed
npm install --force
```

### Rust Analyzer Problems
```bash
# Check installation
which rust-analyzer
rust-analyzer --version

# Rebuild if needed
cargo check
```

### VS Code Integration
1. Reload VS Code window
2. Check GitHub Copilot status in status bar
3. Verify MCP servers in Command Palette: "MCP: Status"

## Project-Specific Context

### Rust Application
- **Crate type:** Binary application with D-Bus integration
- **Key dependencies:** zbus, tokio, tracing, clap
- **Architecture:** Client/server model with system bus communication

### Nix Configuration
- **Flake-based:** Uses modern Nix flakes for reproducible builds
- **NixOS module:** Provides system-wide notification service
- **Dependencies:** systemd, dbus, pkg-config integration

### System Integration
- **D-Bus service:** System bus integration for notifications
- **Polkit integration:** Permission management for notification sending
- **Systemd service:** Background daemon for notification handling

## Performance Tuning

### MCP Settings
- **Timeout:** 30 seconds for server responses
- **Retries:** 3 attempts for failed operations
- **Debug logging:** Disabled for production use

### Copilot Settings
- **Suggestion length:** 500 characters maximum
- **Inline suggestions:** Enabled for better UX
- **Language-specific:** Optimized for Rust and Nix

## Maintenance

### Regular Updates
```bash
# Update MCP servers
npm update

# Update Rust toolchain
rustup update

# Update VS Code extensions
code --update-extensions
```

### Health Checks
- Verify MCP server connectivity weekly
- Check Copilot suggestion quality
- Monitor performance impact on development workflow