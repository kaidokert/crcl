### Summary

*   **ROS:** 110 structs
*   **CRCL:** 91 structs
*   **Total:** 201 structs

## High Confidence Mappings

| ROS Message | CRCL Struct | Notes |
|---|---|---|
| `Point` | `PointType` | Both represent a 3D point. |
| `Vector3` | `VectorType` | Both represent a 3D vector. |
| `Pose` | `PoseType` | Both represent a 3D pose. ROS uses a Point and a Quaternion, while CRCL uses a Point, an X-axis vector, and a Z-axis vector. |
| `Twist` | `TwistType` | Both represent linear and angular velocity. |
| `Wrench` | `WrenchType` | Both represent force and torque. |
| `JointState` | `JointStatusType` | Both represent the state of a single joint. ROS `JointState` is an array for multiple joints, while CRCL has `JointStatusesType` which contains multiple `JointStatusType` instances. |
| `WrenchStamped` | `ForceTorqueSensorStatusType` | Both report force and torque from a sensor. |
| `Bool` | `OnOffSensorStatusType` | For reporting binary sensor state. |
| `String` | `MessageType` | For sending text messages. |
| `Empty` | `InitCanonType`, `EndCanonType` | These are parameter-less commands, similar in spirit to `Empty`. |
| `JointTrajectoryPoint` | `ActuateJointType` | `ActuateJointType` defines a target joint position, similar to a point in a joint trajectory. |

## Loose Mappings

| ROS Message | CRCL Struct | Loose Mapping | Notes |
|---|---|---|---|
| `Transform` | - | `<PoseType>` | A ROS Transform is equivalent to a CRCL Pose, representing position and orientation. |
| `Temperature` | - | `<ScalarSensorStatusType>` | The `temperature` field can be mapped to `ScalarValue`. Variance is lost. |
| `RelativeHumidity` | - | `<ScalarSensorStatusType>` | The `relative_humidity` field can be mapped to `ScalarValue`. Variance is lost. |
| `Illuminance` | - | `<ScalarSensorStatusType>` | The `illuminance` field can be mapped to `ScalarValue`. Variance is lost. |
| `FluidPressure` | - | `<ScalarSensorStatusType>` | The `fluid_pressure` field can be mapped to `ScalarValue`. Variance is lost. |
| `Odometry` | `PoseStatusType` | | `PoseStatusType` contains the current pose and twist, which is a subset of the information in an Odometry message. |
| `Path` | `MoveThroughToType` | | Both define a path as a series of poses/waypoints. |
| `PoseStamped` (as goal) | `MoveToType` | | A command to move to a single designated pose. |
| `JointTrajectory` | - | `<ActuateJointsType>` | An `ActuateJointsType` command represents a single point in a `JointTrajectory`. A full trajectory would require a sequence of these commands. |
| `<URDF>` | `JointLimitType` | | In ROS, joint limits are typically defined in the robot's URDF file, not sent as messages. |
| `Accel` | `TransAccelType`, `RotAccelType` | | CRCL separates translational and rotational acceleration. |

## Unmapped

