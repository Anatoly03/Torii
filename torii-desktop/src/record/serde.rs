//! This module implements serialization and deserialization logic for the [Record] instance.

use crate::{Record, Workspace};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, MapAccess, Visitor},
    ser::SerializeStruct,
};
use std::path::PathBuf;

impl Serialize for Record {
    /// Serialize this record into the given Serde serializer.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::Value;
    /// use app_lib::Workspace;
    ///
    /// let record = Workspace::new("/path/to/workspace").record("act/ing");
    /// let json_string = serde_json::to_string(&record).unwrap();
    /// let json: Value = serde_json::from_str(&json_string).unwrap();
    ///
    /// assert_eq!(json["path"], "/path/to/workspace/act/ing");
    /// assert_eq!(json["name"], "ing");
    /// assert_eq!(json["workspace"]["path"], "/path/to/workspace");
    /// assert_eq!(json["workspace"]["name"], "workspace");
    /// ```
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Record", 3)?;
        state.serialize_field("path", &self.path())?;
        state.serialize_field("name", &self.name())?;
        state.serialize_field("workspace", &self.workspace())?;
        state.end()
    }
}

/// The [RecordVisitor] struct is used to deserialize a [Record] from a map.
pub struct RecordVisitor;

impl<'de> Visitor<'de> for RecordVisitor {
    type Value = Record;

    /// Format a message stating what data the workspace visitor expects to receive.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .write_str("a map with fields `workspace` and `record`, representing a record object")
    }

    /// The input contains a key-value map.
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut workspace = None;
        let mut record = None;

        while let Some(key) = map.next_key()? {
            match key {
                "workspace" => {
                    if workspace.is_some() {
                        return Err(A::Error::duplicate_field("workspace"));
                    }
                    workspace = Some(map.next_value::<Workspace>()?);
                }
                "record" => {
                    if record.is_some() {
                        return Err(A::Error::duplicate_field("record"));
                    }
                    record = Some(map.next_value::<PathBuf>()?);
                }
                _ => {
                    return Err(A::Error::unknown_field(key, &["workspace", "record"]));
                }
            }
        }

        let workspace = workspace.ok_or_else(|| A::Error::missing_field("workspace"))?;
        let record_path = record.ok_or_else(|| A::Error::missing_field("record"))?;

        // (security) input sanitization
        if record_path.is_absolute() {
            return Err(A::Error::custom("record path must be relative"));
        }

        Ok(workspace.record(record_path))
    }
}

impl<'de> Deserialize<'de> for Record {
    /// Deserialize this record from the given Serde deserializer.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use app_lib::Record;
    ///
    /// let json = r#"{"workspace":"/path/to/parent-workspace", "record":"path/to/recordion"}"#;
    /// let record: Record = serde_json::from_str(json).unwrap();
    ///
    /// assert_eq!(record.workspace().name(), "parent-workspace");
    /// assert_eq!(record.workspace().path(), &PathBuf::from("/path/to/parent-workspace"));
    /// assert_eq!(record.name(), "recordion");
    /// assert_eq!(record.path(), PathBuf::from("/path/to/parent-workspace/path/to/recordion"));
    /// ```
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(RecordVisitor)
    }
}
