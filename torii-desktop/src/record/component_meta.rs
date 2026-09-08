use serde::{Deserialize, Serialize};

/// Component Metadata
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentMeta {
    pub name: String,
    pub permissions: ComponentPermissions,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentPermissions {
    pub write: bool,
    pub write_from_file: bool,
    pub read: bool,
    pub remove: bool,
}

