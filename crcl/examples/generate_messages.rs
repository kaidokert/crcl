//! Generate CRCL messages and save them to files
//!
//! This example demonstrates Rust generating CRCL messages
//! that will be consumed by Python
//!
//! Run with: cargo run --example generate_messages

use crcl::status::{JointStatusType, JointStatusesType};
use serde_json;
use serde_xml_rs;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust CRCL Message Generator ===\n");

    // Create test directory
    let test_dir = Path::new("test_messages");
    fs::create_dir_all(&test_dir)?;
    println!("Created directory: {}", test_dir.display());

    // Generate first message: Single joint status
    let joint1 = JointStatusType {
        name: Some("Robot_1_Joint_1".to_string()),
        joint_number: 1,
        joint_position: Some(45.5),
        joint_torque_or_force: Some(12.3),
        joint_velocity: Some(15.0),
    };

    // Save as JSON
    let json1 = serde_json::to_string_pretty(&joint1)?;
    let json1_path = test_dir.join("joint_status_1.json");
    fs::write(&json1_path, &json1)?;
    println!("\n✓ Generated: {}", json1_path.display());
    println!("  Content: {} bytes", json1.len());

    // Generate second message: Multiple joints (SCARA robot)
    let scara_joints = JointStatusesType {
        name: Some("SCARA_Robot_Status".to_string()),
        joint_status: vec![
            JointStatusType {
                name: Some("J1_Base_Rotation".to_string()),
                joint_number: 1,
                joint_position: Some(45.5),
                joint_torque_or_force: Some(12.3),
                joint_velocity: Some(15.0),
            },
            JointStatusType {
                name: Some("J2_Shoulder_Joint".to_string()),
                joint_number: 2,
                joint_position: Some(30.25),
                joint_torque_or_force: Some(8.7),
                joint_velocity: Some(10.5),
            },
            JointStatusType {
                name: Some("J3_Z_Axis_Prismatic".to_string()),
                joint_number: 3,
                joint_position: Some(150.0),  // mm
                joint_torque_or_force: Some(5.2),  // N
                joint_velocity: Some(25.0),  // mm/s
            },
            JointStatusType {
                name: Some("J4_Wrist_Rotation".to_string()),
                joint_number: 4,
                joint_position: Some(90.0),
                joint_torque_or_force: Some(2.1),
                joint_velocity: Some(45.0),
            },
        ],
    };

    // Save as JSON
    let json2 = serde_json::to_string_pretty(&scara_joints)?;
    let json2_path = test_dir.join("scara_joints.json");
    fs::write(&json2_path, &json2)?;
    println!("\n✓ Generated: {}", json2_path.display());
    println!("  Content: {} bytes", json2.len());
    println!("  Joints: {}", scara_joints.joint_status.len());

    // Generate third message: Time series data (multiple snapshots)
    let mut time_series = Vec::new();
    for t in 0..5 {
        let time = t as f64 * 0.1;  // 0.0, 0.1, 0.2, 0.3, 0.4 seconds
        let joints = JointStatusesType {
            name: Some(format!("Trajectory_Point_{}", t)),
            joint_status: vec![
                JointStatusType {
                    name: Some("J1".to_string()),
                    joint_number: 1,
                    joint_position: Some(45.0 + 10.0 * time.sin()),
                    joint_torque_or_force: Some(10.0 + 2.0 * time.cos()),
                    joint_velocity: Some(5.0 * time.cos()),
                },
                JointStatusType {
                    name: Some("J2".to_string()),
                    joint_number: 2,
                    joint_position: Some(30.0 + 15.0 * time.cos()),
                    joint_torque_or_force: Some(8.0 + 1.5 * time.sin()),
                    joint_velocity: Some(7.5 * time.sin()),
                },
            ],
        };
        time_series.push(joints);
    }

    // Save time series as JSON array
    let json3 = serde_json::to_string_pretty(&time_series)?;
    let json3_path = test_dir.join("trajectory_sequence.json");
    fs::write(&json3_path, &json3)?;
    println!("\n✓ Generated: {}", json3_path.display());
    println!("  Content: {} bytes", json3.len());
    println!("  Snapshots: {}", time_series.len());

    // Also generate XML versions for demonstration
    println!("\n--- XML Versions ---");

    // Single joint as XML
    let xml1 = serde_xml_rs::to_string(&joint1)?;
    let xml1_path = test_dir.join("joint_status_1.xml");
    fs::write(&xml1_path, format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", xml1))?;
    println!("✓ Generated: {}", xml1_path.display());

    // SCARA joints as XML
    let xml2 = serde_xml_rs::to_string(&scara_joints)?;
    let xml2_path = test_dir.join("scara_joints.xml");
    fs::write(&xml2_path, format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", xml2))?;
    println!("✓ Generated: {}", xml2_path.display());

    // Summary
    println!("\n=== Generation Complete ===");
    println!("Generated 5 files in {}/", test_dir.display());
    println!("  - 3 JSON files");
    println!("  - 2 XML files");
    println!("\nThese files can now be read by Python!");

    // Print sample of the first JSON for verification
    println!("\nSample of joint_status_1.json:");
    println!("{}", &json1[..200.min(json1.len())]);

    Ok(())
}