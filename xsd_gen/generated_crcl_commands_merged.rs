
#[derive (Debug)]
pub struct ActuateJointType { pub content : ActuateJointTypeContent , } 
#[derive (Debug)]
pub struct ActuateJointTypeContent { pub name : Option < IdType > , pub joint_number : IntType , pub joint_position : DoubleType , pub joint_details : JointDetailsType , } 
#[derive (Debug)]
pub struct ActuateJointsType { pub content : ActuateJointsTypeContent , } 
#[derive (Debug)]
pub struct ActuateJointsTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub actuate_joint : Vec < ActuateJointType > , pub joint_tolerances : Option < JointPositionsTolerancesType > , } 
#[derive (Debug)]
pub enum AngleUnitEnumType { Degree , Radian , } 
#[derive (Debug)]
pub struct CrclCommandType { pub content : CrclCommandTypeContent , } 
#[derive (Debug)]
pub struct CrclCommandTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , } 
#[derive (Debug)]
pub struct CloseToolChangerType { pub content : CloseToolChangerTypeContent , } 
#[derive (Debug)]
pub struct CloseToolChangerTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , } 
#[derive (Debug)]
pub struct ConfigureJointReportType { pub content : ConfigureJointReportTypeContent , } 
#[derive (Debug)]
pub struct ConfigureJointReportTypeContent { pub name : Option < IdType > , pub joint_number : IntType , pub report_position : BooleanType , pub report_torque_or_force : BooleanType , pub report_velocity : BooleanType , } 
#[derive (Debug)]
pub struct ConfigureJointReportsType { pub content : ConfigureJointReportsTypeContent , } 
#[derive (Debug)]
pub struct ConfigureJointReportsTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub reset_all : BooleanType , pub configure_joint_report : Vec < ConfigureJointReportType > , } 
#[derive (Debug)]
pub struct ConfigureStatusReportType { pub content : ConfigureStatusReportTypeContent , } 
#[derive (Debug)]
pub struct ConfigureStatusReportTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub report_joint_statuses : BooleanType , pub report_pose_status : BooleanType , pub report_gripper_status : BooleanType , pub report_settings_status : BooleanType , pub report_sensors_status : BooleanType , pub report_guards_status : BooleanType , } 
#[derive (Debug)]
pub struct DataThingType { pub content : DataThingTypeContent , } 
#[derive (Debug)]
pub struct DataThingTypeContent { pub name : Option < IdType > , } 
#[derive (Debug)]
pub struct DisableGripperType { pub content : DisableGripperTypeContent , } 
#[derive (Debug)]
pub struct DisableGripperTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub gripper_name : NmtokenType , } 
#[derive (Debug)]
pub struct DisableRobotParameterStatusType { pub content : DisableRobotParameterStatusTypeContent , } 
#[derive (Debug)]
pub struct DisableRobotParameterStatusTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub robot_parameter_name : TokenType , } 
#[derive (Debug)]
pub struct DisableSensorType { pub content : DisableSensorTypeContent , } 
#[derive (Debug)]
pub struct DisableSensorTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub sensor_id : TokenType , } 
#[derive (Debug)]
pub struct DwellType { pub content : DwellTypeContent , } 
#[derive (Debug)]
pub struct DwellTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub dwell_time : DoubleType , } 
#[derive (Debug)]
pub struct EnableGripperType { pub content : EnableGripperTypeContent , } 
#[derive (Debug)]
pub struct EnableGripperTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub gripper_name : NmtokenType , pub gripper_option : Vec < ParameterSettingType > , } 
#[derive (Debug)]
pub struct EnableRobotParameterStatusType { pub content : EnableRobotParameterStatusTypeContent , } 
#[derive (Debug)]
pub struct EnableRobotParameterStatusTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub robot_parameter_name : TokenType , } 
#[derive (Debug)]
pub struct EnableSensorType { pub content : EnableSensorTypeContent , } 
#[derive (Debug)]
pub struct EnableSensorTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub sensor_id : TokenType , pub sensor_option : Vec < ParameterSettingType > , } 
#[derive (Debug)]
pub struct EndCanonType { pub content : EndCanonTypeContent , } 
#[derive (Debug)]
pub struct EndCanonTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , } 
#[derive (Debug)]
pub enum ForceUnitEnumType { Newton , Pound , Ounce , }
pub type FractionType = f64;

