# CRCL Rust-Python Interoperability Demo

## Overview
This demo shows seamless message exchange between Rust and Python using CRCL types generated from the same XSD schemas.

## Architecture

```
┌─────────────────┐         ┌──────────────────┐
│   XSD Schemas   │────────▶│   Code Generation │
│                 │         │                  │
│ • DataPrimitives│         │ • xsd_gen (Rust) │
│ • CRCLCommands  │         │ • xsdata (Python)│
│ • CRCLStatus    │         └──────────────────┘
└─────────────────┘                  │
                                    │
                    ┌───────────────┴───────────────┐
                    ▼                               ▼
        ┌──────────────────┐           ┌──────────────────┐
        │   Rust Types     │           │  Python Classes  │
        │                  │           │                  │
        │ • Strongly typed │           │ • Type hints     │
        │ • serde derive   │           │ • Dataclasses    │
        │ • Zero-copy      │           │ • xsdata support │
        └──────────────────┘           └──────────────────┘
                    │                               │
                    ▼                               ▼
        ┌──────────────────┐           ┌──────────────────┐
        │  Rust Generator  │           │  Python Reader   │
        │                  │           │                  │
        │ Generates CRCL   │  JSON/XML │ Reads & logs     │
        │ messages ────────┼──────────▶│ CRCL messages    │
        └──────────────────┘           └──────────────────┘
```

## Demo Flow

### 1️⃣ **Rust Generates Messages** (`cargo run --example generate_messages`)

```rust
// Creates strongly-typed CRCL messages
let joint = JointStatusType {
    name: Some("Robot_1_Joint_1".to_string()),
    joint_number: 1,
    joint_position: Some(45.5),
    joint_torque_or_force: Some(12.3),
    joint_velocity: Some(15.0),
};

// Serializes to JSON/XML
let json = serde_json::to_string_pretty(&joint)?;
fs::write("test_messages/joint_status_1.json", json)?;
```

**Generated Files:**
- `joint_status_1.json` - Single joint status
- `scara_joints.json` - 4-joint SCARA robot status
- `trajectory_sequence.json` - 5-point trajectory
- `*.xml` - XML versions

### 2️⃣ **Python Reads Messages** (`python read_rust_messages.py`)

```python
# Reads the Rust-generated JSON
with open('test_messages/joint_status_1.json') as f:
    data = json.load(f)

# Constructs typed Python object
joint = JointStatusType(
    name=data.get('name'),
    joint_number=data.get('joint_number'),
    joint_position=data.get('joint_position'),
    # ...
)

# Logs with formatting
logger.info(f"✓ Joint: {joint.name}")
logger.debug(f"  Position: {joint.joint_position}°")
```

## Sample Output

### Rust Generation
```
✓ Generated: test_messages/joint_status_1.json (137 bytes)
✓ Generated: test_messages/scara_joints.json (734 bytes)
✓ Generated: test_messages/trajectory_sequence.json (2347 bytes)
```

### Python Reading
```
INFO  [CRCL_Reader] Reading single joint from: joint_status_1.json
INFO  [CRCL_Reader]   ✓ Joint: Robot_1_Joint_1
DEBUG [CRCL_Reader]     Position: 45.5°
DEBUG [CRCL_Reader]     Torque: 12.3 Nm
DEBUG [CRCL_Reader]     Velocity: 15.0°/s

INFO  [CRCL_Reader] Reading multiple joints from: scara_joints.json
INFO  [CRCL_Reader]   ✓ Message: SCARA_Robot_Status
INFO  [CRCL_Reader]   ✓ Found 4 joints:
DEBUG [CRCL_Reader]     • J1_Base_Rotation: pos=45.50, vel=15.00
DEBUG [CRCL_Reader]     • J2_Shoulder_Joint: pos=30.25, vel=10.50
DEBUG [CRCL_Reader]     • J3_Z_Axis_Prismatic: pos=150.00, vel=25.00
DEBUG [CRCL_Reader]     • J4_Wrist_Rotation: pos=90.00, vel=45.00
```

## Message Format Example

```json
{
  "name": "Robot_1_Joint_1",
  "joint_number": 1,
  "joint_position": 45.5,
  "joint_torque_or_force": 12.3,
  "joint_velocity": 15.0
}
```

## Key Benefits Demonstrated

### ✅ **Language Interoperability**
- Rust and Python exchange messages seamlessly
- Same schema, different languages
- JSON as universal format

### ✅ **Type Safety**
- **Rust**: Compile-time checking
- **Python**: Runtime validation with type hints
- Both generated from authoritative XSD

### ✅ **Real-World Use Cases**
1. **Robot Controller** (Rust) → **Monitoring System** (Python)
2. **High-Performance Core** (Rust) → **Data Analysis** (Python)
3. **Embedded System** (Rust) → **Cloud Service** (Python)

### ✅ **Performance & Flexibility**
- Rust for performance-critical paths
- Python for rapid development and analysis
- Same message format ensures compatibility

## Running the Demo

```bash
# Complete demo
./run_interop_demo.sh

# Or run individually:
# 1. Generate with Rust
cargo run --example generate_messages

# 2. Read with Python
python read_rust_messages.py
```

## Conclusion

This demo proves that CRCL messages can be exchanged between different languages while maintaining type safety and structure. The XSD-to-code generation ensures both implementations stay synchronized with the standard.