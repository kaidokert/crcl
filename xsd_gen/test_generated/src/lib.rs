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
        let content = ActuateJointsTypeContent {
            name: None,
            command_id: 123i64,
            guard: Vec::new(),
            actuate_joint: Vec::new(),
            joint_tolerances: None,
        };

        let actuate_joints = ActuateJointsType { content };

        assert_eq!(actuate_joints.content.command_id, 123i64);
    }

    #[test]
    fn test_can_create_joint_statuses() {
        use crcl_status::*;

        // Create a single joint status for joint 0
        let joint0 = JointStatusType {
            content: JointStatusTypeContent {
                name: None,  // Optional XML identifier - typically not needed
                joint_number: 0,
                joint_position: Some(1.57),  // radians
                joint_torque_or_force: Some(10.5),  // N or Nm
                joint_velocity: Some(0.5),  // rad/s
            }
        };

        // Create another joint status for joint 1
        let joint1 = JointStatusType {
            content: JointStatusTypeContent {
                name: None,
                joint_number: 1,
                joint_position: Some(-0.785),
                joint_torque_or_force: Some(8.2),
                joint_velocity: Some(0.3),
            }
        };

        // Combine into a JointStatusesType
        let all_joints = JointStatusesType {
            content: JointStatusesTypeContent {
                name: None,  // Optional XML identifier
                joint_status: vec![joint0, joint1],
            }
        };

        assert_eq!(all_joints.content.joint_status.len(), 2);
        assert_eq!(all_joints.content.joint_status[0].content.joint_number, 0);
        assert_eq!(all_joints.content.joint_status[0].content.joint_position, Some(1.57));
        assert_eq!(all_joints.content.joint_status[1].content.joint_number, 1);
    }

    #[test]
    fn test_can_create_data_primitives() {
        use data_primitives::*;

        // Test that we can actually instantiate the types
        let point_content = PointTypeContent {
            name: None,
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let point = PointType { content: point_content };

        assert_eq!(point.content.x, 1.0);
        assert_eq!(point.content.y, 2.0);
        assert_eq!(point.content.z, 3.0);
    }
}
