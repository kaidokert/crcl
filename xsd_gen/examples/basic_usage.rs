// Basic usage example for CRCL types
//
// This example demonstrates:
// - Creating JointStatusType messages
// - Serializing to XML
// - Deserializing from XML
// - Working with collections (JointStatusesType)
// - Creating ActuateJointsType commands

#[allow(dead_code)]
mod crcl_status {
    include!("../generated_crcl_status_merged.rs");
}

#[allow(dead_code)]
mod crcl_commands {
    include!("../generated_crcl_commands_merged.rs");
}

use crcl_status::*;
use crcl_commands::*;

fn main() {
    println!("=== CRCL Basic Usage Example ===\n");

    // ===== Example 1: Create a single joint status =====
    println!("1. Creating JointStatusType:");

    let joint0 = JointStatusType {
        name: None,
        joint_number: 0,
        joint_position: Some(1.57),           // 90 degrees in radians
        joint_torque_or_force: Some(10.5),    // N or Nm
        joint_velocity: Some(0.5),            // rad/s
    };

    println!("   Joint {}: position={:?}, velocity={:?}\n",
             joint0.joint_number, joint0.joint_position, joint0.joint_velocity);

    // ===== Example 2: Serialize to XML =====
    println!("2. Serializing to XML:");

    match serde_xml_rs::to_string(&joint0) {
        Ok(xml) => {
            println!("{}\n", xml);
        }
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // ===== Example 3: Deserialize from XML =====
    println!("3. Deserializing from XML:");

    let xml_input = r#"<?xml version="1.0"?>
<JointStatusType>
    <joint_number>1</joint_number>
    <joint_position>0.785</joint_position>
    <joint_torque_or_force>5.5</joint_torque_or_force>
    <joint_velocity>0.2</joint_velocity>
</JointStatusType>"#;

    match serde_xml_rs::from_str::<JointStatusType>(xml_input) {
        Ok(joint) => {
            println!("   Parsed joint {}:", joint.joint_number);
            println!("   Position: {:?} rad", joint.joint_position);
            println!("   Force: {:?} N", joint.joint_torque_or_force);
            println!("   Velocity: {:?} rad/s\n", joint.joint_velocity);
        }
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // ===== Example 4: Working with collections =====
    println!("4. Creating JointStatusesType (multiple joints):");

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

    println!("   Total joints: {}", all_joints.joint_status.len());
    for joint in &all_joints.joint_status {
        println!("   Joint {}: pos={:?}", joint.joint_number, joint.joint_position);
    }
    println!();

    // ===== Example 5: Creating a command =====
    println!("5. Creating ActuateJointsType command:");

    let actuate_joint = ActuateJointType {
        name: None,
        joint_number: 0,
        joint_position: 1.57,
        joint_details: JointDetailsType {
            name: None,
        },
    };

    let command = ActuateJointsType {
        name: None,
        command_id: 42,
        guard: Vec::new(),
        actuate_joint: vec![actuate_joint],
        joint_tolerances: None,
    };

    println!("   Command ID: {}", command.command_id);
    println!("   Actuating {} joint(s)\n", command.actuate_joint.len());

    // ===== Example 6: Save/Load from file (commented for safety) =====
    /*
    use std::fs;

    // Save to file
    let xml = serde_xml_rs::to_string(&all_joints).unwrap();
    fs::write("joint_statuses.xml", xml).expect("Failed to write file");
    println!("Saved to joint_statuses.xml");

    // Load from file
    let xml = fs::read_to_string("joint_statuses.xml").expect("Failed to read file");
    let loaded: JointStatusesType = serde_xml_rs::from_str(&xml).unwrap();
    println!("Loaded {} joints from file", loaded.joint_status.len());
    */

    println!("=== Done ===");
}