#[derive (Debug)]
pub struct GetStatusType { pub content : GetStatusTypeContent , } 
#[derive (Debug)]
pub struct GetStatusTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , } 
#[derive (Debug)]
pub enum GuardLimitEnumType { OverMax , UnderMin , IncreaseOverLimit , DecreaseBeyondLimit , } 
#[derive (Debug)]
pub struct GuardType { pub content : GuardTypeContent , } 
#[derive (Debug)]
pub struct GuardTypeContent { pub name : Option < IdType > , pub sensor_id : TokenType , pub sub_field : Option < TokenType > , pub limit_type : GuardLimitEnumType , pub limit_value : DoubleType , pub recheck_time_micro_seconds : Option < LongType > , pub check_count : Option < LongType > , pub last_check_time : Option < LongType > , pub last_check_value : Option < DoubleType > , } 
#[derive (Debug)]
pub struct InitCanonType { pub content : InitCanonTypeContent , } 
#[derive (Debug)]
pub struct InitCanonTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , } 
#[derive (Debug)]
pub struct JointDetailsType { pub content : JointDetailsTypeContent , } 
#[derive (Debug)]
pub struct JointDetailsTypeContent { pub name : Option < IdType > , } 
#[derive (Debug)]
pub struct JointForceTorqueType { pub content : JointForceTorqueTypeContent , } 
#[derive (Debug)]
pub struct JointForceTorqueTypeContent { pub name : Option < IdType > , pub setting : Option < DoubleType > , pub change_rate : Option < DoubleType > , } 
#[derive (Debug)]
pub struct JointPositionToleranceSettingType { pub content : JointPositionToleranceSettingTypeContent , } 
#[derive (Debug)]
pub struct JointPositionToleranceSettingTypeContent { pub name : Option < IdType > , pub joint_number : IntType , pub joint_position_tolerance : DoubleType , } 
#[derive (Debug)]
pub struct JointPositionsTolerancesType { pub content : JointPositionsTolerancesTypeContent , } 
#[derive (Debug)]
pub struct JointPositionsTolerancesTypeContent { pub name : Option < IdType > , pub content_6 : JointPositionsTolerancesContent6Type , } 
#[derive (Debug)]
pub struct JointSpeedAccelType { pub content : JointSpeedAccelTypeContent , } 
#[derive (Debug)]
pub struct JointSpeedAccelTypeContent { pub name : Option < IdType > , pub joint_speed : Option < DoubleType > , pub joint_accel : Option < DoubleType > , } 
#[derive (Debug)]
pub enum LengthUnitEnumType { Meter , Millimeter , Inch , } 
#[derive (Debug)]
pub struct MessageType { pub content : MessageTypeContent , } 
#[derive (Debug)]
pub struct MessageTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub message : StringType , } 
#[derive (Debug)]
pub struct MiddleCommandType { pub content : MiddleCommandTypeContent , } 
#[derive (Debug)]
pub struct MiddleCommandTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , } 
#[derive (Debug)]
pub struct MoveScrewType { pub content : MoveScrewTypeContent , } 
#[derive (Debug)]
pub struct MoveScrewTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub start_position : Option < PoseType > , pub axis_point : Option < PointType > , pub axial_distance_free : Option < DoubleType > , pub axial_distance_screw : DoubleType , pub turn : DoubleType , } 
#[derive (Debug)]
pub struct MoveThroughToType { pub content : MoveThroughToTypeContent , } 
#[derive (Debug)]
pub struct MoveThroughToTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub move_straight : BooleanType , pub waypoint : Vec < PoseType > , pub num_positions : IntType , } 
#[derive (Debug)]
pub struct MoveToType { pub content : MoveToTypeContent , } 
#[derive (Debug)]
pub struct MoveToTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub move_straight : BooleanType , pub end_position : PoseType , } 
#[derive (Debug)]
pub struct OpenToolChangerType { pub content : OpenToolChangerTypeContent , } 
#[derive (Debug)]
pub struct OpenToolChangerTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , } 
#[derive (Debug)]
pub struct ParameterSettingType { pub content : ParameterSettingTypeContent , } 
#[derive (Debug)]
pub struct ParameterSettingTypeContent { pub name : Option < IdType > , pub parameter_name : TokenType , pub parameter_value : TokenType , } 
#[derive (Debug)]
pub struct PointType { pub content : PointTypeContent , } 
#[derive (Debug)]
pub struct PointTypeContent { pub name : Option < IdType > , pub x : DoubleType , pub y : DoubleType , pub z : DoubleType , } 
#[derive (Debug)]
pub struct PoseAndSetType { pub content : PoseAndSetTypeContent , } 
#[derive (Debug)]
pub struct PoseAndSetTypeContent { pub name : Option < IdType > , pub point : PointType , pub x_axis : VectorType , pub z_axis : VectorType , pub coordinated : BooleanType , pub trans_speed : Option < TransSpeedType > , pub rot_speed : Option < RotSpeedType > , pub trans_accel : Option < TransAccelType > , pub rot_accel : Option < RotAccelType > , pub tolerance : Option < PoseToleranceType > , } 
#[derive (Debug)]
pub struct PoseToleranceType { pub content : PoseToleranceTypeContent , } 
#[derive (Debug)]
pub struct PoseToleranceTypeContent { pub name : Option < IdType > , pub x_point_tolerance : Option < DoubleType > , pub y_point_tolerance : Option < DoubleType > , pub z_point_tolerance : Option < DoubleType > , pub x_axis_tolerance : Option < DoubleType > , pub z_axis_tolerance : Option < DoubleType > , } 
#[derive (Debug)]
pub struct PoseType { pub content : PoseTypeContent , } 
#[derive (Debug)]
pub struct PoseTypeContent { pub name : Option < IdType > , pub point : PointType , pub x_axis : VectorType , pub z_axis : VectorType , }
pub type PositiveDecimalType = f64;

