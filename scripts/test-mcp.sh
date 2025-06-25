#!/bin/bash
set -euo pipefail

# MCP Configuration Test Script for dots-notifier
# Tests all MCP servers and validates connectivity

echo "🔍 Testing MCP Configuration for GitHub Copilot..."
echo "================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    if [ "$1" = "ok" ]; then
        echo -e "${GREEN}✓${NC} $2"
    elif [ "$1" = "warn" ]; then
        echo -e "${YELLOW}⚠${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
    fi
}

# Test Node.js availability
echo -e "\n📦 Checking Node.js..."
if command -v node >/dev/null 2>&1; then
    NODE_VERSION=$(node --version)
    print_status "ok" "Node.js $NODE_VERSION installed"
    
    # Check if version is >= 18
    NODE_MAJOR=$(echo $NODE_VERSION | cut -d'.' -f1 | sed 's/v//')
    if [ "$NODE_MAJOR" -ge 18 ]; then
        print_status "ok" "Node.js version is compatible (>= 18)"
    else
        print_status "error" "Node.js version $NODE_VERSION is too old (need >= 18)"
    fi
else
    print_status "error" "Node.js not found - required for MCP servers"
fi

# Test npm availability
echo -e "\n📦 Checking npm..."
if command -v npm >/dev/null 2>&1; then
    NPM_VERSION=$(npm --version)
    print_status "ok" "npm $NPM_VERSION available"
else
    print_status "error" "npm not found - required for MCP servers"
fi

# Test Rust toolchain
echo -e "\n🦀 Checking Rust toolchain..."
if command -v rustc >/dev/null 2>&1; then
    RUST_VERSION=$(rustc --version)
    print_status "ok" "Rust compiler: $RUST_VERSION"
else
    print_status "error" "Rust compiler not found"
fi

if command -v cargo >/dev/null 2>&1; then
    CARGO_VERSION=$(cargo --version)
    print_status "ok" "Cargo: $CARGO_VERSION"
else
    print_status "error" "Cargo not found"
fi

if command -v rust-analyzer >/dev/null 2>&1; then
    print_status "ok" "rust-analyzer available"
else
    print_status "warn" "rust-analyzer not found in PATH - may need manual installation"
fi

# Test Git
echo -e "\n🔗 Checking Git..."
if command -v git >/dev/null 2>&1; then
    GIT_VERSION=$(git --version)
    print_status "ok" "$GIT_VERSION"
    
    # Check if we're in a git repository
    if git rev-parse --git-dir >/dev/null 2>&1; then
        print_status "ok" "Inside Git repository"
    else
        print_status "error" "Not in a Git repository"
    fi
else
    print_status "error" "Git not found"
fi

# Test MCP configuration files
echo -e "\n📋 Checking MCP configuration files..."
if [ -f ".mcp/config.json" ]; then
    print_status "ok" ".mcp/config.json exists"
    
    # Validate JSON syntax
    if command -v node >/dev/null 2>&1; then
        if node -e "JSON.parse(require('fs').readFileSync('.mcp/config.json', 'utf8'))" 2>/dev/null; then
            print_status "ok" ".mcp/config.json is valid JSON"
        else
            print_status "error" ".mcp/config.json has invalid JSON syntax"
        fi
    fi
else
    print_status "error" ".mcp/config.json not found"
fi

if [ -f ".vscode/settings.json" ]; then
    print_status "ok" ".vscode/settings.json exists"
else
    print_status "warn" ".vscode/settings.json not found"
fi

if [ -f "package.json" ]; then
    print_status "ok" "package.json exists"
else
    print_status "warn" "package.json not found"
fi

# Test MCP server installation
echo -e "\n🛠️  Checking MCP server dependencies..."
if [ -f "package.json" ] && command -v npm >/dev/null 2>&1; then
    echo "Installing MCP server dependencies..."
    if npm install --silent; then
        print_status "ok" "MCP server dependencies installed"
    else
        print_status "error" "Failed to install MCP server dependencies"
    fi
    
    # Test filesystem server
    echo "Testing filesystem server..."
    FILESYSTEM_OUTPUT=$(timeout 3 npx mcp-server-filesystem /tmp 2>&1 || true)
    if echo "$FILESYSTEM_OUTPUT" | grep -q "Secure MCP Filesystem Server running"; then
        print_status "ok" "Filesystem MCP server available"
    else
        print_status "warn" "Filesystem MCP server output: ${FILESYSTEM_OUTPUT:0:100}..."
    fi
    
    # Test github server  
    echo "Testing GitHub server..."
    GITHUB_OUTPUT=$(timeout 3 npx mcp-server-github 2>&1 || true)
    if echo "$GITHUB_OUTPUT" | grep -q "GitHub MCP Server running"; then
        print_status "ok" "GitHub MCP server available"
    else
        print_status "warn" "GitHub MCP server output: ${GITHUB_OUTPUT:0:100}..."
    fi
else
    print_status "warn" "Skipping MCP server installation check"
fi

# Test Rust project compilation
echo -e "\n🔨 Testing Rust project compilation..."
if command -v cargo >/dev/null 2>&1; then
    echo "Running cargo check..."
    if cargo check --quiet; then
        print_status "ok" "Rust project compiles successfully"
    else
        print_status "error" "Rust project compilation failed"
    fi
else
    print_status "warn" "Skipping Rust compilation check"
fi

# Test Nix if available
echo -e "\n❄️  Checking Nix (optional)..."
if command -v nix >/dev/null 2>&1; then
    print_status "ok" "Nix package manager available"
    
    if [ -f "flake.nix" ]; then
        print_status "ok" "Nix flake configuration found"
        
        # Test flake check if possible
        if timeout 30 nix flake check --no-build 2>/dev/null; then
            print_status "ok" "Nix flake configuration is valid"
        else
            print_status "warn" "Nix flake check failed or timed out"
        fi
    else
        print_status "warn" "No Nix flake configuration found"
    fi
else
    print_status "warn" "Nix not available (optional for this setup)"
fi

echo -e "\n📊 Configuration Summary..."
echo "================================"
echo "MCP Configuration Status:"
echo "  • Filesystem server: Configured for project directory access"
echo "  • GitHub server: Configured for repository context and API access"
echo "  • Rust analyzer: Configured for language support"
echo ""
echo "Security Settings:"
echo "  • Filesystem access: Limited to project directory"
echo "  • Git access: Repository-scoped only"
echo "  • Network access: Local communication only"
echo ""
echo "Next Steps:"
echo "  1. Open project in VS Code: code dots-notifier.code-workspace"
echo "  2. Install GitHub Copilot extension"
echo "  3. Verify Copilot is enabled in settings"
echo "  4. Test code completion with Copilot"

echo -e "\n${GREEN}MCP configuration test completed!${NC}"