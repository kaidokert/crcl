### Summary

*   **ROS:** 110 structs
*   **CRCL:** 91 structs
*   **Total:** 201 structs

| ROS Message | CRCL Struct | Notes |
|---|---|---|
| `Point` | `PointType` | Both represent a 3D point. |
| `Vector3` | `VectorType` | Both represent a 3D vector. |
| `Pose` | `PoseType` | Both represent a 3D pose. ROS uses a Point and a Quaternion, while CRCL uses a Point, an X-axis vector, and a Z-axis vector. |
| `Twist` | `TwistType` | Both represent linear and angular velocity. |
| `Wrench` | `WrenchType` | Both represent force and torque. |
| `JointState` | `JointStatusType` | Both represent the state of a single joint (position, velocity, effort/torque). ROS `JointState` is an array for multiple joints, while CRCL has `JointStatusesType` which is a container for `JointStatusType` instances. |
| `WrenchStamped` | `ForceTorqueSensorStatusType` | Both report force and torque from a sensor. |
| `Bool` | `OnOffSensorStatusType` | For reporting binary sensor state. |
| `<various>` | `ScalarSensorStatusType` | Reports a single float value from a sensor. Can be mapped to various ROS sensor messages like `Temperature`, `Illuminance`, etc. |
| `Odometry` | `PoseStatusType` | `PoseStatusType` contains the current pose and twist, which is a subset of the information in an Odometry message. |
| `Path` | `MoveThroughToType` | Both define a path as a series of poses/waypoints. |
| `PoseStamped` (as goal) | `MoveToType` | A command to move to a single designated pose. |
| `JointTrajectory` | `ActuateJointsType` | `ActuateJointsType` is a command to move multiple joints to a target position, which can be seen as a single point in a `JointTrajectory`. |
| `JointTrajectoryPoint` | `ActuateJointType` | `ActuateJointType` defines a target joint position, similar to a point in a joint trajectory. |
| `<URDF>` | `JointLimitType` | In ROS, joint limits are typically defined in the robot's URDF file, not sent as messages. |
| `Accel` | `TransAccelType`, `RotAccelType` | CRCL separates translational and rotational acceleration. |
| `String` | `MessageType` | For sending text messages. |
| `Empty` | `InitCanonType`, `EndCanonType` | These are parameter-less commands, similar in spirit to `Empty`. |
| `<missing>` | `CrclStatusType` | This is the top-level CRCL status message, a container for other statuses. No single ROS message is equivalent. |
| `<missing>` | `CommandStatusType` | Reports the status of a command (Done, Error, Working). ROS uses action servers or topics for this, not a specific message type. |
| `<missing>` | `GripperStatusType` | Gripper status is often handled by custom messages or action servers in ROS. |
| `<missing>` | `GuardType` | Represents a safety guard condition, no standard ROS equivalent. |
| `<missing>` | `DwellType` | A command to wait for a specified duration. |
| `<missing>` | `SetRotSpeedType`, `SetTransSpeedType` | Commands to set default speeds. In ROS, this is often a parameter of the motion planning library. |
| `<missing>` | `SetRotAccelType`, `SetTransAccelType` | Commands to set default accelerations. |
| `<missing>` | `StopMotionType` | A command to stop robot motion. |
| `Quaternion` | `<missing>` | CRCL uses two vectors (`XAxis`, `ZAxis`) for orientation instead of a quaternion. |
| `Header` | `<missing>` | CRCL commands have a `command_id` but no equivalent to the ROS header's timestamp or frame_id. |
| `Imu` | `<missing>` | CRCL does not have a specific IMU message, but the data could be represented using other sensor message types. |
| `PointCloud` | `<missing>` | CRCL does not have a specific message for point clouds. |
