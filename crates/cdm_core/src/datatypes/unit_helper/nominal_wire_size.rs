use serde::{Deserialize, Serialize};

use super::IntermediateUnit;
use crate::error::UnitParsingError;

/// Struct representing Nominal wire size which may be different than its actual
/// `CrossSectionalArea`
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntermediateUnit")]
pub struct NominalWireSize {
    /// contained uom Unit
    pub value: NominalWireUnit,
    /// original unit in datafile
    pub original_unit: String,
}

//TODO: replace f64 with a fixed/decimal equivalent type
/// Represents common nominal units for wire sizes
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NominalWireUnit {
    /// Nominal size represented in American Wire Gauge (AWG)
    Awg(f64),
    /// Nominal size represented in mm²
    Mm2(f64),
    /// Nominal size represented in Circular Mil
    Cmil(f64),
}

impl Default for NominalWireUnit {
    fn default() -> Self {
        Self::Mm2(f64::default())
    }
}

impl TryFrom<IntermediateUnit> for NominalWireSize {
    type Error = UnitParsingError;
    #[expect(clippy::too_many_lines, clippy::match_same_arms)]
    fn try_from(item: IntermediateUnit) -> Result<Self, Self::Error> {
        match item.original_unit.as_str() {
            //"YK" | "yottakelvin" => Ok(Self {
            //    value: rational64::TemperatureInterval::new::<yottakelvin>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "YK" | "yottakelvin" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Temperature Interval".to_string(),
            }),
            //"ZK" | "zettakelvin" => Ok(Self {
            //    value: rational64::TemperatureInterval::new::<zettakelvin>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "ZK" | "zettakelvin" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Temperature Interval".to_string(),
            }),
            "EK" | "exakelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<exakelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "PK" | "petakelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<petakelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "TK" | "terakelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<terakelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "GK" | "gigakelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<gigakelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "MK" | "megakelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<megakelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "kK" | "kilokelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<kilokelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "hK" | "hectokelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<hectokelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "daK" | "decakelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<decakelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "K" | "kelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<kelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "dK" | "decikelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<decikelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "cK" | "centikelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<centikelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "mK" | "millikelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<millikelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "µK" | "microkelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<microkelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "nK" | "nanokelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<nanokelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "pK" | "picokelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<picokelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "fK" | "femtokelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<femtokelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "aK" | "attokelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<attokelvin>(item.value),
                original_unit: item.original_unit,
            }),
            //"zK" | "zeptokelvin" => Ok(Self {
            //    value: rational64::TemperatureInterval::new::<zeptokelvin>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "zK" | "zeptokelvin" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Temperature Interval".to_string(),
            }),
            //"yK" | "yoctokelvin" => Ok(Self {
            //    value: rational64::TemperatureInterval::new::<yoctokelvin>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "yK" | "yoctokelvin" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Temperature Interval".to_string(),
            }),
            "°C" | "degree Celsius" => Ok(Self {
                value: rational64::TemperatureInterval::new::<degree_celsius>(item.value),
                original_unit: item.original_unit,
            }),
            "°F" | "degree Fahrenheit" => Ok(Self {
                value: rational64::TemperatureInterval::new::<degree_fahrenheit>(item.value),
                original_unit: item.original_unit,
            }),
            "°R" | "degree Rankine" => Ok(Self {
                value: rational64::TemperatureInterval::new::<degree_rankine>(item.value),
                original_unit: item.original_unit,
            }),

            x => Err(UnitParsingError::UnknownUnit {
                unit_string: x.to_string(),
                quantity_type: "Temperature Interval".to_string(),
            }),
        }
    }
}
