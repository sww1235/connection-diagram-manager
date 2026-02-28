#[cfg(windows)]
use std::fs;
#[cfg(windows)]
use std::os::windows::prelude::*;
use std::{io, path::Path};

#[cfg(windows)]
// from https://users.rust-lang.org/t/read-windows-hidden-file-attribute/51180/7
/// Checks to see if the provided path is a hidden file or not.
pub fn is_hidden(path: &Path) -> io::Result<bool> {
    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;

    let metadata = fs::metadata(path)?;
    let attributes = metadata.file_attributes();

    if (attributes & FILE_ATTRIBUTE_HIDDEN) != 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(unix)]
/// Checks to see if the provided path is a hidden file or not.
pub fn is_hidden(path: &Path) -> io::Result<bool> {
    let file_name = path.file_name().ok_or(io::Error::from(io::ErrorKind::InvalidFilename))?;
    let file_name_str = file_name
        .to_str()
        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Filename not valid UTF-8"))?;
    if file_name_str.starts_with('.') { Ok(true) } else { Ok(false) }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", unix)))]
/// Checks to see if the provided path is a hidden file or not.
pub fn is_hidden(path: &Path) -> io::Result<bool> {
    return Err(io::Error::new(io::ErrorKind::Other, "Unsupported OS"));
}
