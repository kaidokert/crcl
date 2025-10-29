#!/usr/bin/env python3
"""
Advanced XSD to JSON Schema converter using xmlschema library
Preserves more XSD semantics including restrictions, enumerations, and proper types
"""

import json
import sys
from pathlib import Path
import xmlschema
from xmlschema.validators import XsdElement, XsdComplexType, XsdSimpleType, XsdAtomicRestriction

def xsd_to_json_type(xsd_type):
    """Convert XSD type to JSON Schema type with better mapping"""
    if not xsd_type:
        return {"type": "string"}

    type_name = getattr(xsd_type, 'name', str(xsd_type))
    if isinstance(type_name, tuple):
        type_name = type_name[1] if len(type_name) > 1 else str(type_name)

    type_name = str(type_name).lower()

    # Handle restrictions and enumerations
    if isinstance(xsd_type, XsdAtomicRestriction):
        base_type = xsd_to_json_type(xsd_type.base_type)

        # Check for enumeration
        if hasattr(xsd_type, 'enumeration') and xsd_type.enumeration:
            base_type['enum'] = list(xsd_type.enumeration)

        # Check for pattern
        if hasattr(xsd_type, 'patterns') and xsd_type.patterns:
            if hasattr(xsd_type.patterns[0], 'pattern'):
                base_type['pattern'] = xsd_type.patterns[0].pattern
            else:
                base_type['pattern'] = str(xsd_type.patterns[0])

        # Check for min/max length
        if hasattr(xsd_type, 'min_length') and xsd_type.min_length is not None:
            base_type['minLength'] = xsd_type.min_length
        if hasattr(xsd_type, 'max_length') and xsd_type.max_length is not None:
            base_type['maxLength'] = xsd_type.max_length

        # Check for min/max values
        if hasattr(xsd_type, 'min_value') and xsd_type.min_value is not None:
            base_type['minimum'] = xsd_type.min_value
        if hasattr(xsd_type, 'max_value') and xsd_type.max_value is not None:
            base_type['maximum'] = xsd_type.max_value

        return base_type

    # Basic type mappings
    if 'string' in type_name or 'token' in type_name or 'normalizedstring' in type_name:
        return {"type": "string"}
    elif 'ncname' in type_name or 'name' in type_name or 'nmtoken' in type_name:
        return {"type": "string"}
    elif 'long' in type_name or 'int' in type_name or 'short' in type_name or 'byte' in type_name:
        return {"type": "integer"}
    elif 'nonpositiveinteger' in type_name:
        return {"type": "integer", "maximum": 0}
    elif 'negativeinteger' in type_name:
        return {"type": "integer", "maximum": -1}
    elif 'nonnegativeinteger' in type_name or 'unsignedlong' in type_name or 'unsignedint' in type_name:
        return {"type": "integer", "minimum": 0}
    elif 'positiveinteger' in type_name:
        return {"type": "integer", "minimum": 1}
    elif 'decimal' in type_name or 'double' in type_name or 'float' in type_name:
        return {"type": "number"}
    elif 'bool' in type_name:
        return {"type": "boolean"}
    elif 'date' in type_name:
        return {"type": "string", "format": "date"}
    elif 'datetime' in type_name:
        return {"type": "string", "format": "date-time"}
    elif 'time' in type_name:
        return {"type": "string", "format": "time"}
    elif 'anyuri' in type_name:
        return {"type": "string", "format": "uri"}
    else:
        return {"type": "string"}


def convert_element(element):
    """Convert an XSD element to JSON Schema property"""
    if not element:
        return {}

    prop = xsd_to_json_type(element.type)

    # Add description from annotations
    if hasattr(element, 'annotation') and element.annotation:
        docs = getattr(element.annotation, 'documentation', [])
        if docs and len(docs) > 0:
            prop['description'] = str(docs[0]).strip()

    # Handle arrays (maxOccurs > 1)
    if hasattr(element, 'max_occurs') and element.max_occurs and element.max_occurs != 1:
        if element.max_occurs == 'unbounded' or element.max_occurs > 1:
            prop = {
                "type": "array",
                "items": prop
            }
            if hasattr(element, 'min_occurs') and element.min_occurs is not None:
                prop['minItems'] = element.min_occurs
            if element.max_occurs != 'unbounded':
                prop['maxItems'] = element.max_occurs

    # Handle nullable
    if hasattr(element, 'nillable') and element.nillable:
        if 'type' in prop:
            prop['type'] = [prop['type'], 'null']

    # Handle default values
    if hasattr(element, 'default') and element.default is not None:
        prop['default'] = element.default

    return prop