#[derive (Debug)]
pub struct RotAccelAbsoluteType { pub content : RotAccelAbsoluteTypeContent , } 
#[derive (Debug)]
pub struct RotAccelAbsoluteTypeContent { pub name : Option < IdType > , pub setting : DoubleType , } 
#[derive (Debug)]
pub struct RotAccelRelativeType { pub content : RotAccelRelativeTypeContent , } 
#[derive (Debug)]
pub struct RotAccelRelativeTypeContent { pub name : Option < IdType > , pub fraction : FractionType , } 
#[derive (Debug)]
pub struct RotAccelType { pub content : RotAccelTypeContent , } 
#[derive (Debug)]
pub struct RotAccelTypeContent { pub name : Option < IdType > , } 
#[derive (Debug)]
pub struct RotSpeedAbsoluteType { pub content : RotSpeedAbsoluteTypeContent , } 
#[derive (Debug)]
pub struct RotSpeedAbsoluteTypeContent { pub name : Option < IdType > , pub setting : DoubleType , } 
#[derive (Debug)]
pub struct RotSpeedRelativeType { pub content : RotSpeedRelativeTypeContent , } 
#[derive (Debug)]
pub struct RotSpeedRelativeTypeContent { pub name : Option < IdType > , pub fraction : FractionType , } 
#[derive (Debug)]
pub struct RotSpeedType { pub content : RotSpeedTypeContent , } 
#[derive (Debug)]
pub struct RotSpeedTypeContent { pub name : Option < IdType > , } 
#[derive (Debug)]
pub struct RunProgramType { pub content : RunProgramTypeContent , } 
#[derive (Debug)]
pub struct RunProgramTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub program_text : StringType , } 
#[derive (Debug)]
pub struct SetAngleUnitsType { pub content : SetAngleUnitsTypeContent , } 
#[derive (Debug)]
pub struct SetAngleUnitsTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub unit_name : AngleUnitEnumType , } 
#[derive (Debug)]
pub struct SetDefaultJointPositonsTolerancesType { pub content : SetDefaultJointPositonsTolerancesTypeContent , } 
#[derive (Debug)]
pub struct SetDefaultJointPositonsTolerancesTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub joint_tolerances : JointPositionsTolerancesType , } 
#[derive (Debug)]
pub struct SetEndEffectorParametersType { pub content : SetEndEffectorParametersTypeContent , } 
#[derive (Debug)]
pub struct SetEndEffectorParametersTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub parameter_setting : Vec < ParameterSettingType > , } 
#[derive (Debug)]
pub struct SetEndEffectorType { pub content : SetEndEffectorTypeContent , } 
#[derive (Debug)]
pub struct SetEndEffectorTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub setting : FractionType , } 
#[derive (Debug)]
pub struct SetEndPoseToleranceType { pub content : SetEndPoseToleranceTypeContent , } 
#[derive (Debug)]
pub struct SetEndPoseToleranceTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub tolerance : PoseToleranceType , } 
#[derive (Debug)]
pub struct SetForceUnitsType { pub content : SetForceUnitsTypeContent , } 
#[derive (Debug)]
pub struct SetForceUnitsTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub unit_name : ForceUnitEnumType , } 
#[derive (Debug)]
pub struct SetIntermediatePoseToleranceType { pub content : SetIntermediatePoseToleranceTypeContent , } 
#[derive (Debug)]
pub struct SetIntermediatePoseToleranceTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub tolerance : PoseToleranceType , } 
#[derive (Debug)]
pub struct SetLengthUnitsType { pub content : SetLengthUnitsTypeContent , } 
#[derive (Debug)]
pub struct SetLengthUnitsTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub unit_name : LengthUnitEnumType , } 
#[derive (Debug)]
pub struct SetMotionCoordinationType { pub content : SetMotionCoordinationTypeContent , } 
#[derive (Debug)]
pub struct SetMotionCoordinationTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub coordinated : BooleanType , } 
#[derive (Debug)]
pub struct SetRobotParametersType { pub content : SetRobotParametersTypeContent , } 
#[derive (Debug)]
pub struct SetRobotParametersTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub parameter_setting : Vec < ParameterSettingType > , } 
#[derive (Debug)]
pub struct SetRotAccelType { pub content : SetRotAccelTypeContent , } 
#[derive (Debug)]
pub struct SetRotAccelTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub rot_accel : RotAccelType , } 
#[derive (Debug)]
pub struct SetRotSpeedType { pub content : SetRotSpeedTypeContent , } 
#[derive (Debug)]
pub struct SetRotSpeedTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub rot_speed : RotSpeedType , } 
#[derive (Debug)]
pub struct SetTorqueUnitsType { pub content : SetTorqueUnitsTypeContent , } 
#[derive (Debug)]
pub struct SetTorqueUnitsTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub unit_name : TorqueUnitEnumType , } 
#[derive (Debug)]
pub struct SetTransAccelType { pub content : SetTransAccelTypeContent , } 
#[derive (Debug)]
pub struct SetTransAccelTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub trans_accel : TransAccelType , } 
#[derive (Debug)]
pub struct SetTransSpeedType { pub content : SetTransSpeedTypeContent , } 
#[derive (Debug)]
pub struct SetTransSpeedTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub trans_speed : TransSpeedType , } 
#[derive (Debug)]
pub enum StopConditionEnumType { Immediate , Fast , Normal , } 
#[derive (Debug)]
pub struct StopMotionType { pub content : StopMotionTypeContent , } 
#[derive (Debug)]
pub struct StopMotionTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub guard : Vec < GuardType > , pub stop_condition : StopConditionEnumType , } 
#[derive (Debug)]
pub enum TorqueUnitEnumType { NewtonMeter , FootPound , } 
#[derive (Debug)]
pub struct TransAccelAbsoluteType { pub content : TransAccelAbsoluteTypeContent , } 
#[derive (Debug)]
pub struct TransAccelAbsoluteTypeContent { pub name : Option < IdType > , pub setting : DoubleType , } 
#[derive (Debug)]
pub struct TransAccelRelativeType { pub content : TransAccelRelativeTypeContent , } 
#[derive (Debug)]
pub struct TransAccelRelativeTypeContent { pub name : Option < IdType > , pub fraction : FractionType , } 
#[derive (Debug)]
pub struct TransAccelType { pub content : TransAccelTypeContent , } 
#[derive (Debug)]
pub struct TransAccelTypeContent { pub name : Option < IdType > , } 
#[derive (Debug)]
pub struct TransSpeedAbsoluteType { pub content : TransSpeedAbsoluteTypeContent , } 
#[derive (Debug)]
pub struct TransSpeedAbsoluteTypeContent { pub name : Option < IdType > , pub setting : DoubleType , } 
#[derive (Debug)]
pub struct TransSpeedRelativeType { pub content : TransSpeedRelativeTypeContent , } 
#[derive (Debug)]
pub struct TransSpeedRelativeTypeContent { pub name : Option < IdType > , pub fraction : FractionType , } 
#[derive (Debug)]
pub struct TransSpeedType { pub content : TransSpeedTypeContent , } 
#[derive (Debug)]
pub struct TransSpeedTypeContent { pub name : Option < IdType > , } 
#[derive (Debug)]
pub struct TwistType { pub content : TwistTypeContent , } 
#[derive (Debug)]
pub struct TwistTypeContent { pub name : Option < IdType > , pub linear_velocity : VectorType , pub angular_velocity : VectorType , } 
#[derive (Debug)]
pub struct VectorType { pub content : VectorTypeContent , } 
#[derive (Debug)]
pub struct VectorTypeContent { pub name : Option < IdType > , pub i : DoubleType , pub j : DoubleType , pub k : DoubleType , } 
#[derive (Debug)]
pub struct WrenchType { pub content : WrenchTypeContent , } 
#[derive (Debug)]
pub struct WrenchTypeContent { pub name : Option < IdType > , pub force : VectorType , pub moment : VectorType , } 
#[derive (Debug , Default)]
pub struct EntitiesType (pub Vec < String >);

