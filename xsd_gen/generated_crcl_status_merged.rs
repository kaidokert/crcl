
#[derive (Debug)]
pub enum AngleUnitEnumType { Degree , Radian , }
pub type CrclStatus = CrclStatusType;

#[derive (Debug)]
pub struct CrclStatusType { pub content : CrclStatusTypeContent , } 
#[derive (Debug)]
pub struct CrclStatusTypeContent { pub name : Option < IdType > , pub command_status : CommandStatusType , pub joint_statuses : Option < JointStatusesType > , pub pose_status : Option < PoseStatusType > , pub gripper_status : Option < GripperStatusType > , pub settings_status : Option < SettingsStatusType > , pub sensor_statuses : Option < SensorStatusesType > , pub guards_statuses : Option < GuardsStatusesType > , } 
#[derive (Debug)]
pub enum CommandStateEnumType { CrclDone , CrclError , CrclWorking , CrclReady , } 
#[derive (Debug)]
pub struct CommandStatusType { pub content : CommandStatusTypeContent , } 
#[derive (Debug)]
pub struct CommandStatusTypeContent { pub name : Option < IdType > , pub command_id : LongType , pub status_id : LongType , pub command_state : CommandStateEnumType , pub state_description : Option < StringType > , pub program_file : Option < StringType > , pub program_index : Option < IntType > , pub program_length : Option < IntType > , pub override_percent : Option < IntType > , } 
#[derive (Debug)]
pub struct CountSensorStatusType { pub content : CountSensorStatusTypeContent , } 
#[derive (Debug)]
pub struct CountSensorStatusTypeContent { pub name : Option < IdType > , pub sensor_id : TokenType , pub read_count : IntType , pub last_read_time : LongType , pub sensor_parameter_setting : Vec < ParameterSettingType > , pub count_value : IntType , } 
#[derive (Debug)]
pub struct DataThingType { pub content : DataThingTypeContent , } 
#[derive (Debug)]
pub struct DataThingTypeContent { pub name : Option < IdType > , } 
#[derive (Debug)]
pub struct ForceTorqueSensorStatusType { pub content : ForceTorqueSensorStatusTypeContent , } 
#[derive (Debug)]
pub struct ForceTorqueSensorStatusTypeContent { pub name : Option < IdType > , pub sensor_id : TokenType , pub read_count : IntType , pub last_read_time : LongType , pub sensor_parameter_setting : Vec < ParameterSettingType > , pub fx : DoubleType , pub fy : DoubleType , pub fz : DoubleType , pub tx : DoubleType , pub ty : DoubleType , pub tz : DoubleType , } 
#[derive (Debug)]
pub enum ForceUnitEnumType { Newton , Pound , Ounce , }
pub type FractionType = f64;

