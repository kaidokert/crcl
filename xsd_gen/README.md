# XSD Generator for CRCL Schemas

A Rust code generator that converts CRCL (Canonical Robot Command Language) XML Schema Definition files into type-safe Rust structs.

## Overview

This tool uses the `xsd-parser` crate to parse XSD files and generate corresponding Rust type definitions. It handles the CRCL standard schemas for robot control commands and status reporting.

## What It Does

The generator processes three main CRCL schema files:

- **DataPrimitives.xsd** - Base types including vectors, points, poses, and unit enumerations
- **CRCLCommands.xsd** - Command types like `ActuateJointsType` for robot control
- **CRCLStatus.xsd** - Status types like `JointStatusesType` for robot state reporting

The output is compilable Rust code with strongly-typed structs that can be used to work with CRCL messages programmatically.

## Generated Files

Running the generators produces:

- `generated_data_primitives.rs` - Fundamental CRCL data types
- `generated_crcl_commands_merged.rs` - Robot command message types
- `generated_crcl_status_merged.rs` - Robot status message types

All generated types include `#[derive(Debug, Serialize, Deserialize)]` for debugging and XML serialization/deserialization.

## Usage

### Generate DataPrimitives Types Only

```bash
cargo run --bin gen_data_primitives
```

Generates base types from `DataPrimitives.xsd` only.

### Generate All CRCL Types

```bash
cargo run --bin gen_crcl_types
```

Generates both command and status types by merging DataPrimitives into CRCLCommands.xsd and CRCLStatus.xsd.

## Example Usage

### Reading and Writing XML

The generated types support serde XML serialization/deserialization:

```rust
use serde_xml_rs;

// Write Rust struct to XML
let joint = JointStatusType {
    name: None,
    joint_number: 0,
    joint_position: Some(1.57),
    joint_torque_or_force: Some(10.5),
    joint_velocity: Some(0.5),
};

let xml = serde_xml_rs::to_string(&joint).unwrap();
println!("{}", xml);

// Read XML to Rust struct
let xml_input = r#"<?xml version="1.0"?>
<JointStatusType>
    <joint_number>1</joint_number>
    <joint_position>0.785</joint_position>
</JointStatusType>"#;

let parsed: JointStatusType = serde_xml_rs::from_str(xml_input).unwrap();
```

### Creating a Joint Status

```rust
use crcl_status::*;

// Clean, flat structure - no .content wrapper!
let joint0 = JointStatusType {
    name: None,  // Optional XML identifier
    joint_number: 0,
    joint_position: Some(1.57),           // radians
    joint_torque_or_force: Some(10.5),    // N or Nm
    joint_velocity: Some(0.5),            // rad/s
};

// Access fields directly
println!("Joint {}: pos={:?}", joint0.joint_number, joint0.joint_position);
```

### Creating Joint Statuses Collection

```rust
let all_joints = JointStatusesType {
    name: None,
    joint_status: vec![joint0, joint1, joint2],
};

// Access directly - no .content!
for joint in &all_joints.joint_status {
    println!("Joint {}", joint.joint_number);
}
```

## Generated Type Structure

The generator produces clean, flattened Rust structs:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct JointStatusType {
    pub name: Option<IdType>,
    pub joint_number: IntType,
    pub joint_position: Option<DoubleType>,
    pub joint_torque_or_force: Option<DoubleType>,
    pub joint_velocity: Option<DoubleType>,
}
```

The xsd-parser initially generates a two-level `Type/TypeContent` structure, but we automatically flatten it during code generation for a cleaner, more ergonomic API.

## How It Works

1. **Schema Reading** - Loads XSD files from `../schemas/` directory
2. **Schema Merging** - Manually inlines `DataPrimitives.xsd` content where included
3. **Code Generation** - Uses `xsd-parser` to generate Rust type definitions
4. **Flattening** - Removes redundant `Type/TypeContent` wrapper layers for cleaner API
5. **Formatting** - Adds newlines and serde imports for readability
6. **Output** - Writes generated Rust code to files

The manual schema merging works around xsd-parser's limitations with `<xs:include>` directives on Windows. The flattening step removes the unnecessary two-level structure that xsd-parser generates, giving you a clean API without `content` wrappers.

## Examples

See `examples/basic_usage.rs` for a comprehensive example demonstrating:
- Creating messages
- XML serialization/deserialization
- Working with collections
- Creating commands

All examples use the clean, flattened API (no `.content` wrappers).

## Testing

A test suite in `test_generated/` verifies the generated code compiles and works correctly:

```bash
cd test_generated
cargo test
```

## Type Naming

XSD types map to Rust as follows:

- `xs:int` -> `IntType` (i32)
- `xs:long` -> `LongType` (i64)
- `xs:double` -> `DoubleType` (f64)
- `xs:boolean` -> `BooleanType` (bool)
- `xs:string` -> `StringType` (String)
- `xs:ID` -> `IdType` (String)

Optional elements become `Option<T>` and unbounded elements become `Vec<T>`.

## Notes

- The `name` field appears on many types due to inheritance from `DataThingType`. It's an optional XML identifier rarely needed in practice.
- Generated code includes `Debug`, `Serialize`, and `Deserialize` derives for full XML I/O support.
- Schema files must be in `../schemas/` relative to the project directory.
- To use the generated types in your project, include the dependency: `serde-xml-rs = "0.8"`

## Dependencies

- `xsd-parser 1.3.0` - XSD to Rust code generation
- `serde 1.0` - Serialization framework
- `serde-xml-rs 0.8` - XML serialization/deserialization
- `quick-xml 0.38` - Fast XML parser with serde support
