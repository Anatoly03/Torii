//! This module contains the [Record] struct.

pub mod commands;
mod serde;

use crate::{Component, components::get_all_components, workspace::Workspace};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

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
        assert!(
            !path.is_absolute(),
            "record path must be relative: {}",
            path.display()
        );

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

    /// Returns the file system path to the record, relative to the [Workspace]
    /// directory.
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
    /// assert_eq!(record.relative_path(), PathBuf::from("media/record1"));
    /// ```
    #[inline]
    pub fn relative_path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns the name of the record.
    ///
    /// # Example
    ///
    /// ```
    /// use app_lib::Workspace;
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
            // .filter(|p| p.is_file())
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
            .filter_map(|path| {
                let name = path
                    .file_prefix()
                    .map(|f| f.to_string_lossy().into_owned())?;

                // (security) skip config files. make sure the magic dot character is not
                // present in the record name
                match name.contains('.') {
                    true => None,
                    false => Some(name),
                }
            })
            .collect::<HashSet<_>>();
        let records = record_names
            .into_iter()
            .map(|name| Self::new(Workspace::new(directory.clone()), name))
            .collect();
        Ok(records)
    }

    /// Returns true if the record has the given component attached to it.
    pub fn has_component(&self, component: &Box<dyn Component>) -> bool {
        component.is_attached(&self)
    }

    /// Lists the components attached to a specific record.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_test,no_run
    /// //! The following contents are included in folder "workspace":
    /// //!
    /// //! workspace/
    /// //! ├── record1.md
    /// //! └── record1.png
    ///
    /// use app_lib::{Record, Workspace};
    ///
    /// let workspace = Workspace::new("/path/to/workspace");
    /// let record = workspace.record("record1");
    /// let components = record.list_components().unwrap_or(vec![]);
    ///
    /// assert!(components.contains(&"article".to_string()));
    /// assert!(components.contains(&"image".to_string()));
    /// assert!(!components.contains(&"banner".to_string()));
    /// assert!(!components.contains(&"folder".to_string()));
    /// ```
    pub fn list_components(&self) -> Vec<String> {
        get_all_components()
            .iter()
            .filter_map(|comp| match self.has_component(comp) {
                true => Some(comp.component_name().to_string()),
                false => None,
            })
            .collect()
    }

    /// Moves the record to a new path. The `new_path` argument is the new file path
    /// relative to the workspace root.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_test,no_run
    /// //! The following is a logical example.
    /// //!
    /// //! workspace/
    /// //! ├── characters/
    /// //! │   ├── Sarah Vermillion.md
    /// //! │   ├── Edmund Bienenwolf.md
    /// //! │   └── New Location.md
    /// //! ├── locations/
    /// //! │   ├── The Akademiya.md
    /// //! │   └── Thethys Sea.md
    /// //! ├── New Character.md
    /// //! └── New Character.banner.png
    ///
    /// use std::path::PathBuf;
    /// use app_lib::{Record, Workspace};
    ///
    /// let workspace = Workspace::new("/path/to/workspace");
    /// let new_location = workspace.record("characters/New Location");
    /// let new_character = workspace.record("New Character");
    ///
    /// assert_eq(new_location.relative_path(), PathBuf::from("characters/New Location"));
    /// assert_eq(new_character.relative_path(), PathBuf::from("New Character"));
    ///
    /// let renewed_location = new_location.rename(PathBuf::from("locations/New Location")).unwrap();
    /// let renewed_character = new_character.rename(PathBuf::from("characters/New Character")).unwrap();
    ///
    /// assert_eq(renewed_location.relative_path(), PathBuf::from("locations/New Location"));
    /// assert_eq(renewed_character.relative_path(), PathBuf::from("characters/New Character"));
    /// ```
    pub fn rename(&self, new_path: PathBuf) -> Result<Self, std::io::Error> {
        let workspace_path = self.workspace.path();
        let paths = self.associated_paths()?;

        // Move all record-associated paths to the new location.
        for old_path in paths {
            // `old_path` - old absolute file path
            // `new_path` - new file path relative to the workspace root
            // `new_record_path` - new absolute file path, without proper
            //            extensions, only the name preserved.
            let new_record_path = workspace_path.join(&new_path);
            // `old_path_suffix` - the suffix of the old file path, including
            //            the dot. The suffix is: if the file starts with a dot,
            //            everything after the second dot, otherwise everything
            //            after the first dot. For example:
            //
            // New Record.md
            //           ^^^ suffix
            // New Record.banner.png
            //           ^^^^^^^^^^^ suffix
            let old_path_suffix = {
                let old_file = old_path.file_name().ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "old path has no file name")
                })?;

                if old_file.to_string_lossy().starts_with('.') {
                    unimplemented!(
                        "rename: old path starts with a dot (magic file), not supported yet"
                    )
                }

                let old_file_str = old_file.to_string_lossy();
                let suffix_start = old_file_str.find('.').ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "old path has no suffix")
                })?;

                old_file_str[(suffix_start + 1)..].to_string()
            };
            // `new_file_path` - new absolute file path, with proper extension
            //            preserved.
            let new_file_path = new_record_path.with_added_extension(&old_path_suffix);

            std::fs::rename(&old_path, &new_file_path)?;

            // If the old path was a directory, we need to clean up (remove) the old
            // directory because [std::fs::rename] only moves its' contents, keeping
            // the old directory intact.
            if old_path.is_dir() {
                std::fs::remove_dir(&old_path)?;
            }
        }

        // println!(
        //     "Renamed record from {} to {}",
        //     self.path().display(),
        //     new_path.display()
        // );
        Ok(Self::new(self.workspace.clone(), new_path))
    }
}
