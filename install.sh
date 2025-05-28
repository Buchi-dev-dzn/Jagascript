#!/bin/sh

set -e

echo "🔧 Building jagascript in release mode..."
cargo build --release

echo "📦 Installing to ~/.cargo/bin (make sure this path is in your \$PATH)..."
mkdir -p ~/.cargo/bin
cp target/release/jagascript ~/.cargo/bin/jaga

echo "✅ Installation complete! Try running:"
echo ""
echo "    jaga examples/hello.jgs"