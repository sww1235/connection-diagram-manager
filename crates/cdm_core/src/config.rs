use std::path::PathBuf;

use miniquad::conf::Conf as mqConf;
use serde::{Deserialize, Serialize};

/// `Config` represents configuration options for the various cdm binary programs.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "this is a configuration struct with lots of boolean options"
)]
#[expect(clippy::module_name_repetitions, reason = "Distinquish from other config structs")]
#[non_exhaustive]
pub struct ApplicationConfig {
    /// `default_library_locations` contains paths to search for TOML library files to read into the
    /// main [`Library`](crate::datatypes::internal_types::Library).
    ///
    /// If a path is a directory, all TOML files found within the directory and its sub-directories
    /// will be parsed as library files.
    ///
    /// Hidden paths will be ignored.
    pub default_library_locations: Vec<PathBuf>,
    /// Enable postgres database features.
    pub enable_post_gres: bool,
    /// `PostGres` DSN.
    pub post_gres_dsn: String,
    /// Default Area unit for Display.
    pub default_area_unit: String,
    /// Default Length Unit for Display.
    pub default_length_unit: String,
    /// used for cross sectional area of wires for Display.
    pub default_cross_section_area_unit: String,
    /// Default Electrical Potential Unit for Display.
    pub default_electrical_potential_unit: String,
    /// Default Temperature Interval Unit for Display.
    pub default_temperature_interval_unit: String,
    /// use AWG instead of `default_cross_sectional_area_unit` for Display.
    pub use_awg: bool,
    /// If set, will set any default display units to USA customary units
    /// unless specifically set with other settings.
    ///
    /// - Area: square inch
    /// - Length: inch
    /// - Cross Section Area: AWG/circular mils
    /// - Electrical Potential: Volt
    /// - Temperature Interval: Degree Farenheit
    pub use_usa_customary_units: bool,
    /// If this is set, adjust prefixes or units used in display
    /// so there is no more than 3 digits before the decimal place.
    /// If it is not set, will display all units in their default units only.
    pub use_engineering_prefixes: bool,
    #[cfg(feature = "gui")]
    /// Graphics configuration options.
    pub graphics_config: GraphicsConfig,
    // all items below this are from CLI parsing. Ideally this would be in its own struct, but not
    // sure how to properly embed that into the core AppConfig struct with figment.
    /// Export complete PDF.
    pub export_pdf: bool,
    /// Do not use default libraries included with program.
    pub no_default_libs: bool,
    /// Directory that project lives in.
    pub project_directory: Option<PathBuf>,
    /// Increase verbosity of program by adding more v.
    pub verbose: u8,
    /// Only shows log messages with `Error` level. Use twice to completely eliminate output. Takes
    /// precidence over verbose.
    pub quiet: u8,
}

impl Default for ApplicationConfig {
    #[inline]
    fn default() -> Self {
        Self {
            default_library_locations: Vec::new(),
            enable_post_gres: false,
            post_gres_dsn: String::new(),
            default_area_unit: "mm²".to_owned(),
            default_length_unit: "mm".to_owned(),
            default_cross_section_area_unit: "mm²".to_owned(),
            default_electrical_potential_unit: "V".to_owned(),
            default_temperature_interval_unit: "°C".to_owned(),
            use_awg: false,
            use_usa_customary_units: false,
            use_engineering_prefixes: true,
            graphics_config: GraphicsConfig::default(),
            export_pdf: false,
            no_default_libs: false,
            project_directory: None,
            verbose: 0,
            quiet: 0,
        }
    }
}

#[cfg(feature = "gui")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[expect(clippy::module_name_repetitions, reason = "Specialized config struct")]
#[non_exhaustive]
/// Graphics configuration options.
pub struct GraphicsConfig {
    /// Starting window height.
    pub starting_window_height: i32,
    /// Starting window width.
    pub starting_window_width: i32,
    /// Enable high DPI features.
    pub high_dpi: bool,
}

impl Default for GraphicsConfig {
    #[inline]
    fn default() -> Self {
        Self {
            starting_window_height: 1024,
            starting_window_width: 1024,
            high_dpi: false,
        }
    }
}

impl From<GraphicsConfig> for mqConf {
    #[inline]
    fn from(input: GraphicsConfig) -> Self {
        Self {
            window_height: input.starting_window_height,
            window_width: input.starting_window_width,
            high_dpi: input.high_dpi,
            ..Default::default()
        }
    }
}