| ROS Message | CRCL Struct | Notes |
|---|---|---|
| `Inertia` | - | No CRCL equivalent. Could potentially be handled via `SetRobotParametersType`. |
| `TimeReference` | - | No CRCL equivalent. |
| `NavSatFix` | - | No direct CRCL equivalent. Could be mapped to three separate `ScalarSensorStatusType` messages. |
| `LaserScan` | - | No CRCL equivalent. CRCL lacks a type for array-based sensor data. |
| `PointCloud` / `PointCloud2` | - | No CRCL equivalent. CRCL lacks a type for array-based sensor data. |
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
| `Imu` | `<missing>` | No direct equivalent. Data could be sent via multiple `ScalarSensorStatusType` messages. |
| `AccelStamped` | - | Stamped version of Accel. |
| `AccelWithCovariance` | - | Accel with covariance. |
| `AccelWithCovarianceStamped` | - | Stamped version of AccelWithCovariance. |
| `InertiaStamped` | - | Stamped version of Inertia. |
| `Point32` | - | Point with 32-bit floats. |
| `PointStamped` | - | Stamped version of Point. |
| `Polygon` | - | A polygon with 32-bit float vertices. |
| `PolygonInstance` | - | A polygon with an ID. |
| `PolygonInstanceStamped` | - | Stamped version of PolygonInstance. |
| `PolygonStamped` | - | Stamped version of Polygon. |
| `PoseArray` | - | An array of Poses. |
| `PoseWithCovariance` | - | Pose with covariance. |
| `PoseWithCovarianceStamped` | - | Stamped version of PoseWithCovariance. |
| `QuaternionStamped` | - | Stamped version of Quaternion. |
| `TransformStamped` | - | Stamped version of Transform. |
| `TwistStamped` | - | Stamped version of Twist. |
| `TwistWithCovariance` | - | Twist with covariance. |
| `TwistWithCovarianceStamped` | - | Stamped version of TwistWithCovariance. |
| `Vector3Stamped` | - | Stamped version of Vector3. |
| `VelocityStamped` | - | Stamped version of Velocity (Twist). |
| `Goals` | - | A list of goals. |
| `GridCells` | - | Represents a grid of cells. |
| `MapMetaData` | - | Metadata for a map. |
| `OccupancyGrid` | - | Represents an occupancy grid. |
| `Trajectory` | - | A trajectory with points. |
| `TrajectoryPoint` | - | A point in a trajectory. |
| `BatteryState` | - | Represents the state of a battery. |
| `CameraInfo` | - | Information about a camera. |
| `ChannelFloat32` | - | A channel of 32-bit floats. |
| `CompressedImage` | - | A compressed image. |
| `Image` | - | An image. |
| `Joy` | - | Represents a joystick. |
| `JoyFeedback` | - | Feedback for a joystick. |
| `JoyFeedbackArray` | - | An array of joystick feedback. |
| `LaserEcho` | - | An echo from a laser. |
| `MagneticField` | - | Represents a magnetic field. |
| `MultiDOFJointState` | - | State of a multi-DOF joint. |
| `MultiEchoLaserScan` | - | A laser scan with multiple echoes. |
| `NavSatStatus` | - | Status of a navigation satellite. |
| `PointField` | - | A field in a point cloud. |
| `Range` | - | Represents a range measurement. |
| `RegionOfInterest` | - | A region of interest in an image. |
| `Mesh` | - | A 3D mesh. |
| `MeshTriangle` | - | A triangle in a mesh. |
| `Plane` | - | A plane. |
| `SolidPrimitive` | - | A solid primitive shape. |
| `ColorRGBA` | - | An RGBA color. |
| `MultiArrayDimension` | - | A dimension of a multi-array. |
| `MultiArrayLayout` | - | The layout of a multi-array. |
| `MultiDOFJointTrajectory` | - | A trajectory for multi-DOF joints. |
| `MultiDOFJointTrajectoryPoint` | - | A point in a multi-DOF joint trajectory. |
| - | `ConfigureJointReportsType` | Command to configure joint reports. |
| - | `MoveScrewType` | Command to move along a screw axis. |
| - | `OpenGripperType` | Command to open the gripper. |
| - | `CloseGripperType` | Command to close the gripper. |
| - | `SetAngleUnitsType` | Command to set angle units. |
| - | `SetEndEffectorType` | Command to set the end effector. |
| - | `SetEndEffectorParametersType` | Command to set end effector parameters. |
| - | `SetForceUnitsType` | Command to set force units. |
| - | `SetLengthUnitsType` | Command to set length units. |
| - | `SetMotionCoordinationType` | Command to set motion coordination. |
| - | `SetRobotParametersType` | Command to set robot parameters. |
| - | `SetTorqueUnitsType` | Command to set torque units. |
| - | `SettingsStatusType` | Status of various settings. |
| - | `CountSensorStatusType` | Status of a counting sensor. |
