# dots-notifier

A client/server tool to send notifications to all active graphical users on a system.

## Overview

This tool consists of two main components:
- **dots-notifier**: The main server/client binary that runs as a system service
- **dots-notifier-helper**: A user-space helper that actually sends notifications to user sessions

The helper process solves the "Broken pipe" issue that occurs when the system service tries to directly access user session D-Bus connections.

## Installation

### Building from source

```bash
cargo build --release
```

This creates two binaries:
- `target/release/dots-notifier` - Main server/client
- `target/release/dots-notifier-helper` - User-space helper

### Installing binaries

Copy the binaries to a location in your PATH:

```bash
sudo cp target/release/dots-notifier /usr/local/bin/
sudo cp target/release/dots-notifier-helper /usr/local/bin/
```

### Installing systemd user service (Optional)

For users who want to use the systemd integration:

```bash
# Install the user service template
sudo cp systemd/dots-notifier-helper@.service /usr/lib/systemd/user/

# Reload systemd user daemon (run as each user)
systemctl --user daemon-reload
```

## Usage

### Running the server

Start the main notification server (typically as a system service):

```bash
dots-notifier server
```

### Sending notifications

Send a notification to all active graphical users:

```bash
dots-notifier send "Alert Title" "Alert message body"
```

### Direct helper usage

The helper can also be used directly by users:

```bash
dots-notifier-helper "Test Title" "Test message"
```

## Architecture

1. **Main Server**: `dots-notifier server` runs as a system service and listens for D-Bus requests
2. **Client**: `dots-notifier send` connects to the system service to request notifications
3. **Helper Process**: For each active user, the server invokes `dots-notifier-helper` as that user
4. **User Session**: The helper runs with user permissions and can access the user's session D-Bus

This architecture ensures reliable notification delivery while avoiding permission issues.

## System Requirements

- Linux system with systemd and D-Bus
- Active graphical user sessions (X11 or Wayland)
- `sudo` or `systemd-run` available for user process invocation

## Security Notes

- The main server runs with elevated privileges to detect active user sessions
- Helper processes run with user privileges and only access that user's session
- The system uses either `systemd-run --user` or `sudo -u <user>` for secure user context switching