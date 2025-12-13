use std::path::PathBuf;

use miniquad::conf::Conf as mqConf;
use serde::{Deserialize, Serialize};

/// `Config` represents configuration options for the various cdm binary programs
#[derive(Serialize, Deserialize, Debug, Clone)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "this is a configuration struct with lots of boolean options"
)]
#[expect(clippy::module_name_repetitions, reason = "Distinquish from other config structs")]
#[non_exhaustive]
pub struct ApplicationConfig {
    /// `default_library_locations` contains paths to search for TOML library files to read into the
    /// main [`Library`](crate::datatypes::internal_types::Library)
    /// If a path is a directory, all TOML files found within the directory and its sub-directories
    /// will be parsed as library files
    /// Hidden paths will be ignored
    pub default_library_locations: Vec<PathBuf>,
    /// Enable postgres database features
    pub enable_post_gres: bool,
    /// `PostGres` DSN
    pub post_gres_dsn: String,
    /// Default Area unit for Display
    pub default_area_unit: String,
    /// Default Length Unit for Display
    pub default_length_unit: String,
    /// used for cross sectional area of wires for Display
    pub default_cross_section_area_unit: String,
    /// Default Electrical Potential Unit for Display
    pub default_electrical_potential_unit: String,
    /// Default Temperature Interval Unit for Display
    pub default_temperature_interval_unit: String,
    /// use AWG instead of `default_cross_sectional_area_unit` for Display
    pub use_awg: bool,
    /// If set, will set any default display units to USA customary units
    /// unless specifically set with other settings
    ///
    /// Area: square inch
    /// Length: inch
    /// Cross Section Area: AWG/circular mils
    /// Electrical Potential: Volt
    /// Temperature Interval: Degree Farenheit
    pub use_usa_customary_units: bool,
    /// If this is set, adjust prefixes or units used in display
    /// so there is no more than 3 digits before the decimal place
    /// If it is not set, will display all units in their default units only
    pub use_engineering_prefixes: bool,
    #[cfg(feature = "gui")]
    /// Graphics configuration options
    pub graphics_config: Option<GraphicsConfig>,
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
            graphics_config: None,
        }
    }
}

#[cfg(feature = "gui")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[expect(clippy::module_name_repetitions, reason = "Specialized config struct")]
#[non_exhaustive]
/// Graphics configuration options
pub struct GraphicsConfig {
    /// Starting window height
    pub window_height: i32,
    /// Starting window width
    pub window_width: i32,
    /// Enable high DPI features
    pub high_dpi: bool,
}

impl Default for GraphicsConfig {
    #[inline]
    fn default() -> Self {
        Self {
            window_height: 1024,
            window_width: 1024,
            high_dpi: true,
        }
    }
}

impl From<GraphicsConfig> for mqConf {
    #[inline]
    fn from(input: GraphicsConfig) -> Self {
        Self {
            window_height: input.window_height,
            window_width: input.window_width,
            high_dpi: input.high_dpi,
            ..Default::default()
        }
    }
}
