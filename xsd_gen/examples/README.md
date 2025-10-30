# CRCL Examples

## basic_usage.rs

Comprehensive example demonstrating all core features:

- Creating `JointStatusType` messages
- Serializing structs to XML
- Deserializing XML to structs
- Working with collections (`JointStatusesType`)
- Creating command messages (`ActuateJointsType`)

To understand the generated API, start with this example.

## Running Examples

The examples are designed to be read as code samples. They include the generated types inline, so they're self-contained but not meant to be compiled directly.

To use the patterns shown:

1. Copy the relevant code snippets to your project
2. Include the generated files from `xsd_gen/` directory
3. Add dependencies to your `Cargo.toml`:
   ```toml
   serde = { version = "1.0", features = ["derive"] }
   serde-xml-rs = "0.8"
   ```

## Key Patterns

### Creating a simple joint status
```rust
let joint = JointStatusType {
    name: None,
    joint_number: 0,
    joint_position: Some(1.57),
    joint_torque_or_force: Some(10.5),
    joint_velocity: Some(0.5),
};
```

### Serializing to XML
```rust
let xml = serde_xml_rs::to_string(&joint)?;
```

### Deserializing from XML
```rust
let joint: JointStatusType = serde_xml_rs::from_str(xml_string)?;
```

### Direct field access (no .content wrapper!)
```rust
println!("Joint {}: pos={}", joint.joint_number, joint.joint_position);
```
