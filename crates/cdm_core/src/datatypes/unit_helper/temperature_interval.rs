use serde::{Deserialize, Serialize};
use uom::si::{Unit, rational64, temperature_interval::*};

use super::IntermediateUnit;
use crate::error::UnitParsingError;

/// Struct representing `TemperatureInterval` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntermediateUnit")]
pub struct TemperatureInterval {
    /// contained uom Unit
    pub value: rational64::TemperatureInterval,
    /// original unit in datafile
    pub original_unit: String,
}

impl TemperatureInterval {
    /// outputs all usable `TemperatureInterval` units allowed in configuration files in the form of
    /// `<unit name>: <unit abbreviation>`
    #[must_use]
    #[expect(clippy::string_add)]
    pub fn output_units() -> String {
        let string1 = "Unit Name";
        let string2 = "Abbreviation";
        let dash_string = "-".repeat(21);
        format!("{string1:^21}|{string2:^21}\n{dash_string}|{dash_string}\n")
            //    commented units currently unsupported due to bug with uom. iliekturtles/uom#60
            //+ format!("{:^21}|{:^21}\n", yottakelvin::singular(), yottakelvin::abbreviation()).as_str()
            //+ format!("{:^21}|{:^21}\n", zettakelvin::singular(), zettakelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", exakelvin::singular(), exakelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", petakelvin::singular(), petakelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", terakelvin::singular(), terakelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", gigakelvin::singular(), gigakelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", megakelvin::singular(), megakelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", kilokelvin::singular(), kilokelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", hectokelvin::singular(), hectokelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", decakelvin::singular(), decakelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", kelvin::singular(), kelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", decikelvin::singular(), decikelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", centikelvin::singular(), centikelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", millikelvin::singular(), millikelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", microkelvin::singular(), microkelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", nanokelvin::singular(), nanokelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", picokelvin::singular(), picokelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", femtokelvin::singular(), femtokelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", attokelvin::singular(), attokelvin::abbreviation()).as_str()
            //    commented units currently unsupported due to bug with uom. iliekturtles/uom#60
            //+ format!("{:^21}|{:^21}\n", zeptokelvin::singular(), zeptokelvin::abbreviation()).as_str()
            //+ format!("{:^21}|{:^21}\n", yoctokelvin::singular(), yoctokelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", degree_celsius::singular(), degree_celsius::abbreviation()).as_str()
            + format!(
                "{:^21}|{:^21}\n",
                degree_fahrenheit::singular(),
                degree_fahrenheit::abbreviation()
            )
            .as_str()
            + format!("{:^21}|{:^21}\n", degree_rankine::singular(), degree_rankine::abbreviation()).as_str()
    }
}

//TODO: return a different error if the unit is of the wrong type rather than just unknown unit
impl TryFrom<IntermediateUnit> for TemperatureInterval {
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
