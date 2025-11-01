#!/bin/bash
# Comprehensive JQ queries for CRCL message types

echo "======================================"
echo "CRCL MESSAGE TYPE QUERIES"
echo "======================================"
echo

echo "=== COMMAND TYPES (from CRCLCommands.json) ==="
echo

echo "1. Get all command type names:"
echo "   jq '.definitions | keys' jsonschemas/CRCLCommands.json"
jq '.definitions | keys | length' jsonschemas/CRCLCommands.json | xargs -I {} echo "   Total: {} types"
echo

echo "2. Get only actual command types (filtering out primitives/enums):"
echo "   Run this command:"
cat << 'EOF'
jq '.definitions | keys | map(select(
  endswith("Type") and (
    contains("Actuate") or contains("Move") or contains("Set") or
    contains("Get") or contains("Open") or contains("Close") or
    contains("Stop") or contains("Init") or contains("End") or
    contains("Dwell") or contains("Message") or contains("Enable") or
    contains("Disable") or contains("Configure") or contains("Run") or
    contains("MiddleCommand") or contains("CRCLCommand")
  )
))' jsonschemas/CRCLCommands.json
EOF
echo

echo "3. Main CRCL Commands (37 types that extend CRCLCommandType/MiddleCommandType):"
echo "   These are the actual executable commands:"
cat << 'EOF'
  - ActuateJointsType         (Joint control)
  - CloseToolChangerType      (Tool changer)
  - ConfigureJointReportsType (Configuration)
  - ConfigureStatusReportType (Configuration)
  - DisableGripperType        (Gripper control)
  - DisableRobotParameterStatusType
  - DisableSensorType         (Sensor control)
  - DwellType                 (Time delay)
  - EnableGripperType         (Gripper control)
  - EnableRobotParameterStatusType
  - EnableSensorType          (Sensor control)
  - EndCanonType              (Program control)
  - GetStatusType             (Status query)
  - InitCanonType             (Program control)
  - MessageType               (Communication)
  - MoveScrewType             (Motion - helical)
  - MoveThroughToType         (Motion - waypoint)
  - MoveToType                (Motion - point to point)
  - OpenToolChangerType       (Tool changer)
  - RunProgramType            (Program execution)
  - SetAngleUnitsType         (Unit configuration)
  - SetDefaultJointPositonsTolerancesType
  - SetEndEffectorParametersType
  - SetEndEffectorType
  - SetEndPoseToleranceType
  - SetForceUnitsType         (Unit configuration)
  - SetIntermediatePoseToleranceType
  - SetLengthUnitsType        (Unit configuration)
  - SetMotionCoordinationType
  - SetRobotParametersType
  - SetRotAccelType           (Motion parameters)
  - SetRotSpeedType           (Motion parameters)
  - SetTorqueUnitsType        (Unit configuration)
  - SetTransAccelType         (Motion parameters)
  - SetTransSpeedType         (Motion parameters)
  - StopMotionType            (Motion control)
EOF
echo

echo "=== STATUS TYPES (from CRCLStatus.json) ==="
echo

echo "4. Get all status type names:"
echo "   jq '.definitions | keys' jsonschemas/CRCLStatus.json"
jq '.definitions | length' jsonschemas/CRCLStatus.json | xargs -I {} echo "   Total: {} types"
echo

echo "5. Get only status message types:"
echo "   jq '.definitions | keys | map(select(contains(\"Status\")))' jsonschemas/CRCLStatus.json"
echo

echo "6. Main Status Types (17 types):"
cat << 'EOF'
  - CRCLStatusType             (Main status message)
  - CommandStatusType          (Command execution status)
  - JointStatusType            (Single joint status)
  - JointStatusesType          (All joints status)
  - PoseStatusType             (End-effector pose)
  - GripperStatusType          (Gripper state)
  - ParallelGripperStatusType  (2-finger gripper)
  - ThreeFingerGripperStatusType
  - VacuumGripperStatusType
  - SensorStatusType           (Sensor readings)
  - CountSensorStatusType
  - OnOffSensorStatusType
  - ScalarSensorStatusType
  - ForceTorqueSensorStatusType
  - SensorStatusesType         (All sensors)
  - GuardsStatusesType         (Safety guards)
  - SettingsStatusType         (Current settings)
EOF
echo

echo "=== USEFUL COMBINED QUERIES ==="
echo

echo "7. Get command categories:"
cat << 'EOF'
# Motion commands
jq '.definitions | keys | map(select(contains("Move") or contains("Actuate")))' jsonschemas/CRCLCommands.json

# Configuration commands
jq '.definitions | keys | map(select(contains("Set") or contains("Configure")))' jsonschemas/CRCLCommands.json

# Control commands
jq '.definitions | keys | map(select(contains("Enable") or contains("Disable") or contains("Stop")))' jsonschemas/CRCLCommands.json
EOF
echo

echo "8. Get properties of specific command (e.g., MoveToType):"
echo "   jq '.definitions.MoveToType.properties | keys' jsonschemas/CRCLCommands.json"
echo

echo "9. Find required fields for commands:"
echo "   jq '.definitions.MoveToType.required' jsonschemas/CRCLCommands.json"
echo

echo "10. Export all command names to a text file:"
echo "    jq -r '.definitions | keys[]' jsonschemas/CRCLCommands.json > command_types.txt"
echo "    jq -r '.definitions | keys[]' jsonschemas/CRCLStatus.json > status_types.txt"
echo

echo "======================================"