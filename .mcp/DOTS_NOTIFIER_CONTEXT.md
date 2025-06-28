# MCP Server Configuration for dots-notifier

This repository includes a complete Model Context Protocol (MCP) server configuration to enhance AI assistant capabilities when working with the dots-notifier codebase.

## Project Overview

dots-notifier is a Rust-based system notification service that:
- Uses D-Bus for system-wide communication
- Sends notifications to all active graphical users
- Integrates with systemd and login manager
- Includes Nix packaging and configuration
- Provides both server and client modes

## MCP Integration Benefits

With the configured MCP servers, AI assistants can:

### Code Understanding & Development
- **Filesystem access**: Browse and analyze Rust source code, Nix modules, and configuration files
- **Repository context**: Understand the project structure and dependencies
- **Build system awareness**: Work with Cargo.toml, flake.nix, and build configurations

### Repository Management 
- **GitHub integration**: Access issues, pull requests, and repository metadata
- **Version control**: Understand git history and branch structure
- **Documentation**: Access README files, comments, and inline documentation

### System Integration Context
- **D-Bus knowledge**: Understand the notification service architecture
- **Nix ecosystem**: Work with the NixOS module and packaging
- **systemd integration**: Understand service configuration and lifecycle
- **Linux desktop integration**: Context about notification systems and user sessions

## Configuration Files

### `.mcp/config.json`
Main configuration supporting:
- Filesystem exploration with appropriate directory restrictions
- GitHub repository access for issues and PRs
- Optional web search for documentation and troubleshooting
- Persistent memory for conversation context

### `.mcp/claude-desktop.json`
Optimized for Claude Desktop with:
- Repository-specific server naming
- Focused directory access (src/, modules/, root)
- GitHub integration with repository details

### `.mcp/workspace.json`
Editor-agnostic workspace configuration with:
- Dynamic workspace folder resolution
- Project metadata including frameworks and system integration details
- Context about Rust ecosystem (tokio, zbus, tracing, clap)

### `.vscode/settings.json`
VS Code specific configuration with:
- MCP server integration
- Rust analyzer settings optimized for this project
- File associations and search exclusions

## Setup Instructions

1. **Prerequisites**: Ensure Node.js and npm are installed
2. **Run setup**: Execute `.mcp/setup.sh` to validate the configuration
3. **Environment variables**: Set `GITHUB_TOKEN` for GitHub integration
4. **Client configuration**: Copy the appropriate config to your MCP client

## Repository-Specific Context

The MCP configuration is tailored for dots-notifier's specific needs:

### Rust Development
- Access to `src/main.rs` with D-Bus interface implementations
- Understanding of async/await patterns with tokio
- zbus library usage for D-Bus communication
- tracing integration for logging and debugging

### System Integration
- D-Bus service configuration in `modules/notifier.nix`
- systemd service setup and polkit rules
- User session detection and notification delivery
- Cross-desktop compatibility considerations

### Nix Ecosystem
- Flake configuration with Rust overlay
- NixOS module with configurable options
- Build dependencies and system library integration
- Development shell configuration

This MCP setup enables AI assistants to provide contextual help with:
- Debugging D-Bus communication issues
- Understanding Rust async patterns
- Working with Nix packaging and modules
- Troubleshooting system integration problems
- Implementing new notification features
- Maintaining cross-platform compatibility