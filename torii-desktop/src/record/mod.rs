//! This module contains the [Record] struct.

pub mod commands;
mod serde;

use crate::workspace::Workspace;
use std::{collections::HashSet, path::PathBuf};

/// The record struct represents a Torii record.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Record {
    /// The [Workspace] that the record belongs to.
    workspace: Workspace,

    /// The file system path to the record, relative to the [Workspace] directory.
    path: PathBuf,
}

impl Record {
    /// Creates a new record instance which is member of the given [Workspace].
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use app_lib::{Record, Workspace};
    ///
    /// let workspace = Workspace::new("/path/to/workspace");
    /// let record1 = Record::new(&workspace, "record1");
    /// let record2 = Record::new(workspace, "media/record2");
    ///
    /// assert_eq!(record1.name(), "record1");
    /// assert_eq!(record2.name(), "record2");
    /// assert_eq!(record1.path(), PathBuf::from("/path/to/workspace/record1"));
    /// assert_eq!(record2.path(), PathBuf::from("/path/to/workspace/media/record2"));
    /// ```
    #[inline]
    pub fn new<R: Into<Workspace>, T: Into<PathBuf>>(workspace: R, path: T) -> Self {
        let path = path.into();

        #[cfg(debug_assertions)]
        assert!(!path.is_absolute(), "record path must be relative");

        Self {
            workspace: workspace.into(),
            path: path,
        }
    }

    /// Returns the [Workspace] that the record belongs to.
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use app_lib::{Record, Workspace};
    ///
    /// let workspace = Workspace::new("/path/to/workspace");
    /// let record = workspace.record("media/record1");
    ///
    /// assert_eq!(record.workspace().path(), &PathBuf::from("/path/to/workspace"));
    /// ```
    #[inline]
    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    /// Returns the full file system path to the record.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use app_lib::{Record, Workspace};
    ///
    /// let workspace = Workspace::new("/path/to/workspace");
    /// let record = workspace.record("media/record1");
    ///
    /// assert_eq!(record.path(), PathBuf::from("/path/to/workspace/media/record1"));
    /// ```
    #[inline]
    pub fn path(&self) -> PathBuf {
        self.workspace.path().join(&self.path)
    }

    /// Returns the name of the record.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use app_lib::{Record, Workspace};
    ///
    /// let workspace = Workspace::new("/path/to/workspace");
    /// let record = workspace.record("media/record1");
    ///
    /// assert_eq!(record.name(), "record1");
    /// ```
    #[inline]
    pub fn name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
    }

    /// Returns true if the record exists in the file system.
    ///
    /// A record is considered to exist if either of the following exists:
    ///
    /// - A directory with the same name as the record.
    /// - A file with the name being the prefix of the record name.
    pub fn exists(&self) -> bool {
        self.path()
            .parent()
            .map(|p| p.read_dir())
            .unwrap_or_else(|| {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "record path has no parent directory",
                ))
            })
            .map(|dir| {
                dir.filter_map(|e| e.ok())
                    .map(|entry| entry.path())
                    .any(|path| {
                        path.file_prefix()
                            .and_then(|s| s.to_str())
                            .is_some_and(|prefix| prefix == self.name())
                    })
            })
            .unwrap_or(false)
    }

    /// Returns the paths associated with the record.
    ///
    /// A path is considered to be associated with the record if the record name
    /// is its prefix, followed by a dot.
    pub fn associated_paths(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let paths = self
            .path()
            .parent()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "record path has no parent directory",
                )
            })?
            .read_dir()?
            .filter_map(|e| e.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .filter(|path| {
                path.file_prefix()
                    .and_then(|s| s.to_str())
                    .is_some_and(|prefix| prefix == self.name())
            })
            .collect();
        Ok(paths)
    }

    /// Returns the list of records located in the given directory.
    pub fn list(directory: PathBuf) -> Result<Vec<Self>, std::io::Error> {
        let record_names = directory
            .read_dir()?
            .filter_map(|e| e.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file() || path.is_dir())
            .filter_map(|path| path.file_prefix().map(|f| f.to_string_lossy().into_owned()))
            .collect::<HashSet<_>>();
        let records = record_names
            .into_iter()
            .map(|name| Self::new(Workspace::new(directory.clone()), name))
            .collect();
        Ok(records)
    }
}