#[derive (Debug)]
pub struct GripperStatusType { pub content : GripperStatusTypeContent , } 
#[derive (Debug)]
pub struct GripperStatusTypeContent { pub name : Option < IdType > , pub gripper_name : NmtokenType , pub gripper_option : Vec < ParameterSettingType > , pub holding_object : Option < BooleanType > , } 
#[derive (Debug)]
pub enum GuardLimitEnumType { OverMax , UnderMin , IncreaseOverLimit , DecreaseBeyondLimit , } 
#[derive (Debug)]
pub struct GuardType { pub content : GuardTypeContent , } 
#[derive (Debug)]
pub struct GuardTypeContent { pub name : Option < IdType > , pub sensor_id : TokenType , pub sub_field : Option < TokenType > , pub limit_type : GuardLimitEnumType , pub limit_value : DoubleType , pub recheck_time_micro_seconds : Option < LongType > , pub check_count : Option < LongType > , pub last_check_time : Option < LongType > , pub last_check_value : Option < DoubleType > , } 
#[derive (Debug)]
pub struct GuardsStatusesType { pub content : GuardsStatusesTypeContent , } 
#[derive (Debug)]
pub struct GuardsStatusesTypeContent { pub name : Option < IdType > , pub guard : Vec < GuardType > , pub trigger_count : IntType , pub trigger_stop_time_micros : LongType , pub trigger_value : Option < DoubleType > , pub trigger_pose : Option < PoseType > , } 
#[derive (Debug)]
pub struct JointLimitType { pub content : JointLimitTypeContent , } 
#[derive (Debug)]
pub struct JointLimitTypeContent { pub name : Option < IdType > , pub joint_number : IntType , pub joint_min_position : Option < DoubleType > , pub joint_max_position : Option < DoubleType > , pub joint_max_torque_or_force : Option < DoubleType > , pub joint_max_velocity : Option < DoubleType > , } 
#[derive (Debug)]
pub struct JointPositionToleranceSettingType { pub content : JointPositionToleranceSettingTypeContent , } 
#[derive (Debug)]
pub struct JointPositionToleranceSettingTypeContent { pub name : Option < IdType > , pub joint_number : IntType , pub joint_position_tolerance : DoubleType , } 
#[derive (Debug)]
pub struct JointPositionsTolerancesType { pub content : JointPositionsTolerancesTypeContent , } 
#[derive (Debug)]
pub struct JointPositionsTolerancesTypeContent { pub name : Option < IdType > , pub content_6 : JointPositionsTolerancesContent6Type , } 
#[derive (Debug)]
pub struct JointStatusType { pub content : JointStatusTypeContent , } 
#[derive (Debug)]
pub struct JointStatusTypeContent { pub name : Option < IdType > , pub joint_number : IntType , pub joint_position : Option < DoubleType > , pub joint_torque_or_force : Option < DoubleType > , pub joint_velocity : Option < DoubleType > , } 
#[derive (Debug)]
pub struct JointStatusesType { pub content : JointStatusesTypeContent , } 
#[derive (Debug)]
pub struct JointStatusesTypeContent { pub name : Option < IdType > , pub joint_status : Vec < JointStatusType > , } 
#[derive (Debug)]
pub enum LengthUnitEnumType { Meter , Millimeter , Inch , } 
#[derive (Debug)]
pub struct OnOffSensorStatusType { pub content : OnOffSensorStatusTypeContent , } 
#[derive (Debug)]
pub struct OnOffSensorStatusTypeContent { pub name : Option < IdType > , pub sensor_id : TokenType , pub read_count : IntType , pub last_read_time : LongType , pub sensor_parameter_setting : Vec < ParameterSettingType > , pub on : BooleanType , } 
#[derive (Debug)]
pub struct ParallelGripperStatusType { pub content : ParallelGripperStatusTypeContent , } 
#[derive (Debug)]
pub struct ParallelGripperStatusTypeContent { pub name : Option < IdType > , pub gripper_name : NmtokenType , pub gripper_option : Vec < ParameterSettingType > , pub holding_object : Option < BooleanType > , pub separation : DoubleType , } 
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
pub struct PoseStatusType { pub content : PoseStatusTypeContent , } 
#[derive (Debug)]
pub struct PoseStatusTypeContent { pub name : Option < IdType > , pub pose : PoseType , pub twist : Option < TwistType > , pub wrench : Option < WrenchType > , pub configuration : Option < StringType > , } 
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
pub struct ScalarSensorStatusType { pub content : ScalarSensorStatusTypeContent , } 
#[derive (Debug)]
pub struct ScalarSensorStatusTypeContent { pub name : Option < IdType > , pub sensor_id : TokenType , pub read_count : IntType , pub last_read_time : LongType , pub sensor_parameter_setting : Vec < ParameterSettingType > , pub scalar_value : DoubleType , } 
#[derive (Debug)]
pub struct SensorStatusType { pub content : SensorStatusTypeContent , } 
#[derive (Debug)]
pub struct SensorStatusTypeContent { pub name : Option < IdType > , pub sensor_id : TokenType , pub read_count : IntType , pub last_read_time : LongType , pub sensor_parameter_setting : Vec < ParameterSettingType > , } 
#[derive (Debug)]
pub struct SensorStatusesType { pub content : SensorStatusesTypeContent , } 
#[derive (Debug)]
pub struct SensorStatusesTypeContent { pub name : Option < IdType > , pub on_off_sensor_status : Vec < OnOffSensorStatusType > , pub scalar_sensor_status : Vec < ScalarSensorStatusType > , pub count_sensor_status : Vec < CountSensorStatusType > , pub force_torque_sensor_status : Vec < ForceTorqueSensorStatusType > , } 
#[derive (Debug)]
pub struct SettingsStatusType { pub content : SettingsStatusTypeContent , } 
#[derive (Debug)]
pub struct SettingsStatusTypeContent { pub name : Option < IdType > , pub angle_unit_name : Option < AngleUnitEnumType > , pub end_effector_parameter_setting : Vec < ParameterSettingType > , pub end_effector_setting : Option < FractionType > , pub force_unit_name : Option < ForceUnitEnumType > , pub joint_limits : Vec < JointLimitType > , pub intermediate_pose_tolerance : Option < PoseToleranceType > , pub length_unit_name : Option < LengthUnitEnumType > , pub max_cartesian_limit : Option < PointType > , pub min_cartesian_limit : Option < PointType > , pub motion_coordinated : Option < BooleanType > , pub end_pose_tolerance : Option < PoseToleranceType > , pub robot_parameter_setting : Vec < ParameterSettingType > , pub rot_accel_absolute : Option < RotAccelAbsoluteType > , pub rot_accel_relative : Option < RotAccelRelativeType > , pub rot_speed_absolute : Option < RotSpeedAbsoluteType > , pub rot_speed_relative : Option < RotSpeedRelativeType > , pub torque_unit_name : Option < TorqueUnitEnumType > , pub trans_accel_absolute : Option < TransAccelAbsoluteType > , pub trans_accel_relative : Option < TransAccelRelativeType > , pub trans_speed_absolute : Option < TransSpeedAbsoluteType > , pub trans_speed_relative : Option < TransSpeedRelativeType > , pub joint_tolerances : Option < JointPositionsTolerancesType > , } 
#[derive (Debug)]
pub struct ThreeFingerGripperStatusType { pub content : ThreeFingerGripperStatusTypeContent , } 
#[derive (Debug)]
pub struct ThreeFingerGripperStatusTypeContent { pub name : Option < IdType > , pub gripper_name : NmtokenType , pub gripper_option : Vec < ParameterSettingType > , pub holding_object : Option < BooleanType > , pub finger_1_position : Option < FractionType > , pub finger_2_position : Option < FractionType > , pub finger_3_position : Option < FractionType > , pub finger_1_force : Option < DoubleType > , pub finger_2_force : Option < DoubleType > , pub finger_3_force : Option < DoubleType > , } 
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
pub struct VacuumGripperStatusType { pub content : VacuumGripperStatusTypeContent , } 
#[derive (Debug)]
pub struct VacuumGripperStatusTypeContent { pub name : Option < IdType > , pub gripper_name : NmtokenType , pub gripper_option : Vec < ParameterSettingType > , pub holding_object : Option < BooleanType > , pub is_powered : BooleanType , } 
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