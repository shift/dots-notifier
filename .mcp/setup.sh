#!/bin/bash

# MCP Server Setup Script for dots-notifier

set -e

echo "Setting up MCP servers for dots-notifier repository..."

# Check if Node.js is available
if ! command -v npx &> /dev/null; then
    echo "Warning: npx not found. Please install Node.js to use MCP servers."
    echo "Visit: https://nodejs.org/"
    exit 1
fi

# Verify MCP server packages are available
echo "Checking MCP server availability..."

MCP_SERVERS=(
    "@modelcontextprotocol/server-filesystem"
    "@modelcontextprotocol/server-github"
    "@modelcontextprotocol/server-brave-search"
    "@modelcontextprotocol/server-memory"
)

for server in "${MCP_SERVERS[@]}"; do
    echo "  Checking $server..."
    if npm view "$server" version &>/dev/null; then
        echo "    ✓ Available"
    else
        echo "    ⚠ Not found in npm registry"
    fi
done

# Test filesystem server
echo "Testing filesystem MCP server..."
if npx -y @modelcontextprotocol/server-filesystem --version &>/dev/null; then
    echo "  ✓ Filesystem server working"
else
    echo "  ⚠ Filesystem server test failed"
fi

# Check environment variables
echo "Checking environment variables..."
if [ -n "$GITHUB_TOKEN" ]; then
    echo "  ✓ GITHUB_TOKEN is set"
else
    echo "  ⚠ GITHUB_TOKEN not set (required for GitHub MCP server)"
    echo "    Set with: export GITHUB_TOKEN=your_token_here"
fi

if [ -n "$BRAVE_API_KEY" ]; then
    echo "  ✓ BRAVE_API_KEY is set"
else
    echo "  ℹ BRAVE_API_KEY not set (optional for search functionality)"
fi

# Validate JSON configuration files
echo "Validating MCP configuration files..."
for config_file in .mcp/*.json .vscode/settings.json; do
    if [ -f "$config_file" ]; then
        if python3 -m json.tool "$config_file" >/dev/null 2>&1; then
            echo "  ✓ $config_file is valid"
        else
            echo "  ✗ $config_file is invalid JSON"
            exit 1
        fi
    fi
done

echo "MCP server setup validation complete!"
echo ""
echo "Configuration files created:"
echo "  .mcp/config.json          - Main MCP configuration"
echo "  .mcp/claude-desktop.json  - Claude Desktop specific"
echo "  .mcp/workspace.json       - Editor workspace configuration"
echo "  .vscode/settings.json     - VS Code MCP integration"
echo ""
echo "To use with Claude Desktop, copy .mcp/claude-desktop.json contents"
echo "to your Claude Desktop MCP configuration."
echo ""
echo "For other MCP clients, use .mcp/config.json as your base configuration."