use crcl::{ActuateJointType, ActuateJointsType, JointDetails, JointSpeedAccelType, JointForceTorqueType};

#[test]
fn test_joint_details_speed_accel() {
    let joint = ActuateJointType {
        name: None,
        joint_number: 0,
        joint_position: 1.57,
        joint_details: Some(JointDetails::SpeedAccel(JointSpeedAccelType {
            name: None,
            joint_speed: Some(1.5),
            joint_accel: Some(0.5),
        })),
    };

    assert_eq!(joint.joint_number, 0);
    assert_eq!(joint.joint_position, 1.57);

    match joint.joint_details {
        Some(JointDetails::SpeedAccel(ref details)) => {
            assert_eq!(details.joint_speed, Some(1.5));
            assert_eq!(details.joint_accel, Some(0.5));
        }
        _ => panic!("Expected SpeedAccel variant"),
    }
}

#[test]
fn test_joint_details_force_torque() {
    let joint = ActuateJointType {
        name: None,
        joint_number: 1,
        joint_position: -0.785,
        joint_details: Some(JointDetails::ForceTorque(JointForceTorqueType {
            name: None,
            setting: Some(10.5),
            change_rate: Some(2.0),
        })),
    };

    match joint.joint_details {
        Some(JointDetails::ForceTorque(ref details)) => {
            assert_eq!(details.setting, Some(10.5));
            assert_eq!(details.change_rate, Some(2.0));
        }
        _ => panic!("Expected ForceTorque variant"),
    }
}

#[test]
fn test_actuate_joints_with_speed_accel_json() {
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
            ActuateJointType {
                name: None,
                joint_number: 1,
                joint_position: -0.785,
                joint_details: Some(JointDetails::SpeedAccel(JointSpeedAccelType {
                    name: None,
                    joint_speed: Some(2.0),
                    joint_accel: Some(1.0),
                })),
            },
        ],
        joint_tolerances: None,
    };

    let json = serde_json::to_string_pretty(&command).expect("Failed to serialize");
    println!("ActuateJointsType with JointDetails::SpeedAccel:\n{}\n", json);

    // Verify JSON contains speed and accel fields
    assert!(json.contains("\"joint_speed\": 1.5"));
    assert!(json.contains("\"joint_accel\": 0.5"));
    assert!(json.contains("\"joint_speed\": 2.0"));
    assert!(json.contains("\"joint_accel\": 1.0"));
}

#[test]
fn test_actuate_joints_with_force_torque_json() {
    let command = ActuateJointsType {
        name: None,
        command_id: 99,
        guard: Vec::new(),
        actuate_joint: vec![
            ActuateJointType {
                name: None,
                joint_number: 0,
                joint_position: 0.0,
                joint_details: Some(JointDetails::ForceTorque(JointForceTorqueType {
                    name: None,
                    setting: Some(25.0),
                    change_rate: Some(5.0),
                })),
            },
        ],
        joint_tolerances: None,
    };

    let json = serde_json::to_string_pretty(&command).expect("Failed to serialize");
    println!("ActuateJointsType with JointDetails::ForceTorque:\n{}\n", json);

    // Verify JSON contains setting and change_rate fields
    assert!(json.contains("\"setting\": 25.0"));
    assert!(json.contains("\"change_rate\": 5.0"));
}

#[test]
fn test_parse_json_speed_accel() {
    let json = r#"{
        "name": null,
        "joint_number": 0,
        "joint_position": 1.57,
        "joint_details": {
            "name": null,
            "joint_speed": 1.5,
            "joint_accel": 0.5
        }
    }"#;

    let joint: ActuateJointType = serde_json::from_str(json).expect("Failed to parse JSON");

    assert_eq!(joint.joint_number, 0);
    assert_eq!(joint.joint_position, 1.57);

    match joint.joint_details {
        Some(JointDetails::SpeedAccel(details)) => {
            assert_eq!(details.joint_speed, Some(1.5));
            assert_eq!(details.joint_accel, Some(0.5));
        }
        _ => panic!("Expected SpeedAccel variant"),
    }
}

