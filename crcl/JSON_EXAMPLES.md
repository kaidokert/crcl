# JSON Serialization Examples

Quick reference for JSON serialization of key CRCL types.

## JointStatusType (Single Joint Status)

### Rust Code
```rust
use crcl::JointStatusType;

let joint = JointStatusType {
    name: None,
    joint_number: 0,
    joint_position: Some(1.57),           // 90 degrees in radians
    joint_torque_or_force: Some(10.5),    // N or Nm
    joint_velocity: Some(0.5),            // rad/s
};

let json = serde_json::to_string_pretty(&joint)?;
```

### JSON Output
```json
{
  "name": null,
  "joint_number": 0,
  "joint_position": 1.57,
  "joint_torque_or_force": 10.5,
  "joint_velocity": 0.5
}
```

## JointStatusesType (Multiple Joint Statuses)

### Rust Code
```rust
use crcl::{JointStatusType, JointStatusesType};

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
        JointStatusType {
            name: None,
            joint_number: 2,
            joint_position: Some(0.0),
            joint_torque_or_force: None,  // Sensor not available
            joint_velocity: None,
        },
    ],
};

let json = serde_json::to_string_pretty(&all_joints)?;
```

### JSON Output
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
    },
    {
      "name": null,
      "joint_number": 2,
      "joint_position": 0.0,
      "joint_torque_or_force": null,
      "joint_velocity": null
    }
  ]
}
```

## ActuateJointsType (Robot Command)

### Rust Code
```rust
use crcl::{ActuateJointType, ActuateJointsType};
use crcl::commands::JointDetailsType;

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
        ActuateJointType {
            name: None,
            joint_number: 1,
            joint_position: -0.785,
            joint_details: JointDetailsType { name: None },
        },
    ],
    joint_tolerances: None,
};

let json = serde_json::to_string_pretty(&command)?;
```

### JSON Output
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
        "name": null
      }
    },
    {
      "name": null,
      "joint_number": 1,
      "joint_position": -0.785,
      "joint_details": {
        "name": null
      }
    }
  ],
  "joint_tolerances": null
}
```

## Parsing JSON

All types support bidirectional conversion:

```rust
// Parse JSON string to Rust struct
let json = r#"{
    "name": null,
    "joint_number": 1,
    "joint_position": 0.785,
    "joint_torque_or_force": 5.5,
    "joint_velocity": 0.2
}"#;

let joint: JointStatusType = serde_json::from_str(json)?;

// Read from JSON file
let json = std::fs::read_to_string("joint_statuses.json")?;
let joints: JointStatusesType = serde_json::from_str(&json)?;
```

## Compact JSON

For network transmission or storage, use compact format:

```rust
let json = serde_json::to_string(&joint)?;  // Single line, no whitespace
```

Example output:
```json
{"name":null,"joint_number":0,"joint_position":1.57,"joint_torque_or_force":10.5,"joint_velocity":0.5}
```

## Complete Example

See `examples/json_usage.rs` for a complete runnable example:

```bash
cd crcl
cargo run --example json_usage
```

## Tests

Comprehensive JSON tests are available in `tests/json_serialization.rs`:

```bash
cd crcl
cargo test json_serialization
```
