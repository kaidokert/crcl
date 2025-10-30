use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AngleUnitEnumType {
    Degree,
    Radian,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DataThingType {
    pub name: Option<IdType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ForceUnitEnumType {
    Newton,
    Pound,
    Ounce,
}
pub type FractionType = f64;

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
pub enum LengthUnitEnumType {
    Meter,
    Millimeter,
    Inch,
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
