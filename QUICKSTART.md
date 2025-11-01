# Quick Start: Using CRCL in Your Project

This guide shows you how to use the CRCL library in your Rust project.

## Step 1: Add the Dependency

Edit your project's `Cargo.toml`:

```toml
[dependencies]
crcl = { git = "https://github.com/kaidokert/crcl.git", tag = "v0.0.1" }
serde_json = "1.0"      # For JSON support
serde-xml-rs = "0.8"    # For XML support
```

## Step 2: Import the Types

In your Rust code:

```rust
use crcl::{JointStatusType, JointStatusesType, ActuateJointsType, ActuateJointType};
use crcl::commands::JointDetailsType;
```

## Step 3: Create and Use CRCL Messages

### Example: Joint Status (Read from Robot)

```rust
let joint = JointStatusType {
    name: None,
    joint_number: 0,
    joint_position: Some(1.57),           // 90 degrees in radians
    joint_torque_or_force: Some(10.5),    // N or Nm
    joint_velocity: Some(0.5),            // rad/s
};

// Convert to JSON
let json = serde_json::to_string_pretty(&joint)?;
println!("{}", json);

// Convert to XML
let xml = serde_xml_rs::to_string(&joint)?;
```

### Example: Joint Command (Send to Robot)

```rust
let command = ActuateJointsType {
    name: None,
    command_id: 42,
    guard: Vec::new(),
    actuate_joint: vec![
        ActuateJointType {
            name: None,
            joint_number: 0,
            joint_position: 1.57,
            joint_details: JointDetailsType { name: None },
        },
    ],
    joint_tolerances: None,
};

// Convert to JSON
let json = serde_json::to_string_pretty(&command)?;

// Convert to XML
let xml = serde_xml_rs::to_string(&command)?;
```

### Example: Parse Incoming Messages

```rust
// From JSON
let json = r#"{
    "name": null,
    "joint_number": 1,
    "joint_position": 0.785
}"#;
let joint: JointStatusType = serde_json::from_str(json)?;

// From XML
let xml = r#"<?xml version="1.0"?>
<JointStatusType>
    <joint_number>1</joint_number>
    <joint_position>0.785</joint_position>
</JointStatusType>"#;
let joint: JointStatusType = serde_xml_rs::from_str(xml)?;
```

## Step 4: Build and Run

```bash
cargo build
cargo run
```

## Complete Example

See the [test_consumer](../test_consumer) directory for a complete working example.

To run it:

```bash
cd test_consumer
cargo run
```

## Available Types

The library provides three main modules:

- **`crcl::primitives`** - Base types (vectors, points, poses, units)
- **`crcl::commands`** - Command types for robot control
- **`crcl::status`** - Status types for robot state

Common types are re-exported at the crate root:
- `JointStatusType` - Single joint state
- `JointStatusesType` - Collection of joint states
- `ActuateJointType` - Single joint command
- `ActuateJointsType` - Collection of joint commands

## More Information

- [Complete Documentation](crcl/README.md)
- [JSON Examples](crcl/JSON_EXAMPLES.md)
- [XSD Generator Documentation](xsd_gen/README.md)

## Troubleshooting

### "Could not find crcl"

Make sure you've added the git dependency to your `Cargo.toml` exactly as shown above.

### Linker Errors on Windows

See the Windows Linker Configuration section above.

### Type Not Found

Make sure you're importing from the correct module:
- Status types: `use crcl::{JointStatusType, JointStatusesType};`
- Command types: `use crcl::{ActuateJointType, ActuateJointsType};`
- Command details: `use crcl::commands::JointDetailsType;`

