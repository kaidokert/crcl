use crcl::{JointStatusType, JointStatusesType};

#[test]
fn test_create_joint_status() {
    let joint = JointStatusType {
        name: None,
        joint_number: 0,
        joint_position: Some(1.57),
        joint_torque_or_force: Some(10.5),
        joint_velocity: Some(0.5),
    };

    assert_eq!(joint.joint_number, 0);
    assert_eq!(joint.joint_position, Some(1.57));
}

#[test]
fn test_joint_status_xml_serialization() {
    let joint = JointStatusType {
        name: None,
        joint_number: 1,
        joint_position: Some(0.785),
        joint_torque_or_force: Some(5.5),
        joint_velocity: Some(0.2),
    };

    let xml = serde_xml_rs::to_string(&joint).expect("Failed to serialize");

    // Verify no <content> wrapper
    assert!(!xml.contains("<content>"));
    assert!(xml.contains("<joint_number>1</joint_number>"));
}

#[test]
fn test_joint_statuses_collection() {
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

    let all_joints = JointStatusesType {
        name: None,
        joint_status: vec![joint0, joint1],
    };

    assert_eq!(all_joints.joint_status.len(), 2);
    assert_eq!(all_joints.joint_status[0].joint_number, 0);
    assert_eq!(all_joints.joint_status[1].joint_number, 1);
}

#[test]
fn test_import_from_crate_root() {
    // Test that the re-exports work
    let _joint: crcl::JointStatusType;
    let _joints: crcl::JointStatusesType;
}
