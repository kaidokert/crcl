use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AngleUnitEnumType {
    Degree,
    Radian,
}
pub type CrclStatus = CrclStatusType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrclStatusType {
    pub name: Option<IdType>,
    pub command_status: CommandStatusType,
    pub joint_statuses: Option<JointStatusesType>,
    pub pose_status: Option<PoseStatusType>,
    pub gripper_status: Option<GripperStatusType>,
    pub settings_status: Option<SettingsStatusType>,
    pub sensor_statuses: Option<SensorStatusesType>,
    pub guards_statuses: Option<GuardsStatusesType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CommandStateEnumType {
    CrclDone,
    CrclError,
    CrclWorking,
    CrclReady,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandStatusType {
    pub name: Option<IdType>,
    pub command_id: LongType,
    pub status_id: LongType,
    pub command_state: CommandStateEnumType,
    pub state_description: Option<StringType>,
    pub program_file: Option<StringType>,
    pub program_index: Option<IntType>,
    pub program_length: Option<IntType>,
    pub override_percent: Option<IntType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CountSensorStatusType {
    pub name: Option<IdType>,
    pub sensor_id: TokenType,
    pub read_count: IntType,
    pub last_read_time: LongType,
    pub sensor_parameter_setting: Vec<ParameterSettingType>,
    pub count_value: IntType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DataThingType {
    pub name: Option<IdType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ForceTorqueSensorStatusType {
    pub name: Option<IdType>,
    pub sensor_id: TokenType,
    pub read_count: IntType,
    pub last_read_time: LongType,
    pub sensor_parameter_setting: Vec<ParameterSettingType>,
    pub fx: DoubleType,
    pub fy: DoubleType,
    pub fz: DoubleType,
    pub tx: DoubleType,
    pub ty: DoubleType,
    pub tz: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ForceUnitEnumType {
    Newton,
    Pound,
    Ounce,
}
pub type FractionType = f64;

#[derive(Debug, Serialize, Deserialize)]
pub struct GripperStatusType {
    pub name: Option<IdType>,
    pub gripper_name: NmtokenType,
    pub gripper_option: Vec<ParameterSettingType>,
    pub holding_object: Option<BooleanType>,
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
pub struct GuardsStatusesType {
    pub name: Option<IdType>,
    pub guard: Vec<GuardType>,
    pub trigger_count: IntType,
    pub trigger_stop_time_micros: LongType,
    pub trigger_value: Option<DoubleType>,
    pub trigger_pose: Option<PoseType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JointLimitType {
    pub name: Option<IdType>,
    pub joint_number: IntType,
    pub joint_min_position: Option<DoubleType>,
    pub joint_max_position: Option<DoubleType>,
    pub joint_max_torque_or_force: Option<DoubleType>,
    pub joint_max_velocity: Option<DoubleType>,
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
pub struct JointStatusType {
    pub name: Option<IdType>,
    pub joint_number: IntType,
    pub joint_position: Option<DoubleType>,
    pub joint_torque_or_force: Option<DoubleType>,
    pub joint_velocity: Option<DoubleType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JointStatusesType {
    pub name: Option<IdType>,
    pub joint_status: Vec<JointStatusType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum LengthUnitEnumType {
    Meter,
    Millimeter,
    Inch,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OnOffSensorStatusType {
    pub name: Option<IdType>,
    pub sensor_id: TokenType,
    pub read_count: IntType,
    pub last_read_time: LongType,
    pub sensor_parameter_setting: Vec<ParameterSettingType>,
    pub on: BooleanType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ParallelGripperStatusType {
    pub name: Option<IdType>,
    pub gripper_name: NmtokenType,
    pub gripper_option: Vec<ParameterSettingType>,
    pub holding_object: Option<BooleanType>,
    pub separation: DoubleType,
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
pub struct PoseStatusType {
    pub name: Option<IdType>,
    pub pose: PoseType,
    pub twist: Option<TwistType>,
    pub wrench: Option<WrenchType>,
    pub configuration: Option<StringType>,
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
pub struct ScalarSensorStatusType {
    pub name: Option<IdType>,
    pub sensor_id: TokenType,
    pub read_count: IntType,
    pub last_read_time: LongType,
    pub sensor_parameter_setting: Vec<ParameterSettingType>,
    pub scalar_value: DoubleType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SensorStatusType {
    pub name: Option<IdType>,
    pub sensor_id: TokenType,
    pub read_count: IntType,
    pub last_read_time: LongType,
    pub sensor_parameter_setting: Vec<ParameterSettingType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SensorStatusesType {
    pub name: Option<IdType>,
    pub on_off_sensor_status: Vec<OnOffSensorStatusType>,
    pub scalar_sensor_status: Vec<ScalarSensorStatusType>,
    pub count_sensor_status: Vec<CountSensorStatusType>,
    pub force_torque_sensor_status: Vec<ForceTorqueSensorStatusType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsStatusType {
    pub name: Option<IdType>,
    pub angle_unit_name: Option<AngleUnitEnumType>,
    pub end_effector_parameter_setting: Vec<ParameterSettingType>,
    pub end_effector_setting: Option<FractionType>,
    pub force_unit_name: Option<ForceUnitEnumType>,
    pub joint_limits: Vec<JointLimitType>,
    pub intermediate_pose_tolerance: Option<PoseToleranceType>,
    pub length_unit_name: Option<LengthUnitEnumType>,
    pub max_cartesian_limit: Option<PointType>,
    pub min_cartesian_limit: Option<PointType>,
    pub motion_coordinated: Option<BooleanType>,
    pub end_pose_tolerance: Option<PoseToleranceType>,
    pub robot_parameter_setting: Vec<ParameterSettingType>,
    pub rot_accel_absolute: Option<RotAccelAbsoluteType>,
    pub rot_accel_relative: Option<RotAccelRelativeType>,
    pub rot_speed_absolute: Option<RotSpeedAbsoluteType>,
    pub rot_speed_relative: Option<RotSpeedRelativeType>,
    pub torque_unit_name: Option<TorqueUnitEnumType>,
    pub trans_accel_absolute: Option<TransAccelAbsoluteType>,
    pub trans_accel_relative: Option<TransAccelRelativeType>,
    pub trans_speed_absolute: Option<TransSpeedAbsoluteType>,
    pub trans_speed_relative: Option<TransSpeedRelativeType>,
    pub joint_tolerances: Option<JointPositionsTolerancesType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreeFingerGripperStatusType {
    pub name: Option<IdType>,
    pub gripper_name: NmtokenType,
    pub gripper_option: Vec<ParameterSettingType>,
    pub holding_object: Option<BooleanType>,
    pub finger_1_position: Option<FractionType>,
    pub finger_2_position: Option<FractionType>,
    pub finger_3_position: Option<FractionType>,
    pub finger_1_force: Option<DoubleType>,
    pub finger_2_force: Option<DoubleType>,
    pub finger_3_force: Option<DoubleType>,
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
pub struct VacuumGripperStatusType {
    pub name: Option<IdType>,
    pub gripper_name: NmtokenType,
    pub gripper_option: Vec<ParameterSettingType>,
    pub holding_object: Option<BooleanType>,
    pub is_powered: BooleanType,
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
