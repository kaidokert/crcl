// Include the generated code to test if it compiles

#[allow(dead_code)]
mod data_primitives {
    include!("../../generated_data_primitives.rs");
}

#[allow(dead_code)]
mod crcl_commands {
    include!("../../generated_crcl_commands_merged.rs");
}

#[allow(dead_code)]
mod crcl_status {
    include!("../../generated_crcl_status_merged.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_actuate_joints() {
        use crcl_commands::*;

        // Test that we can actually instantiate the types
        let actuate_joints = ActuateJointsType {
            name: None,
            command_id: 123i64,
            guard: Vec::new(),
            actuate_joint: Vec::new(),
            joint_tolerances: None,
        };

        assert_eq!(actuate_joints.command_id, 123i64);
    }

    #[test]
    fn test_can_create_joint_statuses() {
        use crcl_status::*;

        // Create a single joint status for joint 0
        let joint0 = JointStatusType {
            name: None,  // Optional XML identifier - typically not needed
            joint_number: 0,
            joint_position: Some(1.57),  // radians
            joint_torque_or_force: Some(10.5),  // N or Nm
            joint_velocity: Some(0.5),  // rad/s
        };

        // Create another joint status for joint 1
        let joint1 = JointStatusType {
            name: None,
            joint_number: 1,
            joint_position: Some(-0.785),
            joint_torque_or_force: Some(8.2),
            joint_velocity: Some(0.3),
        };

        // Combine into a JointStatusesType
        let all_joints = JointStatusesType {
            name: None,  // Optional XML identifier
            joint_status: vec![joint0, joint1],
        };

        assert_eq!(all_joints.joint_status.len(), 2);
        assert_eq!(all_joints.joint_status[0].joint_number, 0);
        assert_eq!(all_joints.joint_status[0].joint_position, Some(1.57));
        assert_eq!(all_joints.joint_status[1].joint_number, 1);
    }

    #[test]
    fn test_can_create_data_primitives() {
        use data_primitives::*;

        // Test that we can actually instantiate the types
        let point = PointType {
            name: None,
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
    }

    #[test]
    fn test_serialize_joint_status_to_xml() {
        use crcl_status::*;

        let joint0 = JointStatusType {
            name: None,
            joint_number: 0,
            joint_position: Some(1.57),
            joint_torque_or_force: Some(10.5),
            joint_velocity: Some(0.5),
        };

        // Serialize to XML
        let xml = serde_xml_rs::to_string(&joint0).expect("Failed to serialize");

        assert!(xml.contains("JointStatusType"));
        assert!(xml.contains("<joint_number>0</joint_number>"));
        assert!(xml.contains("1.57"));
    }

    #[test]
    fn test_deserialize_joint_status_from_xml() {
        use crcl_status::*;

        let xml = r#"<?xml version="1.0"?>
<JointStatusType>
    <joint_number>2</joint_number>
    <joint_position>0.785</joint_position>
    <joint_torque_or_force>5.5</joint_torque_or_force>
    <joint_velocity>0.1</joint_velocity>
</JointStatusType>"#;

        let joint: JointStatusType = serde_xml_rs::from_str(xml)
            .expect("Failed to deserialize");

        assert_eq!(joint.joint_number, 2);
        assert_eq!(joint.joint_position, Some(0.785));
        assert_eq!(joint.joint_torque_or_force, Some(5.5));
        assert_eq!(joint.joint_velocity, Some(0.1));
    }

    #[test]
    fn test_serialize_actuate_joints() {
        use crcl_commands::*;

        let actuate_joints = ActuateJointsType {
            name: None,
            command_id: 123i64,
            guard: Vec::new(),
            actuate_joint: Vec::new(),
            joint_tolerances: None,
        };

        // Serialize to XML
        let xml = serde_xml_rs::to_string(&actuate_joints)
            .expect("Failed to serialize");

        assert!(xml.contains("ActuateJointsType"));
        assert!(xml.contains("<command_id>123</command_id>"));
    }
}
