use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ActuateJointType {
    pub name: Option<IdType>,
    pub joint_number: IntType,
    pub joint_position: DoubleType,
    pub joint_details: JointDetailsType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ActuateJointsType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub actuate_joint: Vec<ActuateJointType>,
    pub joint_tolerances: Option<JointPositionsTolerancesType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AngleUnitEnumType {
    Degree,
    Radian,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CrclCommandType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CloseToolChangerType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigureJointReportType {
    pub name: Option<IdType>,
    pub joint_number: IntType,
    pub report_position: BooleanType,
    pub report_torque_or_force: BooleanType,
    pub report_velocity: BooleanType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigureJointReportsType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub reset_all: BooleanType,
    pub configure_joint_report: Vec<ConfigureJointReportType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigureStatusReportType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub report_joint_statuses: BooleanType,
    pub report_pose_status: BooleanType,
    pub report_gripper_status: BooleanType,
    pub report_settings_status: BooleanType,
    pub report_sensors_status: BooleanType,
    pub report_guards_status: BooleanType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DataThingType {
    pub name: Option<IdType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DisableGripperType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub gripper_name: NmtokenType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DisableRobotParameterStatusType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub robot_parameter_name: TokenType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DisableSensorType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub sensor_id: TokenType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DwellType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub dwell_time: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EnableGripperType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub gripper_name: NmtokenType,
    pub gripper_option: Vec<ParameterSettingType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EnableRobotParameterStatusType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub robot_parameter_name: TokenType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EnableSensorType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub sensor_id: TokenType,
    pub sensor_option: Vec<ParameterSettingType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EndCanonType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ForceUnitEnumType {
    Newton,
    Pound,
    Ounce,
}
pub type FractionType = f64;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStatusType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum GuardLimitEnumType {
    OverMax,
    UnderMin,
    IncreaseOverLimit,
    DecreaseBeyondLimit,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GuardType {
    pub name: Option<IdType>,
    pub sensor_id: TokenType,
    pub sub_field: Option<TokenType>,
    pub limit_type: GuardLimitEnumType,
    pub limit_value: DoubleType,
    pub recheck_time_micro_seconds: Option<LongType>,
    pub check_count: Option<LongType>,
    pub last_check_time: Option<LongType>,
    pub last_check_value: Option<DoubleType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InitCanonType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JointDetailsType {
    pub name: Option<IdType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JointForceTorqueType {
    pub name: Option<IdType>,
    pub setting: Option<DoubleType>,
    pub change_rate: Option<DoubleType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JointPositionToleranceSettingType {
    pub name: Option<IdType>,
    pub joint_number: IntType,
    pub joint_position_tolerance: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JointPositionsTolerancesType {
    pub name: Option<IdType>,
    pub content_6: JointPositionsTolerancesContent6Type,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JointSpeedAccelType {
    pub name: Option<IdType>,
    pub joint_speed: Option<DoubleType>,
    pub joint_accel: Option<DoubleType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum LengthUnitEnumType {
    Meter,
    Millimeter,
    Inch,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub message: StringType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MiddleCommandType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveScrewType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub start_position: Option<PoseType>,
    pub axis_point: Option<PointType>,
    pub axial_distance_free: Option<DoubleType>,
    pub axial_distance_screw: DoubleType,
    pub turn: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveThroughToType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub move_straight: BooleanType,
    pub waypoint: Vec<PoseType>,
    pub num_positions: IntType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveToType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub move_straight: BooleanType,
    pub end_position: PoseType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenToolChangerType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterSettingType {
    pub name: Option<IdType>,
    pub parameter_name: TokenType,
    pub parameter_value: TokenType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PointType {
    pub name: Option<IdType>,
    pub x: DoubleType,
    pub y: DoubleType,
    pub z: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PoseAndSetType {
    pub name: Option<IdType>,
    pub point: PointType,
    pub x_axis: VectorType,
    pub z_axis: VectorType,
    pub coordinated: BooleanType,
    pub trans_speed: Option<TransSpeedType>,
    pub rot_speed: Option<RotSpeedType>,
    pub trans_accel: Option<TransAccelType>,
    pub rot_accel: Option<RotAccelType>,
    pub tolerance: Option<PoseToleranceType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PoseToleranceType {
    pub name: Option<IdType>,
    pub x_point_tolerance: Option<DoubleType>,
    pub y_point_tolerance: Option<DoubleType>,
    pub z_point_tolerance: Option<DoubleType>,
    pub x_axis_tolerance: Option<DoubleType>,
    pub z_axis_tolerance: Option<DoubleType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PoseType {
    pub name: Option<IdType>,
    pub point: PointType,
    pub x_axis: VectorType,
    pub z_axis: VectorType,
}
pub type PositiveDecimalType = f64;

#[derive(Debug, Serialize, Deserialize)]
pub struct RotAccelAbsoluteType {
    pub name: Option<IdType>,
    pub setting: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RotAccelRelativeType {
    pub name: Option<IdType>,
    pub fraction: FractionType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RotAccelType {
    pub name: Option<IdType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RotSpeedAbsoluteType {
    pub name: Option<IdType>,
    pub setting: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RotSpeedRelativeType {
    pub name: Option<IdType>,
    pub fraction: FractionType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RotSpeedType {
    pub name: Option<IdType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RunProgramType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub program_text: StringType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetAngleUnitsType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub unit_name: AngleUnitEnumType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetDefaultJointPositonsTolerancesType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub joint_tolerances: JointPositionsTolerancesType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetEndEffectorParametersType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub parameter_setting: Vec<ParameterSettingType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetEndEffectorType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub setting: FractionType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetEndPoseToleranceType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub tolerance: PoseToleranceType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetForceUnitsType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub unit_name: ForceUnitEnumType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetIntermediatePoseToleranceType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub tolerance: PoseToleranceType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetLengthUnitsType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub unit_name: LengthUnitEnumType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetMotionCoordinationType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub coordinated: BooleanType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetRobotParametersType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub parameter_setting: Vec<ParameterSettingType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetRotAccelType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub rot_accel: RotAccelType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetRotSpeedType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub rot_speed: RotSpeedType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetTorqueUnitsType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub unit_name: TorqueUnitEnumType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetTransAccelType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub trans_accel: TransAccelType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SetTransSpeedType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub trans_speed: TransSpeedType,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum StopConditionEnumType {
    Immediate,
    Fast,
    Normal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StopMotionType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub guard: Vec<GuardType>,
    pub stop_condition: StopConditionEnumType,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TorqueUnitEnumType {
    NewtonMeter,
    FootPound,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransAccelAbsoluteType {
    pub name: Option<IdType>,
    pub setting: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransAccelRelativeType {
    pub name: Option<IdType>,
    pub fraction: FractionType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransAccelType {
    pub name: Option<IdType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransSpeedAbsoluteType {
    pub name: Option<IdType>,
    pub setting: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransSpeedRelativeType {
    pub name: Option<IdType>,
    pub fraction: FractionType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransSpeedType {
    pub name: Option<IdType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TwistType {
    pub name: Option<IdType>,
    pub linear_velocity: VectorType,
    pub angular_velocity: VectorType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VectorType {
    pub name: Option<IdType>,
    pub i: DoubleType,
    pub j: DoubleType,
    pub k: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WrenchType {
    pub name: Option<IdType>,
    pub force: VectorType,
    pub moment: VectorType,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntitiesType(pub Vec<String>);

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntityType(pub Vec<String>);
pub type IdType = String;
pub type IdrefType = String;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IdrefsType(pub Vec<String>);
pub type NcNameType = String;
pub type NmtokenType = String;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NmtokensType(pub Vec<String>);
pub type NotationType = String;
pub type NameType = String;
pub type QNameType = String;
pub type AnySimpleType = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnyType;
pub type AnyUriType = String;
pub type Base64BinaryType = String;
pub type BooleanType = bool;
pub type ByteType = i8;
pub type DateType = String;
pub type DateTimeType = String;
pub type DecimalType = f64;
pub type DoubleType = f64;
pub type DurationType = String;
pub type FloatType = f32;
pub type GDayType = String;
pub type GMonthType = String;
pub type GMonthDayType = String;
pub type GYearType = String;
pub type GYearMonthType = String;
pub type HexBinaryType = String;
pub type IntType = i32;
pub type IntegerType = i32;
pub type LanguageType = String;
pub type LongType = i64;
pub type NegativeIntegerType = isize;
pub type NonNegativeIntegerType = usize;
pub type NonPositiveIntegerType = isize;
pub type NormalizedStringType = String;
pub type PositiveIntegerType = usize;
pub type ShortType = i16;
pub type StringType = String;
pub type TimeType = String;
pub type TokenType = String;
pub type UnsignedByteType = u8;
pub type UnsignedIntType = u32;
pub type UnsignedLongType = u64;
pub type UnsignedShortType = u16;

#[derive(Debug, Serialize, Deserialize)]
pub struct JointPositionsTolerancesContent6Type {
    pub setting: Vec<JointPositionToleranceSettingType>,
}
