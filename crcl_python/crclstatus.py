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
    RotAccelAbsoluteType,
    RotAccelRelativeType,
    RotSpeedAbsoluteType,
    RotSpeedRelativeType,
    TorqueUnitEnumType,
    TransAccelAbsoluteType,
    TransAccelRelativeType,
    TransSpeedAbsoluteType,
    TransSpeedRelativeType,
    TwistType,
    WrenchType,
)


class CommandStateEnumType(Enum):
    """CommandStateEnumType enumerates the command states that may be used to
    describe command status.

    CRCL_Done means that the most recent command is done. CRCL_Error
    means that the most recent command resulted in an error of some
    sort. CRCL_Working means that the most recent command is being
    executed, and no error has occurred so far, but execution of the
    command is not yet done. CRCL_Ready means that the robot is ready to
    receive commands but has not yet received a command.
    """

    CRCL_DONE = "CRCL_Done"
    CRCL_ERROR = "CRCL_Error"
    CRCL_WORKING = "CRCL_Working"
    CRCL_READY = "CRCL_Ready"


@dataclass
class CommandStatusType(DataThingType):
    """CommandStatusType is derived from DataThingType.

    An instance of CommandStatusType has the following elements:
    Name (inherited, optional)
    CommandID
    StatusID
    CommandState
    StateDescription (optional)
    ProgramFile (optional)
    ProgramIndex (optional)
    ProgramLength (optional).
    The CommandStatusType relates the execution status of the
    currently executing command (or the most recently executed
    command, if there is no current command).
    CommandID echoes the command id from the received command to
    which the status message applies
    StatusID is an ID associated with this particular status
    message.
    StateDescription is an optional brief description of the state
    such as "Joint 3 at -171.0 less than limit -170.0" or
    "Waiting for Operator".
    ProgramFile provides an optional reference if the currently executing
    command is known to have come from a particular file.
    ProgramIndex provoides an optional reference to the element within a
    program. If the currently executing command is known to have come
    from a particular file. The InitCanon command will have index 0,
    and first MiddleCommand will have index 1.
    ProgramLength is the number of commands in the current program if
    known.
    The combination of StatusID and CommandID must be unique
    within a session.
    """

    command_id: Optional[int] = field(
        default=None,
        metadata={
            "name": "CommandID",
            "type": "Element",
            "required": True,
        },
    )
    status_id: Optional[int] = field(
        default=None,
        metadata={
            "name": "StatusID",
            "type": "Element",
            "required": True,
        },
    )
    command_state: Optional[CommandStateEnumType] = field(
        default=None,
        metadata={
            "name": "CommandState",
            "type": "Element",
            "required": True,
        },
    )
    state_description: Optional[str] = field(
        default=None,
        metadata={
            "name": "StateDescription",
            "type": "Element",
        },
    )
    program_file: Optional[str] = field(
        default=None,
        metadata={
            "name": "ProgramFile",
            "type": "Element",
        },
    )
    program_index: Optional[int] = field(
        default=None,
        metadata={
            "name": "ProgramIndex",
            "type": "Element",
        },
    )
    program_length: Optional[int] = field(
        default=None,
        metadata={
            "name": "ProgramLength",
            "type": "Element",
        },
    )
    override_percent: Optional[int] = field(
        default=None,
        metadata={
            "name": "OverridePercent",
            "type": "Element",
        },
    )


