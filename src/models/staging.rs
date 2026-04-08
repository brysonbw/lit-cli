use std::{
    fs,
    io::{ErrorKind, Result, Write},
    path::{Path, PathBuf},
};

use chroma_print::{Color, print_error};

use crate::types::{ProcessCommitEntryResult, StagingResult};

pub const STAGING_PATH: &str = ".staging";

/// A temporary workspace for safely preparing files before moving them to the final destination
pub struct StagingArea {
    staging_path: PathBuf,
    destination_path: PathBuf,
    created_destination: bool,
    committed: bool,
}

impl StagingArea {
    pub fn new(destination_path: &Path) -> StagingResult<Self> {
        let staging_path = destination_path.join(STAGING_PATH);
        // Flag to track the creation of the destination directory (use later to know whether to clean it up on failure)
        let created_destination = !destination_path.exists();

        let setup_dirs = || -> Result<()> {
            fs::create_dir_all(destination_path)?;
            if staging_path.exists() {
                fs::remove_dir_all(&staging_path)?;
            }
            fs::create_dir_all(&staging_path)?;

            return Ok(());
        };

        // If failure, clean up any created directories, since the Drop implementation won't run if we never successfully create the StagingArea instance
        if let Err(error) = setup_dirs() {
            if created_destination && destination_path.exists() {
                let _ = fs::remove_dir_all(destination_path);
            }
            return Err(error.into());
        }

        return Ok(Self {
            staging_path,
            destination_path: destination_path.to_path_buf(),
            created_destination,
            committed: false,
        });
    }

    /// Writes a file to the staging area (creating subfolders if needed)
    pub fn add_file(&self, relative_path: &str, content: &str) -> StagingResult<()> {
        let full_path = self.staging_path.join(relative_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut f = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&full_path)?;

        f.write_all(content.as_bytes())?;
        f.sync_all()?;

        return Ok(());
    }

    /// Moves all files from staging to the destination
    pub fn commit(mut self, destination: &Path) -> StagingResult<()> {
        // Ensure directories are valid beforehand to avoid partial commits
        if !self.staging_path.exists() || !self.staging_path.is_dir() {
            return Err("Staging path is invalid".into());
        }
        if !destination.exists() {
            fs::create_dir_all(destination)?;
        }

        let root_name = destination
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "app".to_string()); // Default to `app` if we can't determine a name

        let entries = fs::read_dir(&self.staging_path)?;

        for entry in entries {
            let entry = entry?;
            self.process_entry(&entry.path(), destination, &self.staging_path, &root_name)?;
        }

        self.committed = true;
        fs::remove_dir_all(&self.staging_path)?;

        return Ok(());
    }

    /// Recursively processes each entry in the staging area, moving it to the target destination while printing the created files
    fn process_entry(
        &self,
        current_path: &Path,
        destination_root: &Path,
        staging_root: &Path,
        project_name: &str,
    ) -> ProcessCommitEntryResult {
        let file_name = current_path.file_name().ok_or(ErrorKind::InvalidInput)?;
        let target_path = destination_root.join(file_name);
        let metadata = fs::metadata(current_path)?;

        if metadata.is_dir() {
            if target_path.exists() && target_path.is_file() {
                fs::remove_file(&target_path)?;
            }

            fs::create_dir_all(&target_path)?;

            for entry in fs::read_dir(current_path)? {
                let entry = entry?;
                self.process_entry(&entry.path(), &target_path, staging_root, project_name)?;
            }

            fs::remove_dir_all(current_path)?;
        } else {
            // For Windows compatibility, ensure overwriting existing files/directories at the target path
            if target_path.exists() {
                if target_path.is_dir() {
                    fs::remove_dir_all(&target_path)?;
                } else {
                    fs::remove_file(&target_path)?;
                }
            }

            fs::rename(current_path, &target_path)?;

            // Strip the staging prefix (e.g. <staging_path>/src/<file_name> -> src/<file_name>)
            let relative_path = current_path
                .strip_prefix(staging_root)
                .unwrap_or(current_path);

            // Prepend the project name (e.g. app/src/<file_name>)
            let display_path = format!("{}/{}", project_name, relative_path.display());

            self.print_create(&display_path, metadata.len());
        }

        return Ok(());
    }

    /// Prints a formatted message indicating a file was created, including its path and size (in bytes)
    fn print_create(&self, display_path: &str, size: u64) {
        println!(
            "{}CREATE{} {}{}{} ({} bytes)",
            Color::Green.value(),
            Color::Reset.value(),
            Color::Cyan.value(),
            display_path,
            Color::Reset.value(),
            size
        );
    }

    /// Gets staging path (for testing purposes only)
    #[cfg(test)]
    pub fn staging_path(&self) -> &Path {
        return &self.staging_path;
    }
}

/// The Drop implementation for StagingArea
/// If panicking or returning early, this runs automatically and cleans up both the staging and destination paths, ensuring no partial files are left behind
impl Drop for StagingArea {
    fn drop(&mut self) {
        if !self.committed {
            if self.staging_path.exists() {
                let _ = fs::remove_dir_all(&self.staging_path);
            }
            if self.created_destination && self.destination_path.exists() {
                let _ = fs::remove_dir_all(&self.destination_path);
            }
            print_error!("Generation aborted:");
        }
    }
}
