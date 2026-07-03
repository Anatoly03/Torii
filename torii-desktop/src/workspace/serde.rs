//! This module implements serialization and deserialization logic for the [Workspace] instance.

use crate::Workspace;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Visitor},
    ser::SerializeStruct,
};

impl Serialize for Workspace {
    /// Serialize this workspace into the given Serde serializer.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::Value;
    /// use app_lib::Workspace;
    ///
    /// let workspace = Workspace::new("/path/to/workspace");
    /// let json_string = serde_json::to_string(&workspace).unwrap();
    /// let json: Value = serde_json::from_str(&json_string).unwrap();
    ///
    /// assert_eq!(json["path"], "/path/to/workspace");
    /// assert_eq!(json["name"], "workspace");
    /// ```
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Workspace", 2)?;
        state.serialize_field("path", &self.path())?;
        state.serialize_field("name", &self.name())?;
        state.end()
    }
}

/// The [WorkspaceVisitor] struct is used to deserialize a [Workspace] from a string.
pub struct WorkspaceVisitor;

impl<'de> Visitor<'de> for WorkspaceVisitor {
    type Value = Workspace;

    /// Format a message stating what data the workspace visitor expects to receive.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing a workspace path")
    }

    /// The input contains a string.
    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Workspace::new(value))
    }
}

impl<'de> Deserialize<'de> for Workspace {
    /// Deserialize this workspace from the given Serde deserializer.
    ///
    /// # Example
    ///
    /// ```
    /// use app_lib::Workspace;
    ///
    /// let json = r#""/path/to/tiny-workspace""#;
    /// let workspace: Workspace = serde_json::from_str(json).unwrap();
    /// assert_eq!(workspace.name(), "tiny-workspace");
    /// ```
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(WorkspaceVisitor)
    }
}
