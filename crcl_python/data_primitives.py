from dataclasses import dataclass, field
from enum import Enum
from typing import Optional


class AngleUnitEnumType(Enum):
    """AngleUnitEnumType enumerates angle units.

    This might be used, for example, in a command that sets angle units.
    """

    DEGREE = "degree"
    RADIAN = "radian"


@dataclass
class DataThingType:
    """An instance of DataThingType has the following elements:
    Name (optional)
    .
    DataThingType is an abstract type from which more specific types
    of data thing are derived. That includes all complex data
    types such as Vector, PoseType, etc."""

    name: Optional[str] = field(
        default=None,
        metadata={
            "name": "Name",
            "type": "Element",
        },
    )


class ForceUnitEnumType(Enum):
    """ForceUnitEnumType enumerates force units.

    This might be used, for example, in a command that sets force units.
    """

    NEWTON = "newton"
    POUND = "pound"
    OUNCE = "ounce"


class GuardLimitEnumType(Enum):
    """GuardLimitEnumType enumerates the kinds of command guard limits.

    OVER_MAX means to value exceeded a fixed limit. UNDER_MIN means the
    value is less than a fixed minimum. INCREASE_OVER_LIMIT means the
    value increased from the starting value by more than the limit.
    DECREASE_BEYOND_LIMIT means the value decrease from the starting
    value by more than the limit.
    """

    OVER_MAX = "OVER_MAX"
    UNDER_MIN = "UNDER_MIN"
    INCREASE_OVER_LIMIT = "INCREASE_OVER_LIMIT"
    DECREASE_BEYOND_LIMIT = "DECREASE_BEYOND_LIMIT"


class LengthUnitEnumType(Enum):
    """LengthUnitEnumType enumerates length units.

    This might be used, for example, in a command that sets length
    units.
    """

    METER = "meter"
    MILLIMETER = "millimeter"
    INCH = "inch"


class TorqueUnitEnumType(Enum):
    """TorqueUnitEnumType enumerates torque units.

    This might be used, for example in a command that sets torque units.
    """

    NEWTON_METER = "newtonMeter"
    FOOT_POUND = "footPound"


@dataclass
class GuardType(DataThingType):
    """The GuardType is derived from DataThingType.

    An instance of GuardType has the following
    elements:
    Name (inherited, optional)
    SensorID
    SubField (optional)
    LimitType
    LimitValue
    RecheckTimeMicroSeconds
    CheckCount
    LastCheckTime
    LastCheckValue
    RequireTrigger
    ErrorOnTrigger
    A GuardType can be added to any command to indicate the
    command should be aborted if the value of the sensors subfield
    increases by at-least the MaxIncrease value if specified OR
    decreases by at-least the MaxDecrease value if specified OR
    is greater than or equal to the MaxValue if specified OR
    is less than or equal to the MinValue if specified.
    If the RecheckTimeMicroSeconds is specified the gaurd the sensor
    will be read to determine if the command must be aborted at that
    frequency and will otherwise check at the default rate for that
    sensor. Hardware and operating system limitations may require the
    sensor be checked less frequently.
    CheckCount, LastCheckTime and LastCheckValue are only ignored when
    inside a command. They may be optionaly set when included in status.
    CheckCount is the number of times the guard has been checked. LastCheckTime is
    the time in milliseconds since 1970 (aka unix-time).
    If RequireTrigger is specified and true, then if the command completes
    without ever detecting a trigger the state will be set to CRCL_ERROR.
    If ErrorOnTrigger is specified and true, then after the guard is triggered
    the state will be set to CRCL_ERROR.
    """

    sensor_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "SensorID",
            "type": "Element",
            "required": True,
        },
    )
    sub_field: Optional[str] = field(
        default=None,
        metadata={
            "name": "SubField",
            "type": "Element",
        },
    )
    limit_type: Optional[GuardLimitEnumType] = field(
        default=None,
        metadata={
            "name": "LimitType",
            "type": "Element",
            "required": True,
        },
    )
    limit_value: Optional[float] = field(
        default=None,
        metadata={
            "name": "LimitValue",
            "type": "Element",
            "required": True,
        },
    )
    recheck_time_micro_seconds: Optional[int] = field(
        default=None,
        metadata={
            "name": "RecheckTimeMicroSeconds",
            "type": "Element",
        },
    )
    check_count: Optional[int] = field(
        default=None,
        metadata={
            "name": "CheckCount",
            "type": "Element",
        },
    )
    last_check_time: Optional[int] = field(
        default=None,
        metadata={
            "name": "LastCheckTime",
            "type": "Element",
        },
    )
    last_check_value: Optional[float] = field(
        default=None,
        metadata={
            "name": "LastCheckValue",
            "type": "Element",
        },
    )
    require_trigger: Optional[bool] = field(
        default=None,
        metadata={
            "name": "RequireTrigger",
            "type": "Element",
        },
    )
    error_on_trigger: Optional[bool] = field(
        default=None,
        metadata={
            "name": "ErrorOnTrigger",
            "type": "Element",
        },
    )


