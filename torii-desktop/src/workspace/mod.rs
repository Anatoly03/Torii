//! This module contains the [Workspace] struct.

mod serde;

use crate::Record;
use std::path::PathBuf;
use torii_desktop_macro::ts_bind;

/// The workspace struct represents a Torii workspace.
#[ts_bind()]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Workspace {
    /// The file system path to the workspace.
    path: PathBuf,

    /// The name of the workspace, which is the last component of the workspace path.
    #[ts_only]
    name: String,
}

impl Workspace {
    /// Creates a new workspace instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use app_lib::Workspace;
    ///
    /// let path = PathBuf::from("/path/to/workspace");
    /// let workspace1 = Workspace::new(path);
    /// let workspace2 = Workspace::new("/path/to/workspace");
    /// assert_eq!(workspace1.path(), workspace2.path());
    /// ```
    #[inline]
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        Self { path: path.into() }
    }

    /// Returns the file system path to the workspace.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use app_lib::Workspace;
    ///
    /// let workspace = Workspace::new("/path/to/workspace");
    /// assert_eq!(workspace.path(), &PathBuf::from("/path/to/workspace"));
    /// ```
    #[inline]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns the name of the workspace.
    ///
    /// # Example
    ///
    /// ```
    /// use app_lib::Workspace;
    ///
    /// let workspace = Workspace::new("/path/to/workspace");
    /// assert_eq!(workspace.name(), "workspace");
    /// ```
    #[inline]
    pub fn name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
    }

    /// Creates a new [Record] instance which is contained by this workspace.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use app_lib::Workspace;
    ///
    /// let workspace = Workspace::new("/path/to/workspace");
    /// let record = workspace.record("record1/record2");
    ///
    /// assert_eq!(record.name(), "record2");
    /// assert_eq!(record.path(), PathBuf::from("/path/to/workspace/record1/record2"));
    /// ```
    #[inline]
    pub fn record<T: Into<PathBuf>>(&self, path: T) -> Record {
        Record::new(self, path.into())
    }
}

/// Implements the `From` trait for converting a reference to a [Workspace] into an
/// owned [Workspace]. This allows for generic functions that accept either a move
/// or a reference.
impl From<&Workspace> for Workspace {
    /// Converts to a reference to a workspace to an owned workspace.
    #[inline]
    fn from(other: &Workspace) -> Self {
        other.clone()
    }
}