def convert_complex_type(xsd_type, schema):
    """Convert an XSD complex type to JSON Schema definition"""
    definition = {
        "type": "object",
        "properties": {},
        "required": []
    }

    # Add description from annotations
    if hasattr(xsd_type, 'annotation') and xsd_type.annotation:
        docs = getattr(xsd_type.annotation, 'documentation', [])
        if docs and len(docs) > 0:
            definition['description'] = str(docs[0]).strip()

    # Process attributes
    if hasattr(xsd_type, 'attributes'):
        for attr_name, attr in xsd_type.attributes.items():
            clean_name = str(attr_name).split('}')[-1]
            if clean_name:
                definition["properties"][clean_name] = xsd_to_json_type(attr.type)
                if hasattr(attr, 'use') and attr.use == 'required':
                    definition["required"].append(clean_name)

    # Process elements
    if hasattr(xsd_type, 'content'):
        if hasattr(xsd_type.content, 'iter_elements'):
            for elem in xsd_type.content.iter_elements():
                if elem.name:
                    elem_name = str(elem.name).split('}')[-1]
                    definition["properties"][elem_name] = convert_element(elem)

                    # Check if required (minOccurs > 0)
                    if hasattr(elem, 'min_occurs') and elem.min_occurs and elem.min_occurs > 0:
                        definition["required"].append(elem_name)

    # Clean up empty required array
    if not definition["required"]:
        del definition["required"]

    return definition


def convert_xsd_file(xsd_path, output_dir="jsonschemas"):
    """Convert XSD file to JSON Schema with advanced features"""
    xsd_path = Path(xsd_path)
    output_dir = Path(output_dir)
    output_dir.mkdir(exist_ok=True)

    print(f"\nConverting {xsd_path.name}...")

    try:
        # Load XSD schema
        schema = xmlschema.XMLSchema(str(xsd_path))

        # Create JSON Schema
        json_schema = {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": f"https://example.com/schemas/{xsd_path.stem}.json",
            "title": xsd_path.stem,
            "description": f"JSON Schema converted from {xsd_path.name}",
            "type": "object",
            "definitions": {}
        }

        # Convert all types to definitions
        for type_name, xsd_type in schema.types.items():
            clean_name = str(type_name).split('}')[-1]
            if clean_name and clean_name != xsd_path.stem:
                if isinstance(xsd_type, XsdComplexType):
                    json_schema["definitions"][clean_name] = convert_complex_type(xsd_type, schema)
                elif isinstance(xsd_type, (XsdSimpleType, XsdAtomicRestriction)):
                    json_schema["definitions"][clean_name] = xsd_to_json_type(xsd_type)

        # Convert root elements
        if schema.elements:
            json_schema["properties"] = {}
            for elem_name, elem in schema.elements.items():
                clean_name = str(elem_name).split('}')[-1]
                if elem.type and hasattr(elem.type, 'name'):
                    type_ref = str(elem.type.name).split('}')[-1]
                    if type_ref in json_schema["definitions"]:
                        json_schema["properties"][clean_name] = {
                            "$ref": f"#/definitions/{type_ref}"
                        }
                    else:
                        json_schema["properties"][clean_name] = convert_element(elem)
                else:
                    json_schema["properties"][clean_name] = convert_element(elem)

        # If no root elements but has types, make the first complex type the root
        if not json_schema.get("properties") and json_schema["definitions"]:
            # Find the main type (usually matches filename or has "Type" suffix)
            main_type = None
            for type_name in json_schema["definitions"]:
                if xsd_path.stem in type_name or type_name.endswith("Type"):
                    main_type = type_name
                    break

            if main_type:
                json_schema = {
                    **json_schema,
                    **json_schema["definitions"][main_type],
                    "definitions": {k: v for k, v in json_schema["definitions"].items() if k != main_type}
                }

        # Save JSON Schema
        output_file = output_dir / f"{xsd_path.stem}.json"
        with open(output_file, 'w') as f:
            json.dump(json_schema, f, indent=2, ensure_ascii=False)

        print(f"  ✓ Saved to {output_file}")
        print(f"  - Definitions: {len(json_schema.get('definitions', {}))}")
        print(f"  - Properties: {len(json_schema.get('properties', {}))}")

        return True, output_file

    except Exception as e:
        print(f"  ✗ Error: {e}")
        import traceback
        traceback.print_exc()
        return False, None


def main():
    """Convert all XSD files in schemas directory"""
    schemas_dir = Path("schemas")

    if not schemas_dir.exists():
        print("Error: schemas directory not found!")
        sys.exit(1)

    xsd_files = list(schemas_dir.glob("*.xsd"))
    print(f"Found {len(xsd_files)} XSD files")

    successful = []
    failed = []

    for xsd_file in xsd_files:
        success, output_path = convert_xsd_file(xsd_file)
        if success:
            successful.append((xsd_file, output_path))
        else:
            failed.append(xsd_file)

    print("\n" + "="*60)
    print("Conversion Summary:")
    print(f"  Successful: {len(successful)}")
    print(f"  Failed: {len(failed)}")

    if successful:
        print("\n✓ Successfully converted:")
        for xsd, json_path in successful:
            print(f"  - {xsd.name} → {json_path.name}")


if __name__ == "__main__":
    main()