Canonical Robot Command Language
================================

The "Canonical Robot Command Language" (CRCL) provides generic command and status definitions that implement the functionality of typical industrial robots without being specific either to the language of a plan that is being executed or to the language used by a robot controller that executes CRCL commands. It can be used with offline planners that create output to be stored in CRCL files or online where CRCL is communicated in both directions via TCP. CRCL commands and status could also be exchanged over TCP between an operator interface and a robot controller or proxy for a robot controller.

This repository contains the reference documentation  in doc/REFERENCE.md. XML Schema Language files for validation and code generation in schemas subdirectory  and example instance files in the instances subdirectory. The tools subdirectory contains language-specific tools for parsing CRCL in C++, Java, and Python.
This repository contains the "standard" CRCL SCHEMA only. It is a standard version 1. There are tool repositories that help integrate CRCL with yoour application. There are example CRCL instance files in the instances subdirectory.

The primay CRCL Java tools can be found here:  [nist/crcl](https://github.com/usnistgov/crcl). This github repository contains experiemntal CRCL XSD as well as Java parsing tools. Under the tools subdirectory in the repository, are language-specific tools for parsing CRCL in  Java, C++, and Python. It is recommended to use Java, as it is the best integration technique, as it uses JaxB to parse the CRCL XSD and generate Java class definitions to parse and serialize the CRCL.

There is an another CRCL example repository found [<u>here</u>](https://github.com/usnistgov/crcl2ros) that contains a ROS workspace to handle CRCL streaming, CRCL to ROS conversion, and ROS to Gazebo simulation. This repository contains C++ code that implements CRCL XML streaming and parsing component, that maps command and status motion primitives from CRCL to ROS, then uses ROS moveit! to plan motion trajectories that are then simulated in Gazebo. Of note, the CRCL is parsed and serialized with XSD tools from Code Synthesis.

## Rust Implementation

This repository also includes a Rust implementation of CRCL:

### Workspace Structure

- **`crcl/`** - Library crate providing strongly-typed Rust structures for CRCL messages
- **`xsd_gen/`** - Code generator that converts CRCL XSD schemas into Rust types

### Quick Example

```rust
use crcl::{JointStatusType, JointStatusesType};

let joint = JointStatusType {
    name: None,
    joint_number: 0,
    joint_position: Some(1.57),
    joint_torque_or_force: Some(10.5),
    joint_velocity: Some(0.5),
};

// Serialize to XML
let xml = serde_xml_rs::to_string(&joint)?;
```

### Building

```bash
cargo build        # Build the workspace
cargo test         # Run all tests
```

### Documentation

- [crcl/README.md](crcl/README.md) - Library usage and examples
- [xsd_gen/README.md](xsd_gen/README.md) - Code generator documentation




