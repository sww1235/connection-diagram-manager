use std::path::PathBuf;

/// Per Project configuration options.
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct ProjectConfiguration {
    /// Name of Project used for display and drawings.
    pub project_name: String,
    /// If default libraries included with the application should
    /// be loaded in addition to the libraries defined in the project file
    /// If neither is defined, then no libraries will be loaded and nothing will work.
    pub load_default_libraries: Option<bool>,
    /// Vector of library paths to include in the project
    /// If a path listed is a directory, all TOML files found within the directory
    /// and subdirectories will be included as libraries in the project
    /// Hidden paths are ignored.
    pub library_paths: Option<Vec<PathBuf>>,
    /// Vector of source paths to include in the project
    /// If a path listed is a directory, all TOML files found within the directory
    /// and subdirectories will be included as sources in the project
    /// Hidden paths are ignored.
    pub source_paths: Option<Vec<PathBuf>>,
    //TODO: see if this can be changed from String to a dedicated struct
    /// Code reference used for ampacity checks and conduit fill.
    pub electrical_code_standard: Option<String>,
    /// IEC project code.
    pub project_code: Option<String>,
    /// Optional project description.
    pub description: Option<String>,
}
