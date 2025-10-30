//! JSON Serialization Examples for CRCL Types
//!
//! This example demonstrates how to serialize and deserialize CRCL messages
//! to/from JSON format using serde_json.
//!
//! Run with: cargo run --example json_usage

use crcl::{ActuateJointType, ActuateJointsType, JointStatusType, JointStatusesType};
use crcl::commands::JointDetailsType;

fn main() {
    println!("=== CRCL JSON Serialization Examples ===\n");

    // ===== Example 1: JointStatusType to JSON =====
    println!("1. JointStatusType → JSON\n");

    let joint = JointStatusType {
        name: None,
        joint_number: 0,
        joint_position: Some(1.57),           // 90 degrees in radians
        joint_torque_or_force: Some(10.5),    // N or Nm
        joint_velocity: Some(0.5),            // rad/s
    };

    let json = serde_json::to_string_pretty(&joint)
        .expect("Failed to serialize to JSON");
    println!("{}\n", json);

    // ===== Example 2: JSON to JointStatusType =====
    println!("2. JSON → JointStatusType\n");

    let json_input = r#"{
        "name": null,
        "joint_number": 1,
        "joint_position": 0.785,
        "joint_torque_or_force": 5.5,
        "joint_velocity": 0.2
    }"#;

    let parsed_joint: JointStatusType = serde_json::from_str(json_input)
        .expect("Failed to parse JSON");
    println!("Parsed joint {}:", parsed_joint.joint_number);
    println!("  Position: {:?} rad", parsed_joint.joint_position);
    println!("  Force: {:?} N", parsed_joint.joint_torque_or_force);
    println!("  Velocity: {:?} rad/s\n", parsed_joint.joint_velocity);

    // ===== Example 3: JointStatusesType to JSON =====
    println!("3. JointStatusesType (multiple joints) → JSON\n");

    let joint0 = JointStatusType {
        name: None,
        joint_number: 0,
        joint_position: Some(1.57),
        joint_torque_or_force: Some(10.5),
        joint_velocity: Some(0.5),
    };

    let joint1 = JointStatusType {
        name: None,
        joint_number: 1,
        joint_position: Some(-0.785),
        joint_torque_or_force: Some(8.2),
        joint_velocity: Some(0.3),
    };

    let joint2 = JointStatusType {
        name: None,
        joint_number: 2,
        joint_position: Some(0.0),
        joint_torque_or_force: None,  // Not available
        joint_velocity: None,
    };

    let all_joints = JointStatusesType {
        name: None,
        joint_status: vec![joint0, joint1, joint2],
    };

    let json = serde_json::to_string_pretty(&all_joints)
        .expect("Failed to serialize to JSON");
    println!("{}\n", json);

    // ===== Example 4: JSON to JointStatusesType =====
    println!("4. JSON → JointStatusesType\n");

    let json_input = r#"{
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
    }"#;

    let parsed_joints: JointStatusesType = serde_json::from_str(json_input)
        .expect("Failed to parse JSON");
    println!("Parsed {} joints:", parsed_joints.joint_status.len());
    for joint in &parsed_joints.joint_status {
        println!("  Joint {}: pos={:?}", joint.joint_number, joint.joint_position);
    }
    println!();

    // ===== Example 5: ActuateJointsType to JSON =====
    println!("5. ActuateJointsType (command) → JSON\n");

    let actuate_joint0 = ActuateJointType {
        name: None,
        joint_number: 0,
        joint_position: 1.57,
        joint_details: JointDetailsType { name: None },
    };

    let actuate_joint1 = ActuateJointType {
        name: None,
        joint_number: 1,
        joint_position: -0.785,
        joint_details: JointDetailsType { name: None },
    };

    let command = ActuateJointsType {
        name: None,
        command_id: 42,
        guard: Vec::new(),
        actuate_joint: vec![actuate_joint0, actuate_joint1],
        joint_tolerances: None,
    };

    let json = serde_json::to_string_pretty(&command)
        .expect("Failed to serialize to JSON");
    println!("{}\n", json);

    // ===== Example 6: JSON to ActuateJointsType =====
    println!("6. JSON → ActuateJointsType\n");

    let json_input = r#"{
        "name": null,
        "command_id": 123,
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
    }"#;

    let parsed_command: ActuateJointsType = serde_json::from_str(json_input)
        .expect("Failed to parse JSON");
    println!("Parsed command ID: {}", parsed_command.command_id);
    println!("Actuating {} joint(s):", parsed_command.actuate_joint.len());
    for joint in &parsed_command.actuate_joint {
        println!("  Joint {}: target position = {}", joint.joint_number, joint.joint_position);
    }
    println!();

    // ===== Example 7: Compact JSON (not pretty-printed) =====
    println!("7. Compact JSON (single-line)\n");

    let compact = serde_json::to_string(&joint)
        .expect("Failed to serialize to compact JSON");
    println!("{}\n", compact);

    // ===== Example 8: Working with JSON files =====
    println!("8. File I/O Example\n");

    // Write to file
    let filename = "joint_statuses.json";
    let json = serde_json::to_string_pretty(&all_joints)
        .expect("Failed to serialize");
    std::fs::write(filename, json)
        .expect("Failed to write file");
    println!("✓ Saved to {}", filename);

    // Read from file
    let json = std::fs::read_to_string(filename)
        .expect("Failed to read file");
    let loaded: JointStatusesType = serde_json::from_str(&json)
        .expect("Failed to parse JSON from file");
    println!("✓ Loaded {} joints from {}", loaded.joint_status.len(), filename);

    // Clean up
    std::fs::remove_file(filename).ok();

    println!("\n=== Done ===");
}
