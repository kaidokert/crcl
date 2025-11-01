# CRCL Python

Python dataclasses for CRCL (Canonical Robot Command Language) generated from XSD schemas.

## Installation

### From GitHub (specific branch)
```bash
pip install git+https://github.com/kaidokert/crcl.git@hack2#subdirectory=crcl_python
```

### From GitHub (tagged version)
```bash
pip install git+https://github.com/kaidokert/crcl.git@v0.1.0#subdirectory=crcl_python
```

### For development
```bash
git clone https://github.com/kaidokert/crcl.git
cd crcl/crcl_python
pip install -e .
```

## Usage

```python
from crcl_python.crclstatus import JointStatusType, JointStatusesType
from crcl_python.crclcommands import MoveToType, ActuateJointsType
import json

# Create a joint status
joint = JointStatusType(
    name="Joint_1",
    joint_number=1,
    joint_position=45.5,
    joint_torque_or_force=12.3,
    joint_velocity=15.0
)

# Serialize to dict (for JSON)
from dataclasses import asdict
joint_dict = asdict(joint)
json_str = json.dumps(joint_dict)

# Parse from JSON
import json
data = json.loads(json_str)
parsed_joint = JointStatusType(**data)
```

## Advanced Usage with xsdata

```python
from xsdata.formats.dataclass.context import XmlContext
from xsdata.formats.dataclass.serializers import JsonSerializer, XmlSerializer
from xsdata.formats.dataclass.parsers import JsonParser, XmlParser

context = XmlContext()

# Serialize to JSON
json_serializer = JsonSerializer(context=context)
json_output = json_serializer.render(joint)

# Serialize to XML
xml_serializer = XmlSerializer(context=context)
xml_output = xml_serializer.render(joint)

# Parse from JSON
json_parser = JsonParser(context=context)
parsed = json_parser.from_string(json_output, JointStatusType)

# Parse from XML
xml_parser = XmlParser(context=context)
parsed = xml_parser.from_string(xml_output, JointStatusType)
```

## Available Types

### Status Types
- `CrclStatusType` - Main status message
- `JointStatusType` - Single joint status
- `JointStatusesType` - Multiple joint statuses
- `PoseStatusType` - End-effector pose status
- `GripperStatusType` - Gripper status
- And more...

### Command Types
- `MoveToType` - Point-to-point motion
- `MoveThroughToType` - Motion through waypoints
- `ActuateJointsType` - Direct joint control
- `SetEndEffectorType` - Tool configuration
- `GetStatusType` - Status query
- And more...

### Data Primitives
- `PointType` - 3D point
- `PoseType` - Position and orientation
- `VectorType` - 3D vector
- `TwistType` - Linear and angular velocity
- `WrenchType` - Force and torque
- And more...

## Generated From

These dataclasses are automatically generated from the official CRCL XSD schemas using `xsdata`.

## License

Same as the parent CRCL repository.