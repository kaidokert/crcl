# crcl

Rust library for CRCL (Canonical Robot Command Language) - providing strongly-typed structures for working with CRCL XML messages.

## Overview

CRCL is a standard XML-based protocol for robot control and status reporting. This library provides Rust types generated from the official CRCL XSD schemas, with full serde support for XML serialization and deserialization.

## Features

- **Type-safe** - All CRCL message types as Rust structs
- **Serde support** - Full XML and JSON serialization/deserialization
- **Clean API** - Flattened structure without redundant wrapper types
- **Well documented** - Generated from official CRCL schemas

## Installation

### From GitHub (Recommended)

Add to your `Cargo.toml`:

```toml
[dependencies]
crcl = { git = "https://github.com/kaidokert/crcl.git", tag = "v0.0.1" }
serde-xml-rs = "0.8"  # For XML support
serde_json = "1.0"    # For JSON support
```

## Usage

### Creating a Joint Status

```rust
use crcl::JointStatusType;

let joint = JointStatusType {
    name: None,
    joint_number: 0,
    joint_position: Some(1.57),           // radians
    joint_torque_or_force: Some(10.5),    // N or Nm
    joint_velocity: Some(0.5),            // rad/s
};

// Access fields directly - clean API!
println!("Joint {}: pos={:?}", joint.joint_number, joint.joint_position);
```

### Working with Collections

```rust
use crcl::JointStatusesType;

let all_joints = JointStatusesType {
    name: None,
    joint_status: vec![joint0, joint1, joint2],
};

// Iterate directly
for joint in &all_joints.joint_status {
    println!("Joint {}", joint.joint_number);
}
```

### XML Serialization

```rust
let xml = serde_xml_rs::to_string(&joint)?;
println!("{}", xml);
```

### XML Deserialization

```rust
let xml_input = r#"<?xml version="1.0"?>
<JointStatusType>
    <joint_number>1</joint_number>
    <joint_position>0.785</joint_position>
</JointStatusType>"#;

let joint: JointStatusType = serde_xml_rs::from_str(xml_input)?;
```

### Creating Commands

#### Simple Position Command (No speed/accel details)

```rust
use crcl::{ActuateJointType, ActuateJointsType};

let actuate_joint = ActuateJointType {
    name: None,
    joint_number: 0,
    joint_position: 1.57,
    joint_details: None,  // Simple position command
};

let command = ActuateJointsType {
    name: None,
    command_id: 42,
    guard: Vec::new(),
    actuate_joint: vec![actuate_joint],
    joint_tolerances: None,
};
```

#### Command with Speed and Acceleration

```rust
use crcl::{ActuateJointType, ActuateJointsType, JointDetails, JointSpeedAccelType};

let actuate_joint = ActuateJointType {
    name: None,
    joint_number: 0,
    joint_position: 1.57,
    joint_details: Some(JointDetails::SpeedAccel(JointSpeedAccelType {
        name: None,
        joint_speed: Some(1.5),  // rad/s
        joint_accel: Some(0.5),  // rad/s²
    })),
};
```

#### Command with Force/Torque

```rust
use crcl::{ActuateJointType, JointDetails, JointForceTorqueType};

let actuate_joint = ActuateJointType {
    name: None,
    joint_number: 1,
    joint_position: 0.0,
    joint_details: Some(JointDetails::ForceTorque(JointForceTorqueType {
        name: None,
        setting: Some(25.0),      // N or Nm
        change_rate: Some(5.0),   // N/s or Nm/s
    })),
};
```

#### Pattern Matching on JointDetails

```rust
match actuate_joint.joint_details {
    None => println!("Simple position command"),
    Some(JointDetails::SpeedAccel(ref details)) => {
        println!("Speed: {:?}, Accel: {:?}",
            details.joint_speed, details.joint_accel);
    }
    Some(JointDetails::ForceTorque(ref details)) => {
        println!("Force/Torque: {:?}", details.setting);
    }
}
```

## JSON Support

All CRCL types support JSON serialization in addition to XML.

### JSON Serialization - JointStatusType

```rust
use crcl::JointStatusType;

let joint = JointStatusType {
    name: None,
    joint_number: 0,
    joint_position: Some(1.57),
    joint_torque_or_force: Some(10.5),
    joint_velocity: Some(0.5),
};

// Serialize to pretty-printed JSON
let json = serde_json::to_string_pretty(&joint)?;
println!("{}", json);
```

