#!/bin/bash
# Useful jq queries for CRCL JSON schemas

echo "=== JQ Queries for CRCL JSON Schemas ==="
echo

echo "1. Get all message type names from CRCLStatus.json:"
echo "   jq '.definitions | keys' jsonschemas/CRCLStatus.json"
echo

echo "2. Get only types ending with 'Type' (main message types):"
echo "   jq '.definitions | keys | map(select(endswith(\"Type\")))' jsonschemas/CRCLStatus.json"
echo

echo "3. Get only status-related message types:"
echo "   jq '.definitions | keys | map(select(contains(\"Status\")))' jsonschemas/CRCLStatus.json"
echo

echo "4. Get properties of a specific type (e.g., JointStatusType):"
echo "   jq '.definitions.JointStatusType.properties | keys' jsonschemas/CRCLStatus.json"
echo

echo "5. Get all types with their property counts:"
echo "   jq '.definitions | to_entries | map({name: .key, property_count: (.value.properties | length)})' jsonschemas/CRCLStatus.json"
echo

echo "6. Find all enum types:"
echo "   jq '.definitions | to_entries | map(select(.value.enum)) | map(.key)' jsonschemas/CRCLStatus.json"
echo

echo "7. Get required fields for each type:"
echo "   jq '.definitions | to_entries | map({type: .key, required: .value.required}) | map(select(.required))' jsonschemas/CRCLStatus.json"
echo

echo "8. Get all types that reference other types (have \$ref):"
echo "   jq '[.. | objects | select(has(\"\$ref\")) | .\"\$ref\"] | unique' jsonschemas/CRCLStatus.json"
echo

echo "9. Get a compact list of just the type names (one per line):"
echo "   jq -r '.definitions | keys[]' jsonschemas/CRCLStatus.json"
echo

echo "10. Count total number of message types:"
echo "    jq '.definitions | length' jsonschemas/CRCLStatus.json"