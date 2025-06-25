# dots-notifier

A robust, native Rust application to send notifications to all active graphical users on a Linux system via D-Bus and systemd integration.

## Features

- **System-wide notifications** - Send notifications to all logged-in graphical users
- **D-Bus integration** - Uses system D-Bus for inter-process communication
- **Systemd service** - Can run as a background daemon
- **Polkit integration** - Secure permission management
- **NixOS module** - Easy system integration on NixOS
- **GitHub Copilot support** - Enhanced development experience with MCP servers

## Quick Start

### Prerequisites

- Rust toolchain (cargo, rustc)
- D-Bus development libraries
- systemd development libraries
- pkg-config

### Building

```bash
cargo build --release
```

### Usage

#### Server Mode (as systemd service)
```bash
./target/release/dots-notifier server
```

#### Client Mode (send notification)
```bash
./target/release/dots-notifier send "Title" "Message body"
```

## NixOS Integration

Add to your NixOS configuration:

```nix
{
  services.system-notifier = {
    enable = true;
    group = "wheel";  # Users in this group can send notifications
  };
}
```

## GitHub Copilot Integration

This project includes MCP (Model Context Protocol) server configuration for enhanced GitHub Copilot support.

### Setup GitHub Copilot

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Test MCP configuration:**
   ```bash
   ./scripts/test-mcp.sh
   ```

3. **Open in VS Code:**
   ```bash
   code dots-notifier.code-workspace
   ```

4. **Install VS Code extensions:**
   - GitHub Copilot
   - GitHub Copilot Chat
   - rust-analyzer
   - Nix Language Support

### MCP Servers Included

- **Filesystem Server** - Provides project structure context
- **GitHub Server** - Integrates with GitHub API for repository context
- **Rust Analyzer** - Language server for Rust-specific assistance

### Configuration Files

- `.mcp/config.json` - MCP server configuration
- `.vscode/settings.json` - VS Code workspace settings
- `package.json` - Node.js dependencies for MCP servers
- `docs/MCP_SETUP.md` - Detailed MCP documentation

## Development

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

### Nix Development Environment

```bash
nix develop
```

## Architecture

### Components

1. **Client/Server Model**
   - Server: Background daemon listening on D-Bus
   - Client: Command-line interface for sending notifications

2. **D-Bus Integration**
   - Custom interface: `me.section.Notifier`
   - System bus communication
   - Polkit policy enforcement

3. **User Discovery**
   - Queries systemd-logind for active sessions
   - Filters for graphical sessions (X11/Wayland)
   - Sends notifications to each user's session

4. **Security**
   - Group-based permissions via Polkit
   - D-Bus policy restrictions
   - Limited filesystem access

## Dependencies

### Rust Crates

- `zbus` - D-Bus communication
- `tokio` - Async runtime
- `tracing` - Structured logging
- `clap` - Command-line parsing
- `futures` - Async utilities

### System Libraries

- `dbus` - D-Bus system integration
- `systemd` - systemd API access
- `pkg-config` - Build system integration

## License

GPL-3.0-or-later

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use clippy for linting (`cargo clippy`)
- Add tests for new functionality
- Update documentation as needed

## Security

### Reporting Issues

Please report security issues privately to the maintainers.

### Security Features

- **Minimal permissions** - Only required D-Bus and filesystem access
- **Group-based authorization** - Configurable user groups
- **Sandboxed execution** - Limited system access
- **Secure defaults** - Conservative permission model

## Support

- **Issues** - Use GitHub issues for bug reports and feature requests
- **Discussions** - Use GitHub discussions for questions and community support
- **Documentation** - See `docs/` directory for detailed documentation