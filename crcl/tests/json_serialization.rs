use crcl::{JointStatusType, JointStatusesType, ActuateJointType, ActuateJointsType, JointDetails, JointSpeedAccelType};

#[test]
fn test_joint_status_to_json() {
    let joint = JointStatusType {
        name: None,
        joint_number: 0,
        joint_position: Some(1.57),
        joint_torque_or_force: Some(10.5),
        joint_velocity: Some(0.5),
    };

    let json = serde_json::to_string_pretty(&joint).expect("Failed to serialize to JSON");
    println!("JointStatusType JSON:\n{}\n", json);

    // Verify structure
    assert!(json.contains("\"joint_number\": 0"));
    assert!(json.contains("\"joint_position\": 1.57"));
    assert!(json.contains("\"joint_torque_or_force\": 10.5"));
    assert!(json.contains("\"joint_velocity\": 0.5"));
}

#[test]
fn test_joint_status_from_json() {
    let json = r#"{
        "name": null,
        "joint_number": 1,
        "joint_position": 0.785,
        "joint_torque_or_force": 5.5,
        "joint_velocity": 0.2
    }"#;

    let joint: JointStatusType = serde_json::from_str(json).expect("Failed to parse JSON");

    assert_eq!(joint.joint_number, 1);
    assert_eq!(joint.joint_position, Some(0.785));
    assert_eq!(joint.joint_torque_or_force, Some(5.5));
    assert_eq!(joint.joint_velocity, Some(0.2));
}

#[test]
fn test_joint_statuses_to_json() {
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
        joint_torque_or_force: None,
        joint_velocity: None,
    };

    let all_joints = JointStatusesType {
        name: None,
        joint_status: vec![joint0, joint1, joint2],
    };

    let json = serde_json::to_string_pretty(&all_joints).expect("Failed to serialize to JSON");
    println!("JointStatusesType JSON:\n{}\n", json);

    // Verify structure
    assert!(json.contains("\"joint_status\""));
    assert!(json.contains("\"joint_number\": 0"));
    assert!(json.contains("\"joint_number\": 1"));
    assert!(json.contains("\"joint_number\": 2"));
}

#[test]
fn test_joint_statuses_from_json() {
    let json = r#"{
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

    let all_joints: JointStatusesType = serde_json::from_str(json).expect("Failed to parse JSON");

    assert_eq!(all_joints.joint_status.len(), 2);
    assert_eq!(all_joints.joint_status[0].joint_number, 0);
    assert_eq!(all_joints.joint_status[0].joint_position, Some(1.57));
    assert_eq!(all_joints.joint_status[1].joint_number, 1);
    assert_eq!(all_joints.joint_status[1].joint_position, Some(-0.785));
}

#[test]
fn test_actuate_joints_to_json() {
    let actuate_joint0 = ActuateJointType {
        name: None,
        joint_number: 0,
        joint_position: 1.57,
        joint_details: Some(JointDetails::SpeedAccel(JointSpeedAccelType {
            name: None,
            joint_speed: Some(1.0),
            joint_accel: Some(0.5),
        })),
    };

    let actuate_joint1 = ActuateJointType {
        name: None,
        joint_number: 1,
        joint_position: -0.785,
        joint_details: Some(JointDetails::SpeedAccel(JointSpeedAccelType {
            name: None,
            joint_speed: Some(2.0),
            joint_accel: Some(1.0),
        })),
    };

    let command = ActuateJointsType {
        name: None,
        command_id: 42,
        guard: Vec::new(),
        actuate_joint: vec![actuate_joint0, actuate_joint1],
        joint_tolerances: None,
    };

    let json = serde_json::to_string_pretty(&command).expect("Failed to serialize to JSON");
    println!("ActuateJointsType JSON:\n{}\n", json);

    // Verify structure
    assert!(json.contains("\"command_id\": 42"));
    assert!(json.contains("\"actuate_joint\""));
    assert!(json.contains("\"joint_number\": 0"));
    assert!(json.contains("\"joint_position\": 1.57"));
}

#[test]
fn test_actuate_joints_from_json() {
    let json = r#"{
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

    let command: ActuateJointsType = serde_json::from_str(json).expect("Failed to parse JSON");

    assert_eq!(command.command_id, 123);
    assert_eq!(command.actuate_joint.len(), 2);
    assert_eq!(command.actuate_joint[0].joint_number, 0);
    assert_eq!(command.actuate_joint[0].joint_position, 1.57);
    assert_eq!(command.actuate_joint[1].joint_number, 1);
    assert_eq!(command.actuate_joint[1].joint_position, -0.785);
}

#[test]
fn test_json_roundtrip_joint_statuses() {
    let original = JointStatusesType {
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

    // Serialize to JSON
    let json = serde_json::to_string(&original).expect("Failed to serialize");

    // Deserialize back
    let parsed: JointStatusesType = serde_json::from_str(&json).expect("Failed to deserialize");

    // Verify data matches
    assert_eq!(parsed.joint_status.len(), original.joint_status.len());
    assert_eq!(parsed.joint_status[0].joint_number, original.joint_status[0].joint_number);
    assert_eq!(parsed.joint_status[0].joint_position, original.joint_status[0].joint_position);
}

#[test]
fn test_json_roundtrip_actuate_joints() {
    let original = ActuateJointsType {
        name: None,
        command_id: 999,
        guard: Vec::new(),
        actuate_joint: vec![
            ActuateJointType {
                name: None,
                joint_number: 0,
                joint_position: 1.57,
                joint_details: Some(JointDetails::SpeedAccel(JointSpeedAccelType {
                    name: None,
                    joint_speed: Some(1.0),
                    joint_accel: Some(0.5),
                })),
            },
        ],
        joint_tolerances: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&original).expect("Failed to serialize");

    // Deserialize back
    let parsed: ActuateJointsType = serde_json::from_str(&json).expect("Failed to deserialize");

    // Verify data matches
    assert_eq!(parsed.command_id, original.command_id);
    assert_eq!(parsed.actuate_joint.len(), original.actuate_joint.len());
    assert_eq!(parsed.actuate_joint[0].joint_number, original.actuate_joint[0].joint_number);
    assert_eq!(parsed.actuate_joint[0].joint_position, original.actuate_joint[0].joint_position);
}
