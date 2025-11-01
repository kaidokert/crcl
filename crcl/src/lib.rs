//! CRCL (Canonical Robot Command Language) Rust Library
//!
//! This library provides strongly-typed Rust structures for working with CRCL XML messages.
//! CRCL is a standard XML-based protocol for robot control and status reporting.
//!
//! # Modules
//!
//! - `primitives` - Base data types (vectors, points, poses, units)
//! - `commands` - Robot command message types (e.g., `ActuateJointsType`)
//! - `status` - Robot status message types (e.g., `JointStatusesType`)
//!
//! # Example
//!
//! ```rust
//! use crcl::status::JointStatusType;
//!
//! let joint = JointStatusType {
//!     name: None,
//!     joint_number: 0,
//!     joint_position: Some(1.57),
//!     joint_torque_or_force: Some(10.5),
//!     joint_velocity: Some(0.5),
//! };
//!
//! // Serialize to XML
//! let xml = serde_xml_rs::to_string(&joint).unwrap();
//! ```

pub mod primitives {
    //! Base CRCL data types
    include!("primitives.rs");
}

pub mod commands {
    //! CRCL command message types
    include!("commands.rs");
}

pub mod status {
    //! CRCL status message types
    include!("status.rs");
}

// Re-export commonly used types at the crate root
pub use commands::{ActuateJointType, ActuateJointsType, JointDetails, JointSpeedAccelType, JointForceTorqueType};
pub use status::{JointStatusType, JointStatusesType};
