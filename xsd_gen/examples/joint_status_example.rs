// Example showing how to create JointStatusType instances

// Include the generated types
#[allow(dead_code)]
mod crcl_status {
    include!("../generated_crcl_status_merged.rs");
}

use crcl_status::*;

fn main() {
    // Create a single joint status for joint 0
    // All fields with realistic values
    let joint0 = JointStatusType {
        content: JointStatusTypeContent {
            // name: Optional XML identifier inherited from DataThingType
            // In practice, you almost never need this - joint_number is the real ID
            name: None,

            // Required: which joint this status is for (0-indexed)
            joint_number: 0,

            // Optional fields - use Some(value) if available, None if not
            joint_position: Some(1.57),           // radians (90 degrees)
            joint_torque_or_force: Some(10.5),    // N or Nm depending on joint type
            joint_velocity: Some(0.5),            // rad/s
        }
    };

    // Another joint with only position (other sensors unavailable)
    let joint1 = JointStatusType {
        content: JointStatusTypeContent {
            name: None,
            joint_number: 1,
            joint_position: Some(-0.785),  // -45 degrees
            joint_torque_or_force: None,   // Not available
            joint_velocity: None,          // Not available
        }
    };

    // Combine multiple joint statuses
    let all_joints = JointStatusesType {
        content: JointStatusesTypeContent {
            name: None,  // Also optional XML identifier
            joint_status: vec![joint0, joint1],
        }
    };

    // Access the data
    println!("Total joints: {}", all_joints.content.joint_status.len());

    for js in &all_joints.content.joint_status {
        println!("Joint {}: position={:?}, velocity={:?}",
                 js.content.joint_number,
                 js.content.joint_position,
                 js.content.joint_velocity);
    }
}