@dataclass
class GripperStatusType(DataThingType):
    """The abstract GripperStatusType is derived from DataThingType.

    An instance of GripperStatusType has the following elements:
    Name (inherited, optional)
    GripperName
    HoldingObject (optional).
    GripperStatusType is an abstract type from which more specialized
    types of gripper status are derived. HoldingObject is true if the
    gripper is expected to be holding an object given its position and/or
    pressure sensors on the finger tips.
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
    holding_object: Optional[bool] = field(
        default=None,
        metadata={
            "name": "HoldingObject",
            "type": "Element",
        },
    )


@dataclass
class GuardsStatusesType(DataThingType):
    """GuardsStatusesType is derived from DataThingType.

    An instance of GuardsStatusesType has the following elements:
    Name (inherited, optional)
    GuardsStatus (multiple).
    Each GuardsStatus element gives the status of one sensor. A robot
    may be associated with any number of internal or external sensors.
    Any custom named internal variable could also be reported with
    the interface.
    """

    guard: list[GuardType] = field(
        default_factory=list,
        metadata={
            "name": "Guard",
            "type": "Element",
        },
    )
    trigger_count: Optional[int] = field(
        default=None,
        metadata={
            "name": "TriggerCount",
            "type": "Element",
            "required": True,
        },
    )
    trigger_stop_time_micros: Optional[int] = field(
        default=None,
        metadata={
            "name": "TriggerStopTimeMicros",
            "type": "Element",
            "required": True,
        },
    )
    trigger_value: Optional[float] = field(
        default=None,
        metadata={
            "name": "TriggerValue",
            "type": "Element",
        },
    )
    trigger_pose: Optional[PoseType] = field(
        default=None,
        metadata={
            "name": "TriggerPose",
            "type": "Element",
        },
    )


@dataclass
class JointLimitType(DataThingType):
    """JointLimitType is derived from DataThingType.

    JointLimitType reports the limits of one joint.
    An instance of JointStatusType has the following elements:
    Name (inherited, optional)
    JointNumber
    JointMinPosition (optional)
    JointMaxPosition (optional)
    JointMaxTorqueOrForce (optional)
    JointMaxVelocity (optional).
    """

    joint_number: Optional[int] = field(
        default=None,
        metadata={
            "name": "JointNumber",
            "type": "Element",
            "required": True,
        },
    )
    joint_min_position: Optional[float] = field(
        default=None,
        metadata={
            "name": "JointMinPosition",
            "type": "Element",
        },
    )
    joint_max_position: Optional[float] = field(
        default=None,
        metadata={
            "name": "JointMaxPosition",
            "type": "Element",
        },
    )
    joint_max_torque_or_force: Optional[float] = field(
        default=None,
        metadata={
            "name": "JointMaxTorqueOrForce",
            "type": "Element",
        },
    )
    joint_max_velocity: Optional[float] = field(
        default=None,
        metadata={
            "name": "JointMaxVelocity",
            "type": "Element",
        },
    )


@dataclass
class JointStatusType(DataThingType):
    """JointStatusType is derived from DataThingType.

    JointStatusType reports the status of one joint.
    An instance of JointStatusType has the following elements:
    Name (inherited, optional)
    JointNumber
    JointPosition (optional)
    JointTorqueOrForce (optional)
    JointVelocity (optional).
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
        },
    )
    joint_torque_or_force: Optional[float] = field(
        default=None,
        metadata={
            "name": "JointTorqueOrForce",
            "type": "Element",
        },
    )
    joint_velocity: Optional[float] = field(
        default=None,
        metadata={
            "name": "JointVelocity",
            "type": "Element",
        },
    )


@dataclass
class PoseStatusType(DataThingType):
    """PoseStatusType is derived from DataThingType.

    An instance of PoseStatusType has the following elements:
    Name (inherited, optional)
    Pose
    Twist (optional)
    Wrench (optional)
    Configuration (optional).
    PoseStatusType provides a Cartesian counterpart to
    JointStatusesType, representing the generalized
    position/orientation, velocities, and forces.
    of a reference frame.
    Configuration provides a robot specific description of the current
    configuration flags and turns.
    """

    pose: Optional[PoseType] = field(
        default=None,
        metadata={
            "name": "Pose",
            "type": "Element",
            "required": True,
        },
    )
    twist: Optional[TwistType] = field(
        default=None,
        metadata={
            "name": "Twist",
            "type": "Element",
        },
    )
    wrench: Optional[WrenchType] = field(
        default=None,
        metadata={
            "name": "Wrench",
            "type": "Element",
        },
    )
    configuration: Optional[str] = field(
        default=None,
        metadata={
            "name": "Configuration",
            "type": "Element",
        },
    )


@dataclass
class SensorStatusType(DataThingType):
    """SensorStatusType is derived from DataThingType.

    SensorStatusType reports the status of one sensor.
    An instance of SensorStatusType has the following elements:
    Name (inherited, optional)
    SensorID
    ReadCount
    LastReadTime
    SensorParameterSetting (optional).
    SensorID should be unique and unlikely to change within a system such as hardware model and serial number or
    a statically set IP or hostname. Read count returns the number of times
    the sensor has been read or a frame number if available.  LastReadTime is in milliseconds
    since 1970 (aka unix time).
    """

    sensor_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "SensorID",
            "type": "Element",
            "required": True,
        },
    )
    read_count: Optional[int] = field(
        default=None,
        metadata={
            "name": "ReadCount",
            "type": "Element",
            "required": True,
        },
    )
    last_read_time: Optional[int] = field(
        default=None,
        metadata={
            "name": "LastReadTime",
            "type": "Element",
            "required": True,
        },
    )
    sensor_parameter_setting: list[ParameterSettingType] = field(
        default_factory=list,
        metadata={
            "name": "SensorParameterSetting",
            "type": "Element",
        },
    )


