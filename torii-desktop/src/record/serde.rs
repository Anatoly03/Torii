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
        let mut state = serializer.serialize_struct("Record", 4)?;
        state.serialize_field("path", &self.path())?;
        state.serialize_field("relative_path", &self.relative_path())?;
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
    /// 
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use app_lib::Record;
    ///
    /// let json = r#"{"workspace":"/path/to/parent-workspace", "path":"/path/to/parent-workspace/path/to/recordion"}"#;
    /// let record: Record = serde_json::from_str(json).unwrap();
    /// assert_eq!(record.path(), PathBuf::from("/path/to/parent-workspace/path/to/recordion"));
    /// assert_eq!(record.relative_path(), &PathBuf::from("path/to/recordion"));
    /// ```
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut workspace = None;
        let mut record_path = None;
        let mut record_relative_path = None;

        while let Some(key) = map.next_key()? {
            match key {
                "workspace" => {
                    if workspace.is_some() {
                        return Err(A::Error::duplicate_field("workspace"));
                    }
                    workspace = Some(map.next_value::<Workspace>()?);
                }
                "path" => {
                    if record_path.is_some() {
                        return Err(A::Error::duplicate_field("record"));
                    }
                    record_path = Some(map.next_value::<PathBuf>()?);
                }
                "relative_path" => {
                    if record_relative_path.is_some() {
                        return Err(A::Error::duplicate_field("relative_path"));
                    }
                    record_relative_path = Some(map.next_value::<PathBuf>()?);
                }
                _ => {
                    let _: serde_json::Value = map.next_value()?;
                }
            }
        }

        let workspace = workspace.ok_or_else(|| A::Error::missing_field("workspace"))?;
        let relative_path = record_relative_path
            .map(|rpath| match rpath.is_absolute() {
                true => Err(A::Error::custom("`relative_path` must not be absolute")),
                false => Ok(rpath),
            })
            .or(record_path.map(|path| {
                path.strip_prefix(workspace.path())
                    .map_err(|_| A::Error::custom("record path is not relative to workspace"))
                    .map(|p| p.to_path_buf())
            }))
            .unwrap_or_else(|| Err(A::Error::missing_field("relative_path")))?;

        Ok(workspace.record(relative_path))
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
    /// let json = r#"{"workspace":"/path/to/parent-workspace", "relative_path":"path/to/recordion"}"#;
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