@dataclass
class JointPositionToleranceSettingType(DataThingType):
    """JointPositionToleranceType is derived from DataThingType.

    An instance of JointPositionToleranceType has the following elements:
    Name (inherited, optional)
    JointNumber
    JointPositionTolerance
    Used to specify how close to a given joint position given with
    an actuate joints command the joint with the given jointnumber
    must be to to the goal position before the command may
    be considered done.
    """

    joint_number: Optional[int] = field(
        default=None,
        metadata={
            "name": "JointNumber",
            "type": "Element",
            "required": True,
        },
    )
    joint_position_tolerance: Optional[float] = field(
        default=None,
        metadata={
            "name": "JointPositionTolerance",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class ParameterSettingType(DataThingType):
    """ParameterSettingType is derived from DataThingType.

    An instance of ParameterSettingType has the following elements:
    Name (inherited, optional)
    ParameterName
    ParameterValue.
    ParameterSettingType is used to set values of parameters. The
    ParameterName and ParameterValue are both strings. The
    ParameterValue string may represent a data type known to the
    receiving system.
    """

    parameter_name: Optional[str] = field(
        default=None,
        metadata={
            "name": "ParameterName",
            "type": "Element",
            "required": True,
        },
    )
    parameter_value: Optional[str] = field(
        default=None,
        metadata={
            "name": "ParameterValue",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class PointType(DataThingType):
    """PointType is derived from DataThingType.

    An instance of PointType has the following elements:
    Name (inherited, optional)
    X
    Y
    Z.
    X, Y, and Z are the Cartesian coordinates of the Point.
    """

    x: Optional[float] = field(
        default=None,
        metadata={
            "name": "X",
            "type": "Element",
            "required": True,
        },
    )
    y: Optional[float] = field(
        default=None,
        metadata={
            "name": "Y",
            "type": "Element",
            "required": True,
        },
    )
    z: Optional[float] = field(
        default=None,
        metadata={
            "name": "Z",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class PoseToleranceType(DataThingType):
    """PoseToleranceType is derived from DataThingType.

    An instance of PoseToleranceType has the following elements:
    Name (inherited, optional)
    XPointTolerance (optional)
    YPointTolerance (optional)
    ZPointTolerance (optional)
    XAxisTolerance (optional)
    ZAxisTolerance (optional).
    The XPointTolerance is the distance along the XAxis in current
    length units within which the controlled point must come from the X
    value of the point given in the pose with which the pose tolerance
    is associated. The YPointTolerance and ZPointTolerance are similar.
    The XAxisTolerance is the angle in current angle units within which
    the XAxis must come from the given XAxis. The ZAxisTolerance is
    similar. All five tolerances must be satisfied at the same instant.
    """

    xpoint_tolerance: Optional[float] = field(
        default=None,
        metadata={
            "name": "XPointTolerance",
            "type": "Element",
        },
    )
    ypoint_tolerance: Optional[float] = field(
        default=None,
        metadata={
            "name": "YPointTolerance",
            "type": "Element",
        },
    )
    zpoint_tolerance: Optional[float] = field(
        default=None,
        metadata={
            "name": "ZPointTolerance",
            "type": "Element",
        },
    )
    xaxis_tolerance: Optional[float] = field(
        default=None,
        metadata={
            "name": "XAxisTolerance",
            "type": "Element",
        },
    )
    zaxis_tolerance: Optional[float] = field(
        default=None,
        metadata={
            "name": "ZAxisTolerance",
            "type": "Element",
        },
    )


@dataclass
class RotAccelType(DataThingType):
    """The abstract RotAccelType is derived from DataThingType.

    An instance of RotAccelType has the following
    elements:
    Name (inherited, optional)
    .
    RotAccelType is an abstract type used as the parent type of:
    RotAccelAbsoluteType
    RotAccelRelativeType.
    """


@dataclass
class RotSpeedType(DataThingType):
    """The abstract RotSpeedType is derived from DataThingType.

    An instance of RotSpeedType has the following
    elements:
    Name (inherited, optional)
    .
    RotSpeedType is an abstract type used as the parent type of:
    RotSpeedAbsoluteType
    RotSpeedRelativeType.
    """


@dataclass
class TransAccelType(DataThingType):
    """The abstract TransAccelType is derived from DataThingType.

    An instance of TransAccelType has the following elements:
    Name (inherited, optional)
    .
    TransAccelType is an abstract type used as the parent type of:
    TransAccelAbsoluteType
    TransAccelRelativeType.
    """


@dataclass
class TransSpeedType(DataThingType):
    """The abstract TransSpeedType is derived from DataThingType.

    An instance of TransSpeedType has the following
    elements:
    Name (inherited, optional)
    .
    TransSpeedType is an abstract type used as the parent type of:
    TransSpeedAbsoluteType
    TransSpeedRelativeType.
    """


@dataclass
class VectorType(DataThingType):
    """VectorType is derived from DataThingType.

    An instance of VectorType has the following elements:
    Name (inherited, optional)
    I
    J
    K.
    I, J, and K represent the usual i, j, and k components of a 3D
    vector.
    """

    i: Optional[float] = field(
        default=None,
        metadata={
            "name": "I",
            "type": "Element",
            "required": True,
        },
    )
    j: Optional[float] = field(
        default=None,
        metadata={
            "name": "J",
            "type": "Element",
            "required": True,
        },
    )
    k: Optional[float] = field(
        default=None,
        metadata={
            "name": "K",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class JointPositionsTolerancesType(DataThingType):
    """JointPositionsTolerancesType is derived from DataThingType.

    An instance of JointPositionsTolerancesType has the following elements:
    Name (inherited, optional)
    JointTolerances
    Used to specify how close to a given joint position given with
    an actuate joints command the joint with the given jointnumber
    must be to to the goal position before the command may
    be considered done.
    """

    setting: list[JointPositionToleranceSettingType] = field(
        default_factory=list,
        metadata={
            "name": "Setting",
            "type": "Element",
            "min_occurs": 1,
        },
    )


@dataclass
class PoseType(DataThingType):
    """PoseType is derived from DataThingType.

    An instance of PoseType has the following elements:
    Name (inherited, optional)
    Point
    XAxis
    ZAxis.
    The Point locates the origin of a coordinate system. The XAxis and
    ZAxis give the orientation of the coordinate system. The data for
    the Point, the ZAxis and the XAxis are expressed relative to another
    coordinate system.
    """

    point: Optional[PointType] = field(
        default=None,
        metadata={
            "name": "Point",
            "type": "Element",
            "required": True,
        },
    )
    xaxis: Optional[VectorType] = field(
        default=None,
        metadata={
            "name": "XAxis",
            "type": "Element",
            "required": True,
        },
    )
    zaxis: Optional[VectorType] = field(
        default=None,
        metadata={
            "name": "ZAxis",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class RotAccelAbsoluteType(RotAccelType):
    """RotAccelAbsoluteType is derived from RotAccelType.

    An instance of RotAccelAbsoluteType has the following elements:
    Name (inherited, optional)
    Setting.
    Setting is a real number that represents the target single axis
    rotational acceleration for the robot, in current angle units per
    second per second.
    """

    setting: Optional[float] = field(
        default=None,
        metadata={
            "name": "Setting",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class RotAccelRelativeType(RotAccelType):
    """RotAccelRelativeType is derived from RotAccelType.

    An instance of RotAccelRelativeType has the following elements:
    Name (inherited, optional)
    Fraction.
    Fraction is a real number that represents the fraction of the
    robot's maximum rotational acceleration that it should use.
    """

    fraction: Optional[float] = field(
        default=None,
        metadata={
            "name": "Fraction",
            "type": "Element",
            "required": True,
            "min_inclusive": 0.0,
            "max_inclusive": 1.0,
        },
    )


@dataclass
class RotSpeedAbsoluteType(RotSpeedType):
    """RotSpeedAbsoluteType is derived from RotSpeedType.

    An instance of RotSpeedAbsoluteType has the following
    elements:
    Name (inherited, optional)
    Setting.
    Setting is a real number that represents the target single axis
    rotational speed for the robot, in current angle units per second.
    """

    setting: Optional[float] = field(
        default=None,
        metadata={
            "name": "Setting",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class RotSpeedRelativeType(RotSpeedType):
    """RotSpeedRelativeType is derived from RotSpeedType.

    An instance of RotSpeedRelativeType has the following elements:
    Name (inherited, optional)
    Fraction.
    Fraction is a real number that represents the fraction of the
    robot's maximum rotational speed that it should use.
    """

    fraction: Optional[float] = field(
        default=None,
        metadata={
            "name": "Fraction",
            "type": "Element",
            "required": True,
            "min_inclusive": 0.0,
            "max_inclusive": 1.0,
        },
    )


@dataclass
class TransAccelAbsoluteType(TransAccelType):
    """TransAccelAbsoluteType is derived from TransAccelType.

    An instance of TransAccelAbsoluteType has the following
    elements:
    Name (inherited, optional)
    Setting.
    Setting is a real number that represents the target
    translational acceleration for the controlled point, in
    current length units per second per second.
    """

    setting: Optional[float] = field(
        default=None,
        metadata={
            "name": "Setting",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class TransAccelRelativeType(TransAccelType):
    """TransAccelRelativeType is derived from TransAccelType.

    An instance of TransAccelRelativeType has the following
    elements:
    Name (inherited, optional)
    Fraction.
    Fraction is a real number that represents the fraction of the
    robot's maximum translational acceleration that it should use.
    """

    fraction: Optional[float] = field(
        default=None,
        metadata={
            "name": "Fraction",
            "type": "Element",
            "required": True,
            "min_inclusive": 0.0,
            "max_inclusive": 1.0,
        },
    )


@dataclass
class TransSpeedAbsoluteType(TransSpeedType):
    """TransSpeedAbsoluteType is derived from TransSpeedType.

    An instance of TransSpeedAbsoluteType has the following
    elements:
    Name (inherited, optional)
    Setting.
    Setting is a real number that represents the target speed for the
    controlled point, in current length units per second.
    """

    setting: Optional[float] = field(
        default=None,
        metadata={
            "name": "Setting",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class TransSpeedRelativeType(TransSpeedType):
    """TransSpeedRelativeType is derived from TransSpeedType.

    An instance of TransSpeedRelativeType has the following elements:
    Name (inherited, optional)
    Fraction.
    Fraction is a real number that represents the fraction of the
    robot's maximum translational speed that it should use.
    """

    fraction: Optional[float] = field(
        default=None,
        metadata={
            "name": "Fraction",
            "type": "Element",
            "required": True,
            "min_inclusive": 0.0,
            "max_inclusive": 1.0,
        },
    )


@dataclass
class TwistType(DataThingType):
    """TwistType is derived from DataThingType.

    An instance of TwistType has the following elements:
    Name (inherited, optional)
    LinearVelocity
    AngularVelocity.
    A TwistType object represents the velocity of a
    rigid object in SE(3).
    """

    linear_velocity: Optional[VectorType] = field(
        default=None,
        metadata={
            "name": "LinearVelocity",
            "type": "Element",
            "required": True,
        },
    )
    angular_velocity: Optional[VectorType] = field(
        default=None,
        metadata={
            "name": "AngularVelocity",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class WrenchType(DataThingType):
    """WrenchType is derived from DataThingType.

    An instance of WrenchType has the following elements:
    Name (inherited, optional)
    Force
    Moment.
    A WrenchType object represents generalized forces and torques on a
    rigid object in SE(3).
    """

    force: Optional[VectorType] = field(
        default=None,
        metadata={
            "name": "Force",
            "type": "Element",
            "required": True,
        },
    )
    moment: Optional[VectorType] = field(
        default=None,
        metadata={
            "name": "Moment",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class PoseAndSetType(PoseType):
    """PoseAndSetType is derived from PoseType.

    An instance of PoseAndSetType has the following elements:
    Name (inherited, optional)
    Point (inherited)
    XAxis (inherited)
    ZAxis  (inherited)
    Coordinated
    TransSpeed (optional)
    RotSpeed (optional)
    TransAccel (optional)
    RotAccel (optional)
    Tolerance (optional).
    PoseAndSetType is used for waypoints of move commands. The
    TransSpeed and TransAccel elements are the target translational
    speed and acceleration for the controlled point as it moves to the
    given pose. The RotSpeed and RotAccel elements are the target
    rotational speed and acceleration for the single axis rotation
    required to move from the initial pose to the target pose. The
    Tolerance is the tolerance for the given pose. The TransSpeed,
    TransAccel, RotSpeed, RotAccel, and Tolerance temporarily override
    any set values. The set values apply again once the given pose is
    reached. If the Coordinated element is set to true, translation and
    rotation should finish simultaneously. If Coordinated is false,
    either translation or rotation may finish first.
    """

    coordinated: Optional[bool] = field(
        default=None,
        metadata={
            "name": "Coordinated",
            "type": "Element",
            "required": True,
        },
    )
    trans_speed: Optional[TransSpeedType] = field(
        default=None,
        metadata={
            "name": "TransSpeed",
            "type": "Element",
        },
    )
    rot_speed: Optional[RotSpeedType] = field(
        default=None,
        metadata={
            "name": "RotSpeed",
            "type": "Element",
        },
    )
    trans_accel: Optional[TransAccelType] = field(
        default=None,
        metadata={
            "name": "TransAccel",
            "type": "Element",
        },
    )
    rot_accel: Optional[RotAccelType] = field(
        default=None,
        metadata={
            "name": "RotAccel",
            "type": "Element",
        },
    )
    tolerance: Optional[PoseToleranceType] = field(
        default=None,
        metadata={
            "name": "Tolerance",
            "type": "Element",
        },
    )
