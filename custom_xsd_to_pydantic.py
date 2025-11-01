#!/usr/bin/env python3
"""
Custom XSD to Pydantic model generator

Pydantic is Python's closest equivalent to Rust's serde:
- Automatic JSON/dict serialization
- Type validation
- IDE support with type hints
- Fast (uses Rust under the hood in v2!)
"""

import xmlschema
from pathlib import Path
from typing import Dict, List, Optional, Any
import re

def xsd_type_to_python(xsd_type_name: str) -> str:
    """Map XSD types to Python/Pydantic types"""
    type_map = {
        'string': 'str',
        'boolean': 'bool',
        'int': 'int',
        'integer': 'int',
        'long': 'int',
        'double': 'float',
        'decimal': 'float',
        'float': 'float',
        'dateTime': 'datetime',
        'date': 'date',
        'token': 'str',
        'NCName': 'str',
        'anyURI': 'str',
    }

    # Extract base type name
    if '{' in xsd_type_name:
        xsd_type_name = xsd_type_name.split('}')[-1]

    # Check for xs: prefix
    if ':' in xsd_type_name:
        xsd_type_name = xsd_type_name.split(':')[-1]

    return type_map.get(xsd_type_name, 'Any')

def clean_name(name: str) -> str:
    """Clean XML names for Python"""
    if '{' in name:
        name = name.split('}')[-1]
    # Convert to snake_case
    name = re.sub('([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    name = re.sub('([a-z\d])([A-Z])', r'\1_\2', name)
    return name.lower()

def generate_pydantic_class(type_name: str, xsd_type, schema) -> str:
    """Generate a Pydantic model from an XSD complex type"""
    clean_type_name = type_name.split('}')[-1] if '}' in type_name else type_name

    lines = [f"class {clean_type_name}(BaseModel):"]
    lines.append(f'    """Generated from XSD type {clean_type_name}"""')

    # Handle attributes
    if hasattr(xsd_type, 'attributes'):
        for attr_name, attr in xsd_type.attributes.items():
            field_name = clean_name(str(attr_name))
            field_type = xsd_type_to_python(str(attr.type) if attr.type else 'string')
            if hasattr(attr, 'use') and attr.use != 'required':
                field_type = f"Optional[{field_type}]"
                lines.append(f"    {field_name}: {field_type} = None")
            else:
                lines.append(f"    {field_name}: {field_type}")

    # Handle elements
    if hasattr(xsd_type, 'content') and hasattr(xsd_type.content, 'iter_elements'):
        for elem in xsd_type.content.iter_elements():
            if elem.name:
                field_name = clean_name(str(elem.name))

                # Determine type
                if elem.type and hasattr(elem.type, 'name'):
                    type_name = str(elem.type.name).split('}')[-1]
                    field_type = type_name
                else:
                    field_type = xsd_type_to_python(str(elem.type) if elem.type else 'string')

                # Handle arrays
                if hasattr(elem, 'max_occurs'):
                    if elem.max_occurs == 'unbounded' or (isinstance(elem.max_occurs, int) and elem.max_occurs > 1):
                        field_type = f"List[{field_type}]"

                # Handle optional
                if hasattr(elem, 'min_occurs') and elem.min_occurs == 0:
                    field_type = f"Optional[{field_type}]"
                    lines.append(f"    {field_name}: {field_type} = None")
                else:
                    lines.append(f"    {field_name}: {field_type}")

    # If no fields, add pass
    if len(lines) == 2:
        lines.append("    pass")

    # Add model config
    lines.append("")
    lines.append("    model_config = ConfigDict(")
    lines.append("        populate_by_name=True,")
    lines.append("        use_enum_values=True,")
    lines.append("        arbitrary_types_allowed=True,")
    lines.append("    )")

    return '\n'.join(lines)

def generate_pydantic_models():
    """Generate Pydantic models from CRCL XSD schemas"""

    schemas_dir = Path("schemas")
    output_dir = Path("crcl_python")
    output_dir.mkdir(exist_ok=True)

    # Process each schema
    for schema_file in ["DataPrimitives.xsd", "CRCLStatus.xsd", "CRCLCommands.xsd"]:
        schema_path = schemas_dir / schema_file
        module_name = schema_file.replace('.xsd', '').lower()

        print(f"\nProcessing {schema_file}...")

        try:
            schema = xmlschema.XMLSchema(str(schema_path))

            output_lines = [
                '"""',
                f'Pydantic models generated from {schema_file}',
                '"""',
                '',
                'from typing import List, Optional, Any, Union',
                'from datetime import datetime, date',
                'from pydantic import BaseModel, Field, ConfigDict',
                '',
                ''
            ]

            # Generate enums first
            enums_generated = []
            for type_name, xsd_type in schema.types.items():
                clean_type_name = type_name.split('}')[-1] if '}' in type_name else type_name

                if hasattr(xsd_type, 'enumeration') and xsd_type.enumeration:
                    print(f"  - Generating enum: {clean_type_name}")
                    output_lines.append(f"class {clean_type_name}(str):")
                    output_lines.append(f'    """Enumeration type"""')
                    for value in xsd_type.enumeration:
                        const_name = str(value).upper().replace('-', '_').replace(' ', '_')
                        output_lines.append(f'    {const_name} = "{value}"')
                    output_lines.append('')
                    enums_generated.append(clean_type_name)

            # Generate simple types
            for type_name, xsd_type in schema.types.items():
                clean_type_name = type_name.split('}')[-1] if '}' in type_name else type_name

                if clean_type_name not in enums_generated:
                    if hasattr(xsd_type, 'python_type'):
                        # Simple type alias
                        python_type = xsd_type_to_python(str(xsd_type.base_type) if hasattr(xsd_type, 'base_type') else 'string')
                        output_lines.append(f"{clean_type_name} = {python_type}")
                        output_lines.append('')

            # Generate complex types
            for type_name, xsd_type in schema.types.items():
                clean_type_name = type_name.split('}')[-1] if '}' in type_name else type_name

                if hasattr(xsd_type, 'content') or hasattr(xsd_type, 'attributes'):
                    if clean_type_name not in enums_generated:
                        print(f"  - Generating class: {clean_type_name}")
                        class_code = generate_pydantic_class(type_name, xsd_type, schema)
                        output_lines.append(class_code)
                        output_lines.append('')
                        output_lines.append('')

            # Write output
            output_file = output_dir / f"{module_name}.py"
            with open(output_file, 'w') as f:
                f.write('\n'.join(output_lines))

            print(f"  ✓ Generated {output_file}")

        except Exception as e:
            print(f"  ✗ Error processing {schema_file}: {e}")

    # Create __init__.py
    init_file = output_dir / "__init__.py"
    with open(init_file, 'w') as f:
        f.write('"""CRCL Python models generated from XSD"""\n\n')
        f.write('from .dataprimitives import *\n')
        f.write('from .crclstatus import *\n')
        f.write('from .crclcommands import *\n')

    print(f"\n✓ Generated Python models in {output_dir}/")
    print("\nExample usage:")
    print("  from crcl_python import JointStatusType")
    print("  joint = JointStatusType(joint_number=1, joint_position=45.5)")
    print("  json_data = joint.model_dump_json()")

if __name__ == "__main__":
    generate_pydantic_models()