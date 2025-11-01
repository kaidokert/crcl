#!/usr/bin/env python3
"""
Generate Python dataclasses from CRCL XSD schemas

Python tools for XSD → Python generation:
1. generateDS - Most mature, generates classes with XML serialization
2. xsdata - Modern, generates dataclasses with multiple format support
3. PyXB - Older but comprehensive
4. xmlschema - Can parse but doesn't generate classes directly

For typed structs with JSON/XML serialization (Python's "serde"):
- pydantic - Most popular, great validation and JSON support
- attrs - Lightweight, good serialization
- dataclasses + dataclasses-json - Standard library + JSON extension
- marshmallow - Schema-based serialization
"""

import subprocess
import sys

def check_and_install_packages():
    """Check and install required packages"""
    required = {
        'xsdata[cli,lxml,soap]': 'xsdata',  # Modern XSD to Python generator
        'pydantic': 'pydantic',              # For validation and serialization
        'dataclasses-json': 'dataclasses_json'
    }

    print("Checking required packages...")
    for package, import_name in required.items():
        try:
            __import__(import_name)
            print(f"  ✓ {package} already installed")
        except ImportError:
            print(f"  Installing {package}...")
            subprocess.check_call([sys.executable, "-m", "pip", "install", package])
    print()

def main():
    print("=== Python XSD Generation Options ===\n")

    print("1. xsdata - Modern dataclass generator (RECOMMENDED)")
    print("   - Generates Python 3.7+ dataclasses")
    print("   - Supports JSON, XML, YAML serialization")
    print("   - Type hints and validation")
    print("   - Command: xsdata generate schemas/")
    print()

    print("2. generateDS - Mature, XML-focused")
    print("   - Generates traditional classes")
    print("   - Excellent XML support")
    print("   - Command: generateDS -o output.py schema.xsd")
    print()

    print("3. Custom generator using xmlschema + templates")
    print("   - Full control over output")
    print("   - Can generate pydantic models")
    print()

    choice = input("Select option (1-3) or 'q' to quit: ").strip()

    if choice == '1':
        use_xsdata()
    elif choice == '2':
        use_generateds()
    elif choice == '3':
        use_custom_generator()
    else:
        print("Exiting...")

def use_xsdata():
    """Use xsdata to generate Python dataclasses"""
    check_and_install_packages()

    print("\n=== Using xsdata to generate Python dataclasses ===\n")

    # Create xsdata config
    config = """
[tool.xsdata]
output = "dataclasses"
package = "crcl_python.models"
compound-fields = true
relative-imports = true
postponed-annotations = true
frozen = false
kw-only = false

[tool.xsdata.formats.dataclass]
eq = true
order = false
unsafe-hash = false
frozen = false
slots = false

[tool.xsdata.formats.json]
render-choice-groups = false

[tool.xsdata.naming]
class = "PascalCase"
field = "snakeCase"
module = "snakeCase"
package = "snakeCase"
"""

    with open(".xsdata.xml", "w") as f:
        f.write("""<?xml version="1.0" encoding="UTF-8"?>
<Config>
    <Output>dataclasses</Output>
    <Package>crcl_python</Package>
    <Structure>filenames</Structure>
    <CompoundFields>true</CompoundFields>
    <IncludeHeader>false</IncludeHeader>
</Config>""")

    print("Running xsdata generator...")
    try:
        # Generate from schemas
        subprocess.run([
            sys.executable, "-m", "xsdata", "generate",
            "-c", ".xsdata.xml",
            "-p", "crcl_python",
            "schemas/"
        ], check=True)
        print("✓ Successfully generated Python dataclasses in crcl_python/")

        # Show example
        print("\nExample generated code structure:")
        print("  crcl_python/")
        print("    ├── __init__.py")
        print("    ├── data_primitives.py  # Base types")
        print("    ├── crcl_commands.py    # Command types")
        print("    └── crcl_status.py      # Status types")

    except subprocess.CalledProcessError as e:
        print(f"Error running xsdata: {e}")
    except FileNotFoundError:
        print("xsdata not found. Installing...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", "xsdata[cli,lxml]"])
        print("Please run the script again.")

def use_generateds():
    """Use generateDS for XML-focused generation"""
    print("\n=== Using generateDS ===\n")
    print("Installing generateDS...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "generateDS"])

    print("\nGenerating Python classes...")
    for schema in ["DataPrimitives", "CRCLCommands", "CRCLStatus"]:
        cmd = [
            "generateDS",
            "-o", f"crcl_python_{schema.lower()}.py",
            "--super", f"crcl_python_base",
            f"schemas/{schema}.xsd"
        ]
        print(f"  Running: {' '.join(cmd)}")
        # subprocess.run(cmd)

def use_custom_generator():
    """Custom generator using xmlschema to create pydantic models"""
    print("\n=== Custom Pydantic Model Generator ===\n")

    check_and_install_packages()

    from custom_xsd_to_pydantic import generate_pydantic_models

    print("Generating pydantic models from XSD...")
    generate_pydantic_models()

if __name__ == "__main__":
    main()