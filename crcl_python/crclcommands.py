from dataclasses import dataclass, field
from enum import Enum
from typing import Optional

from crcl_python.data_primitives import (
    AngleUnitEnumType,
    DataThingType,
    ForceUnitEnumType,
    GuardType,
    JointPositionsTolerancesType,
    LengthUnitEnumType,
    ParameterSettingType,
    PointType,
    PoseToleranceType,
    PoseType,
    RotAccelType,
    RotSpeedType,
    TorqueUnitEnumType,
    TransAccelType,
    TransSpeedType,
)


class StopConditionEnumType(Enum):
    """The StopConditionEnumType enumerates types of commanded stops.

    Any physical devices or built-in control methods of the robot
    controller for stopping the robot are in addition to these commanded
    stop types. Immediate means the robot's drives are deactivated
    immediately and the brakes are applied. This may result in the
    controlled point being off the commanded path when the robot stops.
    Fast means the robot and any external axes are brought to a fast,
    controlled stop. The drives are deactivated after one second, and
    the brakes are applied. The controlled point must be kept on the on
    the commanded path as the robot stops. Normal means the robot and
    any external drives are stopped using a normal braking ramp. The
    drives are not deactivated, and the brakes are not applied. The
    controlled point must be kept on the on the commanded path as the
    robot stops.
    """

    IMMEDIATE = "Immediate"
    FAST = "Fast"
    NORMAL = "Normal"


@dataclass
class CrclcommandType(DataThingType):
    """The abstract CRCLCommandType is derived from DataThingType.

    An instance of CRCLCommandType has the following elements:
    Name (inherited, optional)
    CommandID.
    CRCLCommandType is an abstract type from which all other
    CRCL commands are derived.
    """

    class Meta:
        name = "CRCLCommandType"

    command_id: Optional[int] = field(
        default=None,
        metadata={
            "name": "CommandID",
            "type": "Element",
            "required": True,
        },
    )
    guard: list[GuardType] = field(
        default_factory=list,
        metadata={
            "name": "Guard",
            "type": "Element",
        },
    )


@dataclass
class ConfigureJointReportType(DataThingType):
    """ConfigureJointReportType is derived from DataThingType.

    An instance of ConfigureJointReportType has the following elements:
    Name (inherited, optional)
    JointNumber
    ReportPosition
    ReportTorqueOrForce
    ReportVelocity
    ConfigureJointReportType is used to specify whether and how status
    reporting should be done for the joint identified by its joint
    number. For each ReportXXX element, true means XXX data should be
    reported and false means XXX data should not be reported.
    """

    joint_number: Optional[int] = field(
        default=None,
        metadata={
            "name": "JointNumber",
            "type": "Element",
            "required": True,
        },
    )
    report_position: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ReportPosition",
            "type": "Element",
            "required": True,
        },
    )
    report_torque_or_force: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ReportTorqueOrForce",
            "type": "Element",
            "required": True,
        },
    )
    report_velocity: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ReportVelocity",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class JointDetailsType(DataThingType):
    """The abstract JointDetailsType is derived from DataThingType.

    An instance of JointDetailsType has the following elements:
    Name (inherited, optional)
    .
    JointDetailsType is an abstract type used as the parent type of:
    JointSpeedAccelType
    JointForceTorqueType
    """


@dataclass
class ActuateJointType(DataThingType):
    """ActuateJointType is derived from DataThingType.

    An instance of ActuateJointType has the following elements:
    Name (inherited, optional)
    JointNumber
    JointPosition
    JointDetails.
    JointPosition is the target position for the joint. JointDetails
    provides either (1) the speed and acceleration to use in getting to
    the position or (2) the force or torque and rate of change of force
    or torque to use in getting to the position.
    """

    joint_number: Optional[int] = field(
        default=None,
        metadata={
            "name": "JointNumber",
            "type": "Element",
            "required": True,
        },
    )
    joint_position: Optional[float] = field(
        default=None,
        metadata={
            "name": "JointPosition",
            "type": "Element",
            "required": True,
        },
    )
    joint_details: Optional[JointDetailsType] = field(
        default=None,
        metadata={
            "name": "JointDetails",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class EndCanonType(CrclcommandType):
    """EndCanonType is derived from CRCLCommandType.

    An instance of EndCanonType has the following elements:
    Name (inherited, optional)
    CommandID (inherited).
    An instance of EndCanonType is used to indicate that the robot
    should not execute any further CRCL commands other than an
    instance of InitCanonType until an InitCanonType command is
    received. Other robot-specific actions may be taken in
    preparation for shutting down.
    """


@dataclass
class InitCanonType(CrclcommandType):
    """InitCanonType is derived from CRCLCommandType.

    An instance of InitCanonType has the following elements:
    Name (inherited, optional)
    CommandID (inherited).
    An instance of InitCanonType is used to indicate that the robot
    should be prepared to execute further canonical robot commands.
    When a robot is ready to execute commands, the first CRCL command
    it should be sent is an instance of InitCanonType. Any CRCL
    commands received before an instance of InitCanonType must not be
    executed. Other robot-specific actions may be taken in preparation
    for executing CRCL commands.
    """


@dataclass
class JointForceTorqueType(JointDetailsType):
    """JointForceTorqueType is derived from JointDetailsType.

    An instance of JointForceTorqueType has the following elements:
    Name (inherited, optional)
    Setting (optional)
    ChangeRate (optional).
    JointForceTorqueType specifies the force or torque and the rate of
    change of force or torque for a joint. For a translational joint,
    Setting is in current force units, and ChangeRate is in current
    force units per second. For a rotational joint, Setting is in
    current torque units, and ChangeRate is in current torque units per
    second.
    """

    setting: Optional[float] = field(
        default=None,
        metadata={
            "name": "Setting",
            "type": "Element",
        },
    )
    change_rate: Optional[float] = field(
        default=None,
        metadata={
            "name": "ChangeRate",
            "type": "Element",
        },
    )


@dataclass
class JointSpeedAccelType(JointDetailsType):
    """JointSpeedAccelType is derived from JointDetailsType.

    An instance of JointSpeedAccelType has the following elements:
    Name (inherited, optional)
    JointSpeed (optional)
    JointAccel (optional).
    JointSpeedAccelType specifies the speed and acceleration for a
    joint. For a rotational joint, the speed units are the current
    angle units per second, and the acceleration units are the current
    angle units per second per second. For a translational joint, the
    speed units are the current length units per second, and the
    acceleration units are the current length units per second per
    second.
    """

    joint_speed: Optional[float] = field(
        default=None,
        metadata={
            "name": "JointSpeed",
            "type": "Element",
        },
    )
    joint_accel: Optional[float] = field(
        default=None,
        metadata={
            "name": "JointAccel",
            "type": "Element",
        },
    )


@dataclass
class MiddleCommandType(CrclcommandType):
    """The abstract MiddleCommandType is derived from CRCLCommandType.

    MiddleCommandType has  the following elements:
    Name (inherited, optional)
    CommandID (inherited).
    MiddleCommandType is the abstract parent type of specific CRCL
    command types. Only derived types of MiddleCommandType may be
    instantiated.
    """


@dataclass
class ActuateJointsType(MiddleCommandType):
    """ActuateJointsType is derived from MiddleCommandType.

    An instance of ActuateJointsType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    ActuateJoint (multiple).
    Each joint may appear in at most one ActuateJoint element. If
    a joint appears in no ActuateJoint element, its actuation should
    be as previously set.
    """

    actuate_joint: list[ActuateJointType] = field(
        default_factory=list,
        metadata={
            "name": "ActuateJoint",
            "type": "Element",
            "min_occurs": 1,
        },
    )
    joint_tolerances: Optional[JointPositionsTolerancesType] = field(
        default=None,
        metadata={
            "name": "JointTolerances",
            "type": "Element",
        },
    )


@dataclass
class CloseToolChangerType(MiddleCommandType):
    """CloseToolChangerType is derived from MiddleCommandType.

    An instance of CloseToolChangerType has the following elements:
    Name (inherited, optional)
    CommandID (inherited).
    After an instance of CloseToolChangerType is executed, it is
    understood that if the tool changer was in position to acquire an
    end effector, the end effector will be mounted on the robot. In
    that case, the controlled point will change.
    """


@dataclass
class ConfigureJointReportsType(MiddleCommandType):
    """ConfigureJointReportsType is derived from MiddleCommandType.

    An instance of ConfigureJointReportsType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    ResetAll
    ConfigureJointReport (multiple).
    ConfigureJointReportsType is used to specify how the status of the
    robot joints should be reported. The ConfigureJointReports command
    may be used more than once during a session to change joint status
    reporting.
    If ResetAll is set to true, an instance of
    ConfigureJointReportsType resets the joint reporting of all
    joints, and, in that case, if there is no ConfigureJointReport
    element for a joint in the instance, the status of that joint
    should not be reported. Thus, an instance of
    ConfigureJointReportsType with ResetAll set to true and no
    ConfigureJointReport elements turns off all joint reporting.
    If ResetAll is set to false, status reporting is changed for only
    those joints given in a ConfigureJointReport element. Status
    reporting for other joints remains the same.
    No joint may appear in more than one ConfigureJointReport element.
    Joint numbers in ConfigureJointReport elements must be given in
    increasing order.
    See the in-line documentation of CRCLStatus.xsd for further
    information.
    """

    reset_all: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ResetAll",
            "type": "Element",
            "required": True,
        },
    )
    configure_joint_report: list[ConfigureJointReportType] = field(
        default_factory=list,
        metadata={
            "name": "ConfigureJointReport",
            "type": "Element",
        },
    )


@dataclass
class ConfigureStatusReportType(MiddleCommandType):
    """ConfigureStatusReportType is derived from MiddleCommandType.

    An instance of ConfigureStatusReportType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    ReportJointStatuses
    ReportPoseStatus
    ReportGripperStatus
    ReportSettingsStatus
    ReportSensorsStatus
    ReportGuardsStatus.
    ConfigureStatusReportType is used to specify how the status of the
    robot should be reported.
    """

    report_joint_statuses: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ReportJointStatuses",
            "type": "Element",
            "required": True,
        },
    )
    report_pose_status: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ReportPoseStatus",
            "type": "Element",
            "required": True,
        },
    )
    report_gripper_status: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ReportGripperStatus",
            "type": "Element",
            "required": True,
        },
    )
    report_settings_status: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ReportSettingsStatus",
            "type": "Element",
            "required": True,
        },
    )
    report_sensors_status: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ReportSensorsStatus",
            "type": "Element",
            "required": True,
        },
    )
    report_guards_status: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ReportGuardsStatus",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class DisableGripperType(MiddleCommandType):
    """DisableGripperType is derived from MiddleCommandType.

    An instance of DisableGripperType has the following elements:
    Name (inherited, optional)`
    CommandID (inherited)
    GripperId.
    This disables the reporting of a sensor.
    """

    gripper_name: Optional[str] = field(
        default=None,
        metadata={
            "name": "GripperName",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class DisableRobotParameterStatusType(MiddleCommandType):
    """DisableRobotParameterStatusType is derived from MiddleCommandType.

    An instance of DisableRobotParameterStatusType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    RobotParameterId.
    This disables the reporting of a property in status robotProperties.
    """

    robot_parameter_name: Optional[str] = field(
        default=None,
        metadata={
            "name": "RobotParameterName",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class DisableSensorType(MiddleCommandType):
    """DisableSensorType is derived from MiddleCommandType.

    An instance of DisableSensorType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    SensorId.
    This disables the reporting of a sensor.
    """

    sensor_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "SensorID",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class DwellType(MiddleCommandType):
    """DwellType is derived from MiddleCommandType.

    An instance of DwellType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    DwellTime.
    The DwellTime is an amount of time, in seconds, that the robot
    should wait before executing the next command.
    """

    dwell_time: Optional[float] = field(
        default=None,
        metadata={
            "name": "DwellTime",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class EnableGripperType(MiddleCommandType):
    """EnableGripperType is derived from MiddleCommandType.

    An instance of EnableGripperType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    GripperId
    GripperOptions (optional).
    This enables the reporting of a sensor.
    """

    gripper_name: Optional[str] = field(
        default=None,
        metadata={
            "name": "GripperName",
            "type": "Element",
            "required": True,
        },
    )
    gripper_option: list[ParameterSettingType] = field(
        default_factory=list,
        metadata={
            "name": "GripperOption",
            "type": "Element",
        },
    )


@dataclass
class EnableRobotParameterStatusType(MiddleCommandType):
    """EnableRobotParameterStatusType is derived from MiddleCommandType.

    An instance of EnableRobotParameterStatusType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    EnableRobotParameterName.
    This enables the reporting of a property in status robotProperties.
    """

    robot_parameter_name: Optional[str] = field(
        default=None,
        metadata={
            "name": "RobotParameterName",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class EnableSensorType(MiddleCommandType):
    """EnableSensorType is derived from MiddleCommandType.

    An instance of EnableSensorType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    SensorId
    SensorOptions (optional).
    This enables the reporting of a sensor.
    """

    sensor_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "SensorID",
            "type": "Element",
            "required": True,
        },
    )
    sensor_option: list[ParameterSettingType] = field(
        default_factory=list,
        metadata={
            "name": "SensorOption",
            "type": "Element",
        },
    )


@dataclass
class GetStatusType(MiddleCommandType):
    """GetStatusType is derived from MiddleCommandType.

    An instance of GetStatusType has the following elements:
    Name (inherited, optional)
    CommandID (inherited).
    An instance of GetStatusType is used to indicate that the robot
    should report status immediately. The joint status portion of
    the status report must be as set by the most recent
    ConfigureJointReports command.
    """


@dataclass
class LoopBlockEndType(MiddleCommandType):
    """LoopBlockEndType is derived from MiddleCommandType.

    An instance of LoopBlockStartType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    LoopName
    MaxCount.
    This starts a simple loop to repeat a set of commands. It must be
    followed with a LoopBlockEndType with the same LoopName.
    LoopName values should be unique in a program. MaxCount provides
    the number of times the loop will be repeated.
    """

    loop_name: Optional[str] = field(
        default=None,
        metadata={
            "name": "LoopName",
            "type": "Element",
            "required": True,
        },
    )
    max_count: Optional[int] = field(
        default=None,
        metadata={
            "name": "MaxCount",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class LoopBlockStartType(MiddleCommandType):
    """LoopBlockStartType is derived from MiddleCommandType.

    An instance of LoopBlockStartType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    LoopName.
    This starts a simple loop to repeat a set of commands. It must be
    followed with a LoopBlockEndType with the same LoopName.
    LoopName values should be unique in a program.
    """

    loop_name: Optional[str] = field(
        default=None,
        metadata={
            "name": "LoopName",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class MessageType(MiddleCommandType):
    """MessageType is derived from MiddleCommandType.

    An instance of MessageType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    Message.
    Message is a string that should be displayed by the robot
    controller.
    """

    message: Optional[str] = field(
        default=None,
        metadata={
            "name": "Message",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class MoveScrewType(MiddleCommandType):
    """MoveScrewType is derived from MiddleCommandType.

    An instance of MoveScrewType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    StartPosition (optional)
    AxisPoint (optional)
    AxialDistanceFree (optional)
    AxialDistanceScrew
    Turn.
    This command is designed for attaching screws, nuts, and bolts.
    It might also be used for drilling.
    The command is executed as follows.
    First, if the StartPosition exists, the controlled point and axis
    are moved to the StartPosition along any convenient trajectory.
    Second, if the AxialDistanceFree exists and is not zero, the
    controlled point is moved along the axis by the given
    AxialDistanceFree.
    Third and finally, a screwing motion is made. If there is no
    AxisPoint (or if an AxisPoint is given that is at the controlled
    point), the gripper rotates around its axis through the angle given
    by Turn at a constant rate while simultaneously translating along
    the axis at a constant rate (the currently set speed) through the
    AxialDistanceScrew so that the rotation and translation finish at
    the same time. If there is an AxisPoint and it differs from the
    location of the controlled point, the controlled point
    simultaneously (1) rotates as above, (2) revolves around an axis
    through the AxisPoint parallel to the controlled axis, and (3)
    translates as above. That makes a helical motion of the controlled
    point. The motion along the helix is done at the currently set
    speed.
    A positive value of AxialDistanceFree or AxialDistanceScrew means
    to move away from the end effector. A negative value means to move
    toward the end effector.
    A positive value of Turn means to rotate (and possibly revolve) in
    a counterclockwise sense as viewed from the positive Z axis of the
    gripper (the region extending away from the gripper).
    The robot must reach the EndPosition within the tolerance
    established (1) by the tolerance given for the pose in the
    EndPosition, if there is a tolerance in the EndPosition, or if not
    (2) by the most recently executed instance of
    SetEndPoseToleranceType. The speed and acceleration to use are set
    either in the EndPosition or by previously executed CRCL commands.
    In an instance file, the type of StartPosition may be either
    PoseType or PoseAndSetType, which is derived from PoseType.
    """

    start_position: Optional[PoseType] = field(
        default=None,
        metadata={
            "name": "StartPosition",
            "type": "Element",
        },
    )
    axis_point: Optional[PointType] = field(
        default=None,
        metadata={
            "name": "AxisPoint",
            "type": "Element",
        },
    )
    axial_distance_free: Optional[float] = field(
        default=None,
        metadata={
            "name": "AxialDistanceFree",
            "type": "Element",
        },
    )
    axial_distance_screw: Optional[float] = field(
        default=None,
        metadata={
            "name": "AxialDistanceScrew",
            "type": "Element",
            "required": True,
        },
    )
    turn: Optional[float] = field(
        default=None,
        metadata={
            "name": "Turn",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class MoveThroughToType(MiddleCommandType):
    """MoveThroughToType is derived from MiddleCommandType.

    An instance of MoveThroughToType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    MoveStraight
    Waypoint (multiple)
    NumPositions.
    Each Waypoint before the last is a Pose that the robot should move
    through. The last Waypoint is the Pose the robot should be in after
    the command is fully executed. NumPositions is the number of
    instances of the Waypoint element. The robot must pass each point
    within the tolerance established (1) by the tolerance given for the
    pose in the Waypoint, if there is a tolerance in the Waypoint, or
    if not (2) by the most recently executed instance of
    SetIntermediatePoseToleranceType (or by SetEndPoseToleranceType for
    the final Waypoint). The speed and acceleration to use are set
    either in the Waypoint or by previously executed CRCL commands.
    If the value of MoveStraight is true, the controlled point must be
    moved in a straight line between Waypoints. If the value of
    MoveStraight is false, the controlled point may be moved along any
    convenient trajectory between Waypoints. In either case, there are
    no restrictions on the values of XAxis and ZAxis between waypoints.
    The type of each Waypoint may be either PoseType or
    PoseAndSetType, which is derived from PoseType.
    """

    move_straight: Optional[bool] = field(
        default=None,
        metadata={
            "name": "MoveStraight",
            "type": "Element",
            "required": True,
        },
    )
    waypoint: list[PoseType] = field(
        default_factory=list,
        metadata={
            "name": "Waypoint",
            "type": "Element",
            "min_occurs": 2,
        },
    )
    num_positions: Optional[int] = field(
        default=None,
        metadata={
            "name": "NumPositions",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class MoveToType(MiddleCommandType):
    """MoveToType is derived from MiddleCommandType.

    An instance of MoveToType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    MoveStraight
    EndPosition.
    EndPosition is a Pose to which the robot will move. If the value of
    MoveStraight is true, the controlled point must be moved in a
    straight line. If the value of MoveStraight is false, the
    controlled point may be moved along any convenient trajectory.
    The robot must reach the EndPosition within the tolerance
    established (1) by the tolerance given for the pose in the
    EndPosition, if there is a tolerance in the EndPosition, or if not
    (2) by the most recently executed instance of
    SetEndPoseToleranceType. The speed and acceleration to use are set
    either in the EndPosition or by previously executed CRCL commands.
    The type of EndPosition may be either PoseType or
    PoseAndSetType, which is derived from PoseType.
    """

    move_straight: Optional[bool] = field(
        default=None,
        metadata={
            "name": "MoveStraight",
            "type": "Element",
            "required": True,
        },
    )
    end_position: Optional[PoseType] = field(
        default=None,
        metadata={
            "name": "EndPosition",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class OpenToolChangerType(MiddleCommandType):
    """OpenToolChangerType is derived from MiddleCommandType.

    An instance of OpenToolChangerType has the following elements:
    Name (inherited, optional)
    CommandID (inherited).
    After an instance of OpenToolChangerType is executed, it is
    understood that if a gripper was mounted on the robot, the
    gripper is no longer mounted on the robot. In that case,
    the controlled point will change.
    """


@dataclass
class RunProgramType(MiddleCommandType):
    """RunProgramType is derived from MiddleCommandType.

    An instance of RunProgramType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    ProgramText.
    The RunProgramType is used to instruct the low level controller to
    run a program written in a non-CRCL language that controller
    understands. The ProgramText element gives the text of the program.
    """

    program_text: Optional[str] = field(
        default=None,
        metadata={
            "name": "ProgramText",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetAngleUnitsType(MiddleCommandType):
    """SetAngleUnits is derived from MiddleCommandType.

    An instance of SetAngleUnits has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    UnitName.
    UnitName is a string that can be only the literals 'radian' or
    'degree'. This tells the robot that all further commands
    giving angle values will implicitly use the named unit.
    """

    unit_name: Optional[AngleUnitEnumType] = field(
        default=None,
        metadata={
            "name": "UnitName",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetDefaultJointPositonsTolerancesType(MiddleCommandType):
    """SetDefaultJointsPositonToleranceType is derived from MiddleCommandType.

    An instance of SetDefaultJointsPositonToleranceType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    JointTolerances.
    The Tolerance element indicates to the robot the precision with
    which it must reach its end location for each joint for which
    an ActuateJoints command does not include a specific tolerance.
    It does not apply during force control or cartesian moves.
    """

    joint_tolerances: Optional[JointPositionsTolerancesType] = field(
        default=None,
        metadata={
            "name": "JointTolerances",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetEndEffectorParametersType(MiddleCommandType):
    """SetEndEffectorParametersType is derived from MiddleCommandType.

    An instance of SetEndEffectorParametersType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    ParameterSetting (multiple).
    SetEndEffectorParametersType is for setting parameters of end
    effectors that have parameters. The meaning of the parameter
    settings is not part of CRCL. It is expected that this command will
    be used only to send parameter values that can be used by the end
    effector currently in use.
    """

    parameter_setting: list[ParameterSettingType] = field(
        default_factory=list,
        metadata={
            "name": "ParameterSetting",
            "type": "Element",
            "min_occurs": 1,
        },
    )


@dataclass
class SetEndEffectorType(MiddleCommandType):
    """SetEndEffectorType is derived from MiddleCommandType.

    An instance of SetEndEffectorType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    Setting.
    SetEndEffectorType is for setting the effectivity of end effectors.
    If an end effector has multiple control modes, the control mode
    must be set using a SetEndEffectorParameters command, so that the
    meaning of SetEndEffector commands is unambiguous.
    For end effectors that have a continuously variable setting, the
    Setting means a fraction of maximum openness, force, torque, power,
    etc.
    For end effectors that have only two choices (powered or unpowered,
    open or closed, on or off), a positive Setting value means powered,
    open, or on, while a zero Setting value means unpowered, closed, or
    off.
    """

    setting: Optional[float] = field(
        default=None,
        metadata={
            "name": "Setting",
            "type": "Element",
            "required": True,
            "min_inclusive": 0.0,
            "max_inclusive": 1.0,
        },
    )


@dataclass
class SetEndPoseToleranceType(MiddleCommandType):
    """SetEndPoseToleranceType is derived from MiddleCommandType.

    An instance of SetEndPoseToleranceType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    Tolerance.
    The Tolerance element indicates to the robot the precision with
    which it must reach its end location.
    """

    tolerance: Optional[PoseToleranceType] = field(
        default=None,
        metadata={
            "name": "Tolerance",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetForceUnitsType(MiddleCommandType):
    """SetForceUnitsType is derived from MiddleCommandType.

    An instance of SetForceUnitsType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    UnitName.
    UnitName is a string that can be only the literals 'newton',
    'pound', or 'ounce'. This tells the robot that all further commands
    giving force values will implicitly use the named unit.
    """

    unit_name: Optional[ForceUnitEnumType] = field(
        default=None,
        metadata={
            "name": "UnitName",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetIntermediatePoseToleranceType(MiddleCommandType):
    """SetIntermediatePoseToleranceType is derived from MiddleCommandType.

    An instance of SetIntermediatePoseToleranceType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    Tolerance.
    The Tolerance element indicates to the robot the precision with
    which it must reach each intermediate waypoint.
    """

    tolerance: Optional[PoseToleranceType] = field(
        default=None,
        metadata={
            "name": "Tolerance",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetLengthUnitsType(MiddleCommandType):
    """SetLengthUnitsType is derived from MiddleCommandType.

    An instance of SetLengthUnitsType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    UnitName.
    UnitName is a string that can be only the literals 'meter',
    'millimeter', or 'inch'. This tells the robot that all further
    commands giving position or length values will implicitly use the
    named unit.
    """

    unit_name: Optional[LengthUnitEnumType] = field(
        default=None,
        metadata={
            "name": "UnitName",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetMotionCoordinationType(MiddleCommandType):
    """SetMotionCoordinationType is derived from MiddleCommandType.

    An instance of SetMotionCoordinationType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    Coordinated.
    Coordinated is a boolean. If the value is true, rotational and
    translational motion must finish simultaneously in motion commands
    (including each segment in a multiple segment motion command),
    except as possibly temporarily overridden in the the motion
    command. If the value is false, there is no such requirement.
    """

    coordinated: Optional[bool] = field(
        default=None,
        metadata={
            "name": "Coordinated",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetRobotParametersType(MiddleCommandType):
    """SetRobotParametersType is derived from MiddleCommandType.

    An instance of SetRobotParametersType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    ParameterSetting (multiple).
    SetRobotParametersType is for setting robot parameters that
    cannot be set by any other CRCL command. The meaning of the
    parameter settings is not part of CRCL.
    """

    parameter_setting: list[ParameterSettingType] = field(
        default_factory=list,
        metadata={
            "name": "ParameterSetting",
            "type": "Element",
            "min_occurs": 1,
        },
    )


@dataclass
class SetRotAccelType(MiddleCommandType):
    """SetRotAccelType is derived from MiddleCommandType.

    An instance of SetRotAccelType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    RotAccel.
    RotAccel specifies the rotational acceleration that should
    be used.
    """

    rot_accel: Optional[RotAccelType] = field(
        default=None,
        metadata={
            "name": "RotAccel",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetRotSpeedType(MiddleCommandType):
    """SetRotSpeedType is derived from MiddleCommandType.

    An instance of SetRotSpeedType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    RotSpeed.
    RotSpeed specifies the rotational speed that should be used.
    """

    rot_speed: Optional[RotSpeedType] = field(
        default=None,
        metadata={
            "name": "RotSpeed",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetTorqueUnitsType(MiddleCommandType):
    """SetTorqueUnitsType is derived from MiddleCommandType.

    An instance of SetTorqueUnitsType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    UnitName.
    UnitName is a string that can be only the literals 'newtonMeter'
    or 'footPound'. This tells the robot that all further commands
    giving torque values will implicitly use the named unit.
    """

    unit_name: Optional[TorqueUnitEnumType] = field(
        default=None,
        metadata={
            "name": "UnitName",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetTransAccelType(MiddleCommandType):
    """The SetTransAccelType is derived from MiddleCommandType.

    An instance of SetTransAccelType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    TransAccel.
    TransAccel specifies the translational acceleration that should
    be used.
    """

    trans_accel: Optional[TransAccelType] = field(
        default=None,
        metadata={
            "name": "TransAccel",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SetTransSpeedType(MiddleCommandType):
    """SetTransSpeedType is derived from MiddleCommandType.

    An instance of SetTransSpeedType has the following
    elements:
    Name (inherited, optional)
    CommandID (inherited)
    TransSpeed.
    TransSpeed specifies the translational speed that should be used.
    """

    trans_speed: Optional[TransSpeedType] = field(
        default=None,
        metadata={
            "name": "TransSpeed",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class StopMotionType(MiddleCommandType):
    """StopMotionType is derived from MiddleCommandType.

    An instance of StopMotionType has the following elements:
    Name (inherited, optional)
    CommandID (inherited)
    StopCondition.
    StopCondition is an enumerated value indicating how the stop
    should occur.
    """

    stop_condition: Optional[StopConditionEnumType] = field(
        default=None,
        metadata={
            "name": "StopCondition",
            "type": "Element",
            "required": True,
        },
    )
