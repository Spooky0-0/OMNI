#!/usr/bin/env bash
set -e

echo "Starting OMNIBUS Cross-Compilation & Deployment Script..."

# Build all workspace members simultaneously
echo "[1/2] Compiling DEC, DCSE, RAFCE, and OMNIBUS..."
cargo build --release --workspace

echo "[2/2] Deployment artifacts ready in target/release/"
ls -la target/release/{omnibus,dec,dcse,rafce} 2>/dev/null || true

echo "Deployment finished."