@dataclass
class CountSensorStatusType(SensorStatusType):
    """CountSensorStatusType is derived from SensorStatusType CountSensorStatusType
    reports the status of a counting/integer sensor.

    An instance of CountSensorStatusType has the following elements:
    Name (inherited, optional)
    SensorID (inherited)
    SensorParameterSetting (inherited, optional)
    CountValue.
    """

    count_value: Optional[int] = field(
        default=None,
        metadata={
            "name": "CountValue.",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class ForceTorqueSensorStatusType(SensorStatusType):
    """SensorStatusType is derived from DataThingType.

    SensorStatusType reports the status of one force/torque sensor.
    Forces ar in Newtons, Torque in Newton-meters.
    An instance of SensorStatusType has the following elements:
    Name (inherited, optional)
    SensorID (inherited)
    SensorParameterSetting (inherited, optional)
    Fx
    Fy
    Fz
    Tx
    Ty
    Tz.
    """

    fx: Optional[float] = field(
        default=None,
        metadata={
            "name": "Fx",
            "type": "Element",
            "required": True,
        },
    )
    fy: Optional[float] = field(
        default=None,
        metadata={
            "name": "Fy",
            "type": "Element",
            "required": True,
        },
    )
    fz: Optional[float] = field(
        default=None,
        metadata={
            "name": "Fz",
            "type": "Element",
            "required": True,
        },
    )
    tx: Optional[float] = field(
        default=None,
        metadata={
            "name": "Tx",
            "type": "Element",
            "required": True,
        },
    )
    ty: Optional[float] = field(
        default=None,
        metadata={
            "name": "Ty",
            "type": "Element",
            "required": True,
        },
    )
    tz: Optional[float] = field(
        default=None,
        metadata={
            "name": "Tz",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class JointStatusesType(DataThingType):
    """JointStatusesType is derived from DataThingType.

    An instance of JointStatusesType has the following elements:
    Name (inherited, optional)
    JointStatus (multiple).
    Each JointStatus element gives the status of one joint. No
    joint may be reported more than once in an instance of
    JointStatusesType. See notes at the beginning of this file
    regarding configuring joint status.
    """

    joint_status: list[JointStatusType] = field(
        default_factory=list,
        metadata={
            "name": "JointStatus",
            "type": "Element",
            "min_occurs": 1,
        },
    )


@dataclass
class OnOffSensorStatusType(SensorStatusType):
    """SensorStatusType is derived from SensorStatusType SensorStatusType reports
    the status of one on/off sensor.

    An instance of SensorStatusType has the following elements:
    Name (inherited, optional)
    SensorID (inherited)
    SensorParameterSetting (inherited, optional)
    On.
    """

    on: Optional[bool] = field(
        default=None,
        metadata={
            "name": "On",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class ParallelGripperStatusType(GripperStatusType):
    """ "ParallelGripperStatusType is derived from GripperStatusType.

    An instance of ParallelGripperStatusType has the following elements:
    Name (inherited, optional)
    GripperName (inherited)
    Separation.
    ParallelGripperStatusType gives gripper status for a parallel
    jaw gripper. The Separation element gives the distance between
    the jaws in length units.
    """

    separation: Optional[float] = field(
        default=None,
        metadata={
            "name": "Separation",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class ScalarSensorStatusType(SensorStatusType):
    """SensorStatusType is derived from DataThingType.

    SensorStatusType reports the status of scalar/analog sensor.
    An instance of ScalarSensorStatusType has the following elements:
    Name (inherited, optional)
    SensorID (inherited)
    ScalarValue.
    """

    scalar_value: Optional[float] = field(
        default=None,
        metadata={
            "name": "ScalarValue.",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SettingsStatusType(DataThingType):
    """SettingsStatusType is derived from DataThingType.

    It provides the values
    echoed back from the appropriate command to set that parameter. It might
    also provide the initial default value read from a configuration file or
    from a lower level controller on startup if no command has yet been
    given.
    An instance of SettingsStatusType has the following elements:
    Name (inherited, optional)
    AngleUnitName (optional)
    EndEffectorParameterSetting (optional)
    EndEffectorSetting (optional)
    ForceUnitName (optional)
    IntermediatePoseTolerance (optional)
    JointLimits (optional)
    LengthUnitName (optional)
    MaxCartesianLimit (optional)
    MinCartesianLimit (optional)
    MotionCoordinated (optional)
    PoseTolerance (optional)
    RobotParameterSetting (optional)
    RotAccelAbsolute (optional)
    RotAccelRelative (optional)
    RotSpeedAbsolute (optional)
    RotSpeedRelative (optional)
    TorqueUnitName (optional)
    TransAccelAbsolute (optional)
    TransAccelRelative (optional)
    TransSpeedAbsolute (optional)
    TransSpeedRelative (optional).
    AngleUnitName is a string that can be only the literals 'radian' or
    'degree'. This tells the robot that all further commands
    giving angle values will implicitly use the named unit.
    EndEffectorParameterSetting is for setting parameters of end
    effectors that have parameters. The meaning of the parameter
    settings is not part of CRCL. It is expected that this command will
    be used only to send parameter values that can be used by the end
    effector currently in use.
    EndEffectorSetting is for setting the effectivity of end effectors.
    If an end effector has multiple control modes, the control mode
    must be set using a SetEndEffectorParameters command, so that the
    meaning of SetEndEffector commands is unambiguous. For end effectors
    that have a continuously variable setting, the Setting means a
    fraction of maximum openness, force, torque, power, etc. For end
    effectors that have only two choices (powered or unpowered, open or
    closed, on or off), a positive Setting value means powered, open,
    or on, while a zero Setting value means unpowered, closed, or off.
    ForceUnitName is a string that can be only the literals 'newton',
    'pound', or 'ounce'. This tells the robot that all further commands
    giving force values will implicitly use the named unit.
    JointLimits represents a list of different possible limits associated
    with each joint. These limits can not be directly set through CRCL.
    IntermediatePoseTolerance indicates to the robot the precision with
    which it must reach each intermediate waypoint.
    LengthUnitName is a string that can be only the literals 'meter',
    'millimeter', or 'inch'. This tells the robot that all further
    commands giving position or length values will implicitly use the
    named unit.
    MaxCartesianLimit is the point with greatest X,Y, and Z values that can
    be reached without violating a configured cartesian limit. It can no
    be directly changed through CRCL.
    MinCartesianLimit is the point with lowest X,Y, and Z values that can
    be reached without violating a configured cartesian limit. It can no
    be directly changed through CRCL.
    MotionCoordinated is a boolean. If the value is true, rotational and
    translational motion must finish simultaneously in motion commands
    (including each segment in a multiple segment motion command),
    except as possibly temporarily overridden in the the motion
    command. If the value is false, there is no such requirement.
    PoseTolerance indicates to the robot the precision with
    which it must reach its end location.
    RobotParameterSetting is for setting robot parameters that
    cannot be set by any other CRCL command. The meaning of the
    parameter settings is not part of CRCL.
    RotAccelAbsolute represents the target single axis
    rotational acceleration for the robot, in current angle units per
    second per second.
    RotAccelRelative represents the fraction of the
    robot's maximum rotational acceleration that it should use.
    RotSpeedAbsolute represents the target single axis
    rotational speed for the robot, in current angle units per
    second.
    RotSpeedRelative represents the fraction of the
    robot's maximum rotational speed that it should use.
    TorqueUnitName is a string that can be only the literals 'newtonMeter'
    or 'footPound'. This tells the robot that all further commands
    giving torque values will implicitly use the named unit.
    TransAccelAbsolute represents the translational acceleration for the
    controlled point, in current length units per second per second.
    TransAccelRelative represents the fraction of the
    robot's maximum translational acceleration that it should use.
    TransSpeedAbsolute represents the translational speed for the
    controlled point, in current length units per second.
    TransSpeedRelative represents the fraction of the
    robot's maximum translational speed that it should use.
    """

    angle_unit_name: Optional[AngleUnitEnumType] = field(
        default=None,
        metadata={
            "name": "AngleUnitName",
            "type": "Element",
        },
    )
    end_effector_parameter_setting: list[ParameterSettingType] = field(
        default_factory=list,
        metadata={
            "name": "EndEffectorParameterSetting",
            "type": "Element",
        },
    )
    end_effector_setting: Optional[float] = field(
        default=None,
        metadata={
            "name": "EndEffectorSetting",
            "type": "Element",
            "min_inclusive": 0.0,
            "max_inclusive": 1.0,
        },
    )
    force_unit_name: Optional[ForceUnitEnumType] = field(
        default=None,
        metadata={
            "name": "ForceUnitName",
            "type": "Element",
        },
    )
    joint_limits: list[JointLimitType] = field(
        default_factory=list,
        metadata={
            "name": "JointLimits",
            "type": "Element",
        },
    )
    intermediate_pose_tolerance: Optional[PoseToleranceType] = field(
        default=None,
        metadata={
            "name": "IntermediatePoseTolerance",
            "type": "Element",
        },
    )
    length_unit_name: Optional[LengthUnitEnumType] = field(
        default=None,
        metadata={
            "name": "LengthUnitName",
            "type": "Element",
        },
    )
    max_cartesian_limit: Optional[PointType] = field(
        default=None,
        metadata={
            "name": "MaxCartesianLimit",
            "type": "Element",
        },
    )
    min_cartesian_limit: Optional[PointType] = field(
        default=None,
        metadata={
            "name": "MinCartesianLimit",
            "type": "Element",
        },
    )
    motion_coordinated: Optional[bool] = field(
        default=None,
        metadata={
            "name": "MotionCoordinated",
            "type": "Element",
        },
    )
    end_pose_tolerance: Optional[PoseToleranceType] = field(
        default=None,
        metadata={
            "name": "EndPoseTolerance",
            "type": "Element",
        },
    )
    robot_parameter_setting: list[ParameterSettingType] = field(
        default_factory=list,
        metadata={
            "name": "RobotParameterSetting",
            "type": "Element",
        },
    )
    rot_accel_absolute: Optional[RotAccelAbsoluteType] = field(
        default=None,
        metadata={
            "name": "RotAccelAbsolute",
            "type": "Element",
        },
    )
    rot_accel_relative: Optional[RotAccelRelativeType] = field(
        default=None,
        metadata={
            "name": "RotAccelRelative",
            "type": "Element",
        },
    )
    rot_speed_absolute: Optional[RotSpeedAbsoluteType] = field(
        default=None,
        metadata={
            "name": "RotSpeedAbsolute",
            "type": "Element",
        },
    )
    rot_speed_relative: Optional[RotSpeedRelativeType] = field(
        default=None,
        metadata={
            "name": "RotSpeedRelative",
            "type": "Element",
        },
    )
    torque_unit_name: Optional[TorqueUnitEnumType] = field(
        default=None,
        metadata={
            "name": "TorqueUnitName",
            "type": "Element",
        },
    )
    trans_accel_absolute: Optional[TransAccelAbsoluteType] = field(
        default=None,
        metadata={
            "name": "TransAccelAbsolute",
            "type": "Element",
        },
    )
    trans_accel_relative: Optional[TransAccelRelativeType] = field(
        default=None,
        metadata={
            "name": "TransAccelRelative",
            "type": "Element",
        },
    )
    trans_speed_absolute: Optional[TransSpeedAbsoluteType] = field(
        default=None,
        metadata={
            "name": "TransSpeedAbsolute",
            "type": "Element",
        },
    )
    trans_speed_relative: Optional[TransSpeedRelativeType] = field(
        default=None,
        metadata={
            "name": "TransSpeedRelative",
            "type": "Element",
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
class ThreeFingerGripperStatusType(GripperStatusType):
    """ThreeFingerGripperStatusType is derived from GripperStatusType.

    An instance of ThreeFingerGripperStatusType has the following elements:
    Name (inherited, optional)
    GripperName (inherited)
    Finger1Position (optional)
    Finger2Position (optional)
    Finger3Position (optional)
    Finger1Force (optional)
    Finger2Force (optional)
    Finger3Force (optional).
    ThreeFingerGripperStatusType gives gripper status for a three
    finger gripper. The fingers are assumed to be non-articulated.
    Finger position is 0.0 at fully closed and 1.0 at fully open and
    linear in either angle or distance for rotating fingers or
    sliding fingers, respectively. All elements are optional, but
    typically all three positions will be used if any one of
    them is used, and similarly for the three forces.
    Force units are as set by the most recent SetForceUnits command.
    The system sending CRCL commands and the system executing them
    must agree on which fingers are Finger1, Finger2,and Finger3.
    """

    finger1_position: Optional[float] = field(
        default=None,
        metadata={
            "name": "Finger1Position",
            "type": "Element",
            "min_inclusive": 0.0,
            "max_inclusive": 1.0,
        },
    )
    finger2_position: Optional[float] = field(
        default=None,
        metadata={
            "name": "Finger2Position",
            "type": "Element",
            "min_inclusive": 0.0,
            "max_inclusive": 1.0,
        },
    )
    finger3_position: Optional[float] = field(
        default=None,
        metadata={
            "name": "Finger3Position",
            "type": "Element",
            "min_inclusive": 0.0,
            "max_inclusive": 1.0,
        },
    )
    finger1_force: Optional[float] = field(
        default=None,
        metadata={
            "name": "Finger1Force",
            "type": "Element",
        },
    )
    finger2_force: Optional[float] = field(
        default=None,
        metadata={
            "name": "Finger2Force",
            "type": "Element",
        },
    )
    finger3_force: Optional[float] = field(
        default=None,
        metadata={
            "name": "Finger3Force",
            "type": "Element",
        },
    )


@dataclass
class VacuumGripperStatusType(GripperStatusType):
    """VacuumGripperStatusType is derived from GripperStatusType.

    An instance of VacuumGripperStatusType has the following elements:
    Name (inherited, optional)
    GripperName (inherited)
    IsPowered.
    VacuumGripperStatusType gives gripper status for a vacuum
    gripper. The IsPowered element is true if a vacuum is being
    applied and is false if not.
    """

    is_powered: Optional[bool] = field(
        default=None,
        metadata={
            "name": "IsPowered",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class SensorStatusesType(DataThingType):
    """SensorStatusesType is derived from DataThingType.

    An instance of SensorStatusesType has the following elements:
    Name (inherited, optional)
    SensorStatus (multiple).
    Each SensorStatus element gives the status of one sensor. A robot
    may be associated with any number of internal or external sensors.
    Any custom named internal variable could also be reported with
    the interface.
    """

    on_off_sensor_status: list[OnOffSensorStatusType] = field(
        default_factory=list,
        metadata={
            "name": "OnOffSensorStatus",
            "type": "Element",
        },
    )
    scalar_sensor_status: list[ScalarSensorStatusType] = field(
        default_factory=list,
        metadata={
            "name": "ScalarSensorStatus",
            "type": "Element",
        },
    )
    count_sensor_status: list[CountSensorStatusType] = field(
        default_factory=list,
        metadata={
            "name": "CountSensorStatus",
            "type": "Element",
        },
    )
    force_torque_sensor_status: list[ForceTorqueSensorStatusType] = field(
        default_factory=list,
        metadata={
            "name": "ForceTorqueSensorStatus",
            "type": "Element",
        },
    )


@dataclass
class CrclstatusType(DataThingType):
    """CRCLStatusType is derived from DataThingType.

    An instance of CRCLStatusType has the following elements:
    Name (inherited, optional)
    CommandStatus
    JointStatuses (optional)
    PoseStatus (optional)
    GripperStatus (optional)
    SettingsStatus (optional).
    Status is returned periodically by the controller.
    See notes at the beginning of this file regarding configuring
    CRCL status messages.
    GripperStatus should not be reported when there is no gripper
    and should be reported when there is a gripper.
    The coordinate system in which the Pose is reported is always
    robot coordinates.
    If CRCL status is being reported on separate channels for both
    a robot and a gripper, the status reported on the robot
    channel should include a Pose, while the status reported on the
    gripper channel should not include a Pose.
    """

    class Meta:
        name = "CRCLStatusType"

    command_status: Optional[CommandStatusType] = field(
        default=None,
        metadata={
            "name": "CommandStatus",
            "type": "Element",
            "required": True,
        },
    )
    joint_statuses: Optional[JointStatusesType] = field(
        default=None,
        metadata={
            "name": "JointStatuses",
            "type": "Element",
        },
    )
    pose_status: Optional[PoseStatusType] = field(
        default=None,
        metadata={
            "name": "PoseStatus",
            "type": "Element",
        },
    )
    gripper_status: Optional[GripperStatusType] = field(
        default=None,
        metadata={
            "name": "GripperStatus",
            "type": "Element",
        },
    )
    settings_status: Optional[SettingsStatusType] = field(
        default=None,
        metadata={
            "name": "SettingsStatus",
            "type": "Element",
        },
    )
    sensor_statuses: Optional[SensorStatusesType] = field(
        default=None,
        metadata={
            "name": "SensorStatuses",
            "type": "Element",
        },
    )
    guards_statuses: Optional[GuardsStatusesType] = field(
        default=None,
        metadata={
            "name": "GuardsStatuses",
            "type": "Element",
        },
    )


@dataclass
class Crclstatus(CrclstatusType):
    """
    Root element.
    """

    class Meta:
        name = "CRCLStatus"
