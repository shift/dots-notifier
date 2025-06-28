# MCP Server Configuration

This directory contains configuration files for Model Context Protocol (MCP) servers that enhance AI assistant capabilities when working with the dots-notifier repository.

## Configured MCP Servers

### Core Development Servers

- **filesystem**: Provides file system access for code exploration and editing
- **git**: Enables Git repository management and history access
- **github**: Integrates with GitHub for issues, PRs, and repository management
- **shell**: Allows execution of shell commands for building, testing, and system operations
- **sqlite**: Provides database access capabilities for potential future features

### Additional Servers

- **brave-search**: Web search capabilities for documentation and troubleshooting
- **memory**: Persistent memory for maintaining context across conversations

## Usage

The MCP configuration is automatically detected by compatible AI assistants and IDEs that support the Model Context Protocol.

### Environment Variables

Some servers require environment variables to be set:

- `GITHUB_TOKEN`: GitHub personal access token for GitHub server
- `BRAVE_API_KEY`: Brave Search API key for search server (optional)

### Repository-Specific Context

The filesystem and git servers are pre-configured to work with this dots-notifier repository, providing:

- Access to Rust source code in `src/`
- Nix configuration in `modules/`
- Build system files (Cargo.toml, flake.nix)
- Git history and branch management

This configuration enables AI assistants to:
- Understand the D-Bus notification system architecture
- Help with Rust development and debugging
- Assist with Nix module configuration
- Manage Git workflows and GitHub integration
- Execute build and test commands
- Search for relevant documentation and solutions