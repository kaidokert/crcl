//! SCARA Robot Status Example
//!
//! This example creates a CRCL status message for a 3-axis SCARA robot
//! and demonstrates serialization to both JSON and XML formats.

use crcl::status::{JointStatusType, JointStatusesType};
use serde_json;
use serde_xml_rs;

fn main() {
    println!("=== SCARA Robot Status Example ===\n");

    // Create joint statuses for a 3-axis SCARA robot
    let joint_statuses = JointStatusesType {
        name: Some("SCARA_Robot_Joint_Status".to_string()),
        joint_status: vec![
            JointStatusType {
                name: Some("J1_Base_Rotation".to_string()),
                joint_number: 1,
                joint_position: Some(45.5),        // degrees
                joint_torque_or_force: Some(12.3), // Nm
                joint_velocity: Some(15.0),        // deg/s
            },
            JointStatusType {
                name: Some("J2_Shoulder_Joint".to_string()),
                joint_number: 2,
                joint_position: Some(30.25),       // degrees
                joint_torque_or_force: Some(8.7),  // Nm
                joint_velocity: Some(10.5),        // deg/s
            },
            JointStatusType {
                name: Some("J3_Z_Axis_Linear".to_string()),
                joint_number: 3,
                joint_position: Some(150.0),       // mm
                joint_torque_or_force: Some(5.2),  // N
                joint_velocity: Some(25.0),        // mm/s
            },
            JointStatusType {
                name: Some("J4_Wrist_Rotation".to_string()),
                joint_number: 4,
                joint_position: Some(90.0),        // degrees
                joint_torque_or_force: Some(2.1),  // Nm
                joint_velocity: Some(45.0),        // deg/s
            },
        ],
    };

    // Serialize to JSON
    println!("JSON Format:");
    println!("------------");
    let json = serde_json::to_string_pretty(&joint_statuses)
        .expect("Failed to serialize to JSON");
    println!("{}\n", json);

    // Serialize to XML
    println!("XML Format:");
    println!("-----------");
    let xml = serde_xml_rs::to_string(&joint_statuses)
        .expect("Failed to serialize to XML");

    // Pretty-print XML by adding newlines
    let pretty_xml = xml
        .replace("><", ">\n<")
        .replace("<joint_status>", "\n  <joint_status>")
        .replace("</joint_status>", "\n  </joint_status>");

    println!("{}\n", pretty_xml);

    // Demonstrate parsing back from JSON
    println!("Round-trip test (JSON → Struct → JSON):");
    println!("----------------------------------------");
    let parsed: JointStatusesType = serde_json::from_str(&json)
        .expect("Failed to parse JSON");

    println!("Successfully parsed {} joints", parsed.joint_status.len());
    for joint in &parsed.joint_status {
        println!("  Joint {}: {} at position {:?}",
                 joint.joint_number,
                 joint.name.as_ref().unwrap_or(&"unnamed".to_string()),
                 joint.joint_position);
    }
}