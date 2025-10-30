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

All generated types include `#[derive(Debug)]` for easy debugging.

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

### Creating a Joint Status

```rust
use crcl_status::*;

let joint0 = JointStatusType {
    content: JointStatusTypeContent {
        name: None,  // Optional XML identifier
        joint_number: 0,
        joint_position: Some(1.57),           // radians
        joint_torque_or_force: Some(10.5),    // N or Nm
        joint_velocity: Some(0.5),            // rad/s
    }
};
```

### Creating Joint Statuses Collection

```rust
let all_joints = JointStatusesType {
    content: JointStatusesTypeContent {
        name: None,
        joint_status: vec![joint0, joint1, joint2],
    }
};
```

## Generated Type Structure

The generator produces a two-level structure for each XSD complex type:

```rust
// Wrapper type
pub struct JointStatusType {
    pub content: JointStatusTypeContent
}

// Content with actual fields
pub struct JointStatusTypeContent {
    pub name: Option<IdType>,
    pub joint_number: IntType,
    pub joint_position: Option<DoubleType>,
    pub joint_torque_or_force: Option<DoubleType>,
    pub joint_velocity: Option<DoubleType>,
}
```

This mirrors the XSD structure where elements have both type information and content.

## How It Works

1. **Schema Reading** - Loads XSD files from `../schemas/` directory
2. **Schema Merging** - Manually inlines `DataPrimitives.xsd` content where included
3. **Code Generation** - Uses `xsd-parser` to generate Rust type definitions
4. **Formatting** - Adds newlines for readability
5. **Output** - Writes generated Rust code to files

The manual schema merging works around xsd-parser's limitations with `<xs:include>` directives on Windows.

## Testing

A test suite verifies the generated code compiles and can be instantiated:

```bash
cd test_generated
cargo test
```

See `examples/joint_status_example.rs` for a complete working example.

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
- Generated code includes only `Debug` derive. Serde serialization support requires additional configuration.
- Schema files must be in `../schemas/` relative to the project directory.

## Dependencies

- `xsd-parser 1.3.0` - XSD to Rust code generation
- `serde 1.0` - Added as dependency but not yet configured in generated code
