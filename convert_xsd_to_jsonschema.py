#!/usr/bin/env python3
"""
Convert XSD schemas to JSON Schema format
"""

import json
import os
from pathlib import Path
import xmlschema
import sys

def get_json_type(xsd_type):
    """Map XSD types to JSON Schema types"""
    if not xsd_type:
        return "string"

    type_name = str(xsd_type).lower()

    # Basic type mappings
    if 'string' in type_name or 'token' in type_name or 'name' in type_name:
        return "string"
    elif 'int' in type_name or 'long' in type_name or 'short' in type_name:
        return "integer"
    elif 'decimal' in type_name or 'double' in type_name or 'float' in type_name:
        return "number"
    elif 'bool' in type_name:
        return "boolean"
    elif 'date' in type_name or 'time' in type_name:
        return "string"
    else:
        return "string"  # Default fallback

def convert_xsd_to_jsonschema(xsd_path, output_dir="jsonschemas"):
    """
    Convert a single XSD file to JSON Schema

    Args:
        xsd_path: Path to XSD file
        output_dir: Directory to save JSON schemas
    """
    xsd_path = Path(xsd_path)
    output_dir = Path(output_dir)
    output_dir.mkdir(exist_ok=True)

    print(f"\nConverting {xsd_path.name}...")

    try:
        # Load the XSD schema
        schema = xmlschema.XMLSchema(str(xsd_path))

        # Create a JSON Schema representation
        json_schema = {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": xsd_path.stem,
            "type": "object",
            "definitions": {},
            "properties": {}
        }

        # Convert complex types
        for type_name, xsd_type in schema.types.items():
            if hasattr(xsd_type, 'attributes'):
                definition = {
                    "type": "object",
                    "properties": {}
                }

                # Add attributes
                if hasattr(xsd_type, 'attributes'):
                    for attr_name, attr in xsd_type.attributes.items():
                        if attr_name and hasattr(attr, 'type'):
                            definition["properties"][str(attr_name).split('}')[-1]] = {
                                "type": get_json_type(attr.type)
                            }

                # Add elements from content
                if hasattr(xsd_type, 'content') and hasattr(xsd_type.content, 'iter_elements'):
                    for elem in xsd_type.content.iter_elements():
                        if elem.name:
                            elem_name = str(elem.name).split('}')[-1]
                            definition["properties"][elem_name] = {
                                "type": get_json_type(elem.type)
                            }
                            if elem.min_occurs == 0:
                                definition.setdefault("required", [])
                            else:
                                definition.setdefault("required", []).append(elem_name)

                type_name_clean = str(type_name).split('}')[-1]
                if type_name_clean:
                    json_schema["definitions"][type_name_clean] = definition

        # Convert root elements
        for elem in schema.elements.values():
            elem_name = str(elem.name).split('}')[-1]
            if elem.type and hasattr(elem.type, 'name'):
                type_ref = str(elem.type.name).split('}')[-1]
                json_schema["properties"][elem_name] = {
                    "$ref": f"#/definitions/{type_ref}"
                }
            else:
                json_schema["properties"][elem_name] = {
                    "type": get_json_type(elem.type)
                }

        # Output file path
        output_file = output_dir / f"{xsd_path.stem}.json"

        # Save the JSON Schema with pretty formatting
        with open(output_file, 'w') as f:
            json.dump(json_schema, f, indent=2)

        print(f"  ✓ Saved to {output_file}")

        # Print some statistics
        print(f"  - Root elements: {len(schema.elements)}")
        print(f"  - Complex types: {len([t for t in schema.types.values() if hasattr(t, 'attributes')])}")
        print(f"  - Total types: {len(schema.types)}")

        return True, output_file

    except Exception as e:
        print(f"  ✗ Error: {e}")
        return False, None

def main():
    """Convert all XSD files in schemas directory"""

    schemas_dir = Path("schemas")

    if not schemas_dir.exists():
        print("Error: schemas directory not found!")
        sys.exit(1)

    # Find all XSD files
    xsd_files = list(schemas_dir.glob("*.xsd"))

    if not xsd_files:
        print("No XSD files found in schemas directory!")
        sys.exit(1)

    print(f"Found {len(xsd_files)} XSD files to convert")

    # Track results
    successful = []
    failed = []

    # Convert each file
    for xsd_file in xsd_files:
        success, output_path = convert_xsd_to_jsonschema(xsd_file)
        if success:
            successful.append((xsd_file, output_path))
        else:
            failed.append(xsd_file)

    # Summary
    print("\n" + "="*50)
    print(f"Conversion Summary:")
    print(f"  Successful: {len(successful)}")
    print(f"  Failed: {len(failed)}")

    if failed:
        print("\nFailed conversions:")
        for f in failed:
            print(f"  - {f.name}")

    if successful:
        print("\nSuccessfully converted:")
        for xsd, json_path in successful:
            print(f"  - {xsd.name} → {json_path.name}")

            # Show a sample of the converted JSON Schema
            with open(json_path, 'r') as f:
                sample = json.load(f)
                print(f"    Schema type: {sample.get('type', 'N/A')}")
                if 'properties' in sample:
                    props = list(sample['properties'].keys())[:3]
                    print(f"    Sample properties: {', '.join(props)}{'...' if len(sample['properties']) > 3 else ''}")

if __name__ == "__main__":
    main()