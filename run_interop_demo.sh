#!/bin/bash
# Rust-Python CRCL Interoperability Demo
#
# This script demonstrates:
# 1. Rust generating CRCL messages (like a robot controller)
# 2. Python reading those messages (like a monitoring system)

set -e

echo "=========================================="
echo "🔄 CRCL Rust-Python Interoperability Demo"
echo "=========================================="
echo

# Clean up previous test messages
if [ -d "test_messages" ]; then
    echo "🧹 Cleaning up previous test messages..."
    rm -rf test_messages
    echo
fi

# Step 1: Generate messages with Rust
echo "Step 1: Generate CRCL messages with Rust 🦀"
echo "-------------------------------------------"
cargo run --example generate_messages --quiet
echo

# Step 2: Verify files were created
echo "Step 2: Verify generated files 📁"
echo "---------------------------------"
echo "Files in test_messages/:"
ls -lh test_messages/ | tail -n +2 | awk '{print "  " $9 " (" $5 ")"}'
echo

# Step 3: Read messages with Python
echo "Step 3: Read messages with Python 🐍"
echo "------------------------------------"
source .venv/bin/activate
python read_rust_messages.py
echo

# Step 4: Show a sample message
echo "Step 4: Sample Message Content 📋"
echo "---------------------------------"
echo "Example JSON (joint_status_1.json):"
cat test_messages/joint_status_1.json
echo
echo

# Summary
echo "=========================================="
echo "✅ Demo Complete!"
echo "=========================================="
echo
echo "What this demonstrated:"
echo "  1. Rust generated strongly-typed CRCL messages"
echo "  2. Messages were serialized to JSON and XML"
echo "  3. Python successfully read the Rust-generated messages"
echo "  4. Both languages use the same CRCL schema definitions"
echo
echo "Key Benefits:"
echo "  • Language interoperability via standard formats"
echo "  • Type safety in both languages"
echo "  • Generated from authoritative XSD source"
echo "  • No manual message construction needed"