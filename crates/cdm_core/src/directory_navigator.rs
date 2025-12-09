use std::{
    fs,
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

/// Recursively walk the provided directory and return paths to all files contained within.
///
/// # Errors
///
/// Will error if provided path is not a directory or if there are IO errors encountered when
/// reading the directory
#[inline]
pub fn files_in_dir<T>(dir: T, extension: Option<&str>, follow_symlinks: bool) -> Result<Vec<PathBuf>>
where T: AsRef<Path> {
    if !dir.as_ref().is_dir() {
        return Err(Error::from(ErrorKind::NotADirectory));
    }
    let mut file_paths: Vec<PathBuf> = Vec::new();
    files_in_dir_inner(dir, &mut file_paths, extension, follow_symlinks)?;
    Ok(file_paths)
}

/// Inner function for recursively walking a directory
///
/// Adds each path found to the `paths` `Vec` as canonicalized path.
///
/// Can optionally filter the returned files by file extension
///
/// Can filter out symlinks.
fn files_in_dir_inner<T>(dir: T, paths: &mut Vec<PathBuf>, extension: Option<&str>, follow_symlinks: bool) -> Result<()>
where T: AsRef<Path> {
    if dir.as_ref().is_dir() {
        for entry in fs::read_dir(dir)? {
            #[expect(clippy::shadow_reuse, reason = "unwrapping value")]
            let entry = entry?;
            let path = entry.path();
            if path.is_symlink() && !follow_symlinks {
                continue;
            }
            if path.is_dir() {
                files_in_dir_inner(path, paths, extension, follow_symlinks)?;
            } else if let Some(test_extension) = extension
                && let Some(file_extension) = path.extension()
                && test_extension == file_extension
            {
                paths.push(path.canonicalize()?);
            } else if extension.is_none() {
                paths.push(path.canonicalize()?);
            } else {
                // allowing clippy::needless_else with this comment
                //
            }
        }
        Ok(())
    } else {
        Err(Error::from(ErrorKind::NotADirectory))
    }
}