#[test]
fn test_parse_json_force_torque() {
    let json = r#"{
        "name": null,
        "joint_number": 1,
        "joint_position": 0.0,
        "joint_details": {
            "name": null,
            "setting": 25.0,
            "change_rate": 5.0
        }
    }"#;

    let joint: ActuateJointType = serde_json::from_str(json).expect("Failed to parse JSON");

    match joint.joint_details {
        Some(JointDetails::ForceTorque(details)) => {
            assert_eq!(details.setting, Some(25.0));
            assert_eq!(details.change_rate, Some(5.0));
        }
        _ => panic!("Expected ForceTorque variant"),
    }
}

#[test]
fn test_roundtrip_speed_accel() {
    let original = ActuateJointsType {
        name: None,
        command_id: 123,
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

    // Serialize to JSON
    let json = serde_json::to_string(&original).expect("Failed to serialize");

    // Deserialize back
    let parsed: ActuateJointsType = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(parsed.command_id, original.command_id);
    assert_eq!(parsed.actuate_joint.len(), 1);
    assert_eq!(parsed.actuate_joint[0].joint_number, 0);
    assert_eq!(parsed.actuate_joint[0].joint_position, 1.57);

    match &parsed.actuate_joint[0].joint_details {
        Some(JointDetails::SpeedAccel(details)) => {
            assert_eq!(details.joint_speed, Some(1.5));
            assert_eq!(details.joint_accel, Some(0.5));
        }
        _ => panic!("Expected SpeedAccel variant"),
    }
}

#[test]
fn test_joint_details_none_to_json() {
    let joint = ActuateJointType {
        name: None,
        joint_number: 0,
        joint_position: 1.57,
        joint_details: None,
    };

    let json = serde_json::to_string_pretty(&joint).expect("Failed to serialize");
    println!("ActuateJointType with None joint_details:\n{}\n", json);

    // Verify joint_details is null in JSON
    assert!(json.contains("\"joint_details\": null"));
    assert_eq!(joint.joint_number, 0);
    assert_eq!(joint.joint_position, 1.57);
}

#[test]
fn test_parse_json_with_null_joint_details() {
    let json = r#"{
        "name": null,
        "joint_number": 0,
        "joint_position": 1.57,
        "joint_details": null
    }"#;

    let joint: ActuateJointType = serde_json::from_str(json).expect("Failed to parse JSON");

    assert_eq!(joint.joint_number, 0);
    assert_eq!(joint.joint_position, 1.57);
    assert!(joint.joint_details.is_none());
}

#[test]
fn test_parse_json_with_missing_joint_details() {
    // Test that joint_details field can be omitted entirely from JSON
    let json = r#"{
        "name": null,
        "joint_number": 1,
        "joint_position": -0.785
    }"#;

    let joint: ActuateJointType = serde_json::from_str(json).expect("Failed to parse JSON");

    assert_eq!(joint.joint_number, 1);
    assert_eq!(joint.joint_position, -0.785);
    assert!(joint.joint_details.is_none());
}

#[test]
fn test_roundtrip_joint_details_none() {
    let original = ActuateJointsType {
        name: None,
        command_id: 456,
        guard: Vec::new(),
        actuate_joint: vec![
            ActuateJointType {
                name: None,
                joint_number: 0,
                joint_position: 1.57,
                joint_details: None,
            },
            ActuateJointType {
                name: None,
                joint_number: 1,
                joint_position: -0.785,
                joint_details: Some(JointDetails::SpeedAccel(JointSpeedAccelType {
                    name: None,
                    joint_speed: Some(2.0),
                    joint_accel: Some(1.0),
                })),
            },
        ],
        joint_tolerances: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&original).expect("Failed to serialize");
    println!("Mixed None and Some joint_details:\n{}\n", json);

    // Deserialize back
    let parsed: ActuateJointsType = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(parsed.command_id, 456);
    assert_eq!(parsed.actuate_joint.len(), 2);

    // First joint should have None
    assert_eq!(parsed.actuate_joint[0].joint_number, 0);
    assert_eq!(parsed.actuate_joint[0].joint_position, 1.57);
    assert!(parsed.actuate_joint[0].joint_details.is_none());

    // Second joint should have SpeedAccel
    assert_eq!(parsed.actuate_joint[1].joint_number, 1);
    assert_eq!(parsed.actuate_joint[1].joint_position, -0.785);
    match &parsed.actuate_joint[1].joint_details {
        Some(JointDetails::SpeedAccel(details)) => {
            assert_eq!(details.joint_speed, Some(2.0));
            assert_eq!(details.joint_accel, Some(1.0));
        }
        _ => panic!("Expected SpeedAccel variant"),
    }
}
