#!/bin/bash

# Install tarpaulin if not already installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin
fi

echo "Running code coverage analysis..."

# Run coverage analysis
cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out xml --output-dir coverage

echo "Coverage report generated in coverage/ directory"

# Also generate a simple text summary
cargo tarpaulin --verbose --all-features --workspace --timeout 120