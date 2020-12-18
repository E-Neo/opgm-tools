//! Types

/// Type of vertex id
pub type VId = i32;

/// Type of vertex label
pub type VLabel = i16;

/// Type of edge label
pub type ELabel = i16;

#[repr(C, packed)]
pub struct VIdVLabel(pub VId, pub VLabel);

#[repr(C, packed)]
pub struct VIdVIdELabel(pub VId, pub VId, pub ELabel);