Output:
```json
{
  "name": null,
  "joint_number": 0,
  "joint_position": 1.57,
  "joint_torque_or_force": 10.5,
  "joint_velocity": 0.5
}
```

### JSON Deserialization - JointStatusType

```rust
let json = r#"{
    "name": null,
    "joint_number": 1,
    "joint_position": 0.785,
    "joint_torque_or_force": 5.5,
    "joint_velocity": 0.2
}"#;

let joint: JointStatusType = serde_json::from_str(json)?;
```

### JSON Serialization - JointStatusesType

```rust
use crcl::JointStatusesType;

let all_joints = JointStatusesType {
    name: None,
    joint_status: vec![
        JointStatusType {
            name: None,
            joint_number: 0,
            joint_position: Some(1.57),
            joint_torque_or_force: Some(10.5),
            joint_velocity: Some(0.5),
        },
        JointStatusType {
            name: None,
            joint_number: 1,
            joint_position: Some(-0.785),
            joint_torque_or_force: Some(8.2),
            joint_velocity: Some(0.3),
        },
    ],
};

let json = serde_json::to_string_pretty(&all_joints)?;
```

Output:
```json
{
  "name": null,
  "joint_status": [
    {
      "name": null,
      "joint_number": 0,
      "joint_position": 1.57,
      "joint_torque_or_force": 10.5,
      "joint_velocity": 0.5
    },
    {
      "name": null,
      "joint_number": 1,
      "joint_position": -0.785,
      "joint_torque_or_force": 8.2,
      "joint_velocity": 0.3
    }
  ]
}
```

### JSON Serialization - ActuateJointsType

#### With Speed/Accel Details

```rust
use crcl::{ActuateJointType, ActuateJointsType, JointDetails, JointSpeedAccelType};

let command = ActuateJointsType {
    name: None,
    command_id: 42,
    guard: Vec::new(),
    actuate_joint: vec![
        ActuateJointType {
            name: None,
            joint_number: 0,
            joint_position: 1.57,
            joint_details: Some(JointDetails::SpeedAccel(JointSpeedAccelType {
                name: None,
                joint_speed: Some(1.5),
                joint_accel: Some(0.5),
            })),
        },
    ],
    joint_tolerances: None,
};

let json = serde_json::to_string_pretty(&command)?;
```

Output:
```json
{
  "name": null,
  "command_id": 42,
  "guard": [],
  "actuate_joint": [
    {
      "name": null,
      "joint_number": 0,
      "joint_position": 1.57,
      "joint_details": {
        "name": null,
        "joint_speed": 1.5,
        "joint_accel": 0.5
      }
    }
  ],
  "joint_tolerances": null
}
```

#### Without Details (None)

```rust
let command = ActuateJointsType {
    name: None,
    command_id: 123,
    guard: Vec::new(),
    actuate_joint: vec![
        ActuateJointType {
            name: None,
            joint_number: 0,
            joint_position: 1.57,
            joint_details: None,  // Simple position command
        },
    ],
    joint_tolerances: None,
};
```

Output:
```json
{
  "name": null,
  "command_id": 123,
  "guard": [],
  "actuate_joint": [
    {
      "name": null,
      "joint_number": 0,
      "joint_position": 1.57,
      "joint_details": null
    }
  ],
  "joint_tolerances": null
}
```

### JSON File I/O

```rust
// Save to JSON file
let json = serde_json::to_string_pretty(&all_joints)?;
std::fs::write("joint_statuses.json", json)?;

// Load from JSON file
let json = std::fs::read_to_string("joint_statuses.json")?;
let joints: JointStatusesType = serde_json::from_str(&json)?;
```

## Examples

See the `examples/` directory for complete working examples:
- `examples/json_usage.rs` - Comprehensive JSON serialization examples
- Run with: `cargo run --example json_usage`

## Modules

- **`primitives`** - Base data types (vectors, points, poses, unit enumerations)
- **`commands`** - Robot command message types
- **`status`** - Robot status message types

Common types are re-exported at the crate root for convenience.

## Regenerating Types

The types in this crate are generated from CRCL XSD schemas. To regenerate:

```bash
cd ../xsd_gen
cargo run --bin gen_crcl_types
```

This will update `crcl/src/commands.rs` and `crcl/src/status.rs`.

## License

MIT OR Apache-2.0