#[derive (Debug , Default)]
pub struct EntityType (pub Vec < String >) ;
pub type IdType = String ;
pub type IdrefType = String;

#[derive (Debug , Default)]
pub struct IdrefsType (pub Vec < String >) ;
pub type NcNameType = String ;
pub type NmtokenType = String;

#[derive (Debug , Default)]
pub struct NmtokensType (pub Vec < String >) ;
pub type NotationType = String ;
pub type NameType = String ;
pub type QNameType = String ;
pub type AnySimpleType = String;

#[derive (Debug)]
pub struct AnyType ;
pub type AnyUriType = String ;
pub type Base64BinaryType = String ;
pub type BooleanType = bool ;
pub type ByteType = i8 ;
pub type DateType = String ;
pub type DateTimeType = String ;
pub type DecimalType = f64 ;
pub type DoubleType = f64 ;
pub type DurationType = String ;
pub type FloatType = f32 ;
pub type GDayType = String ;
pub type GMonthType = String ;
pub type GMonthDayType = String ;
pub type GYearType = String ;
pub type GYearMonthType = String ;
pub type HexBinaryType = String ;
pub type IntType = i32 ;
pub type IntegerType = i32 ;
pub type LanguageType = String ;
pub type LongType = i64 ;
pub type NegativeIntegerType = isize ;
pub type NonNegativeIntegerType = usize ;
pub type NonPositiveIntegerType = isize ;
pub type NormalizedStringType = String ;
pub type PositiveIntegerType = usize ;
pub type ShortType = i16 ;
pub type StringType = String ;
pub type TimeType = String ;
pub type TokenType = String ;
pub type UnsignedByteType = u8 ;
pub type UnsignedIntType = u32 ;
pub type UnsignedLongType = u64 ;
pub type UnsignedShortType = u16;

#[derive (Debug)]
pub struct JointPositionsTolerancesContent6Type { pub content : JointPositionsTolerancesContent6TypeContent , } 
#[derive (Debug)]
pub struct JointPositionsTolerancesContent6TypeContent { pub setting : Vec < JointPositionToleranceSettingType > , }