use num_traits::ToPrimitive as _;
use serde::{Deserialize, Serialize};

use super::IntermediateUnit;
use crate::error::UnitParsingError;

/// Struct representing Nominal wire size which may be different than its actual
/// `CrossSectionalArea`
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntermediateUnit")]
#[non_exhaustive]
pub struct NominalWireSize {
    /// contained uom Unit
    pub value: NominalWireUnit,
    /// original unit in datafile
    pub original_unit: String,
}

//TODO: replace f64 with a fixed/decimal equivalent type
/// Represents common nominal units for wire sizes
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum NominalWireUnit {
    /// Nominal size represented in American Wire Gauge (AWG)
    Awg(f64),
    /// Nominal size represented in mm²
    Mm2(f64),
    /// Nominal size represented in Circular Mil
    Cmil(f64),
}

impl Default for NominalWireUnit {
    #[inline]
    fn default() -> Self {
        Self::Mm2(f64::default())
    }
}

impl NominalWireSize {
    /// outputs all usable `NominalWireSize` units allowed in configuration files in the form of
    /// `<unit name>: <unit abbreviation>`
    #[must_use]
    #[expect(clippy::string_add, reason = "easier and cleaner than one massive format string")]
    #[inline]
    pub fn output_units() -> String {
        // need to do this hack AFAIK so the spacing is the same
        let string1 = "Unit Name";
        let string2 = "Abbreviation";
        let dash_string = "-".repeat(21);
        format!("{string1:^21}|{string2:^21}\n{dash_string}|{dash_string}\n")
            + format!("{:^21}|{:^21}\n", "American Wire Gauge", "awg").as_str()
            + format!("{:^21}|{:^21}\n", "square millimeter", "mm²").as_str()
            + format!("{:^21}|{:^21}\n", "circular mil", "cmil").as_str()
            + format!("{:^21}|{:^21}\n", "thousand circular mil", "kcmil").as_str()
            + format!("{:^21}|{:^21}\n", "thousand circular mil", "MCM").as_str()
    }
}

impl TryFrom<IntermediateUnit> for NominalWireSize {
    type Error = UnitParsingError;
    #[inline]
    fn try_from(item: IntermediateUnit) -> Result<Self, Self::Error> {
        match item.original_unit.to_uppercase().as_str() {
            "AWG" | "American Wire Gauge" => Ok(Self {
                value: NominalWireUnit::Awg(item.value.to_f64().ok_or(UnitParsingError::ValueError {
                    quantity_type: "Nominal Wire Size".to_owned(),
                    data_type: "f64".to_owned(),
                })?),
                original_unit: item.original_unit,
            }),
            "MM²" => Ok(Self {
                value: NominalWireUnit::Mm2(item.value.to_f64().ok_or(UnitParsingError::ValueError {
                    quantity_type: "Nominal Wire Size".to_owned(),
                    data_type: "f64".to_owned(),
                })?),
                original_unit: item.original_unit,
            }),
            "CMIL" => Ok(Self {
                value: NominalWireUnit::Cmil(item.value.to_f64().ok_or(UnitParsingError::ValueError {
                    quantity_type: "Nominal Wire Size".to_owned(),
                    data_type: "f64".to_owned(),
                })?),
                original_unit: item.original_unit,
            }),
            "KCMIL" | "MCM" => Ok(Self {
                value: NominalWireUnit::Cmil(
                    item.value.to_f64().ok_or(UnitParsingError::ValueError {
                        quantity_type: "Nominal Wire Size".to_owned(),
                        data_type: "f64".to_owned(),
                    })? / 1000.0,
                ),
                original_unit: item.original_unit,
            }),

            x => Err(UnitParsingError::UnknownUnit {
                unit_string: x.to_owned(),
                quantity_type: "Nominal Wire Size".to_owned(),
            }),
        }
    }
}
