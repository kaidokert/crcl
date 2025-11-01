# JointDetails: Speed/Acceleration vs Force/Torque Control

## Overview

The `JointDetails` field in `ActuateJointType` allows you to specify **how** a joint should move to its target position. CRCL supports two control modes:

1. **Speed/Acceleration Control** - Specify velocity and acceleration limits
2. **Force/Torque Control** - Specify force/torque and rate of change

## Background

In the CRCL XSD schema, `JointDetailsType` is an abstract base type with two concrete implementations:
- `JointSpeedAccelType` - for velocity control
- `JointForceTorqueType` - for force control

The Rust implementation wraps these in a `JointDetails` enum for type-safe polymorphism.

## Usage

### Speed/Acceleration Control (Most Common)

Use this for position-controlled moves with velocity and acceleration limits:

```rust
use crcl::{ActuateJointType, JointDetails, JointSpeedAccelType};

let joint = ActuateJointType {
    name: None,
    joint_number: 0,
    joint_position: 1.57,  // Target position in radians
    joint_details: JointDetails::SpeedAccel(JointSpeedAccelType {
        name: None,
        joint_speed: Some(1.5),      // rad/s for rotational joints
        joint_accel: Some(0.5),      // rad/s² for rotational joints
    }),
};
```

**Units:**
- **Rotational joints**: angle units/second (speed), angle units/second² (accel)
- **Translational joints**: length units/second (speed), length units/second² (accel)

### Force/Torque Control

Use this for compliance control or when you want to limit the applied force:

```rust
use crcl::{ActuateJointType, JointDetails, JointForceTorqueType};

let joint = ActuateJointType {
    name: None,
    joint_number: 1,
    joint_position: 0.0,   // Target position
    joint_details: JointDetails::ForceTorque(JointForceTorqueType {
        name: None,
        setting: Some(25.0),          // N for translational, Nm for rotational
        change_rate: Some(5.0),       // N/s or Nm/s
    }),
};
```

**Units:**
- **Rotational joints**: torque units (Nm), torque units/second
- **Translational joints**: force units (N), force units/second

## Complete Example with ActuateJointsType

```rust
use crcl::{ActuateJointsType, ActuateJointType, JointDetails,
           JointSpeedAccelType, JointForceTorqueType};

let command = ActuateJointsType {
    name: None,
    command_id: 42,
    guard: Vec::new(),
    actuate_joint: vec![
        // Joint 0: Speed control
        ActuateJointType {
            name: None,
            joint_number: 0,
            joint_position: 1.57,
            joint_details: JointDetails::SpeedAccel(JointSpeedAccelType {
                name: None,
                joint_speed: Some(1.0),
                joint_accel: Some(0.5),
            }),
        },
        // Joint 1: Force control
        ActuateJointType {
            name: None,
            joint_number: 1,
            joint_position: 0.0,
            joint_details: JointDetails::ForceTorque(JointForceTorqueType {
                name: None,
                setting: Some(10.0),
                change_rate: Some(2.0),
            }),
        },
    ],
    joint_tolerances: None,
};

// Serialize to JSON
let json = serde_json::to_string_pretty(&command)?;
```

## JSON Representation

### Speed/Acceleration

```json
{
  "name": null,
  "joint_number": 0,
  "joint_position": 1.57,
  "joint_details": {
    "name": null,
    "joint_speed": 1.0,
    "joint_accel": 0.5
  }
}
```

### Force/Torque

```json
{
  "name": null,
  "joint_number": 1,
  "joint_position": 0.0,
  "joint_details": {
    "name": null,
    "setting": 10.0,
    "change_rate": 2.0
  }
}
```

## Pattern Matching

You can inspect which control mode is being used:

```rust
match joint.joint_details {
    JointDetails::SpeedAccel(ref details) => {
        println!("Speed control: {:?}", details.joint_speed);
        println!("Accel: {:?}", details.joint_accel);
    }
    JointDetails::ForceTorque(ref details) => {
        println!("Force/Torque control: {:?}", details.setting);
        println!("Change rate: {:?}", details.change_rate);
    }
}
```

## When to Use Each Mode

### Use Speed/Acceleration when:
- You want predictable, smooth motion
- You're doing pick-and-place operations
- You need coordinated multi-joint moves
- **This is the most common mode**

### Use Force/Torque when:
- You're doing assembly with insertion forces
- You need compliant behavior (e.g., polishing)
- You want to limit forces during contact
- You're doing force-controlled search operations

## Optional Fields

All fields in both control modes are optional (except `joint_number` and `joint_position` in the parent `ActuateJointType`):

```rust
// Minimal - let robot use defaults
JointDetails::SpeedAccel(JointSpeedAccelType {
    name: None,
    joint_speed: None,    // Use robot default
    joint_accel: None,    // Use robot default
})
```

## See Also

- [JSON Examples](JSON_EXAMPLES.md) - Full JSON serialization examples
- Tests: `tests/joint_details.rs` - Comprehensive test suite
- CRCL Spec: Section on ActuateJointsType
