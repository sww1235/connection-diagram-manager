use serde::{Deserialize, Serialize};
use uom::si::{Unit, electric_potential::*, rational64};

use super::IntermediateUnit;
use crate::error::UnitParsingError;

/// Struct representing `Electric Potential` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntermediateUnit")]
pub struct ElectricPotential {
    /// contained uom Unit
    pub value: rational64::ElectricPotential,
    /// original unit in datafile
    pub original_unit: String,
}

impl ElectricPotential {
    /// outputs all usable `ElectricPotential` units allowed in configuration files in the form of
    /// `<unit name>: <unit abbreviation>`
    #[must_use]
    #[expect(clippy::string_add)]
    pub fn output_units() -> String {
        let string1 = "Unit Name";
        let string2 = "Abbreviation";
        let dash_string = "-".repeat(21);
        format!("{string1:^21}|{string2:^21}\n{dash_string}|{dash_string}\n")
            //    commented units currently unsupported due to bug with uom. iliekturtles/uom#60
            //+ format!("{:^21}|{:^21}\n", yottavolt::singular(), yottavolt::abbreviation()).as_str()
            //+ format!("{:^21}|{:^21}\n", zettavolt::singular(), zettavolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", exavolt::singular(), exavolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", petavolt::singular(), petavolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", teravolt::singular(), teravolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", gigavolt::singular(), gigavolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", megavolt::singular(), megavolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", kilovolt::singular(), kilovolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", hectovolt::singular(), hectovolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", decavolt::singular(), decavolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", volt::singular(), volt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", decivolt::singular(), decivolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", centivolt::singular(), centivolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", millivolt::singular(), millivolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", microvolt::singular(), microvolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", nanovolt::singular(), nanovolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", picovolt::singular(), picovolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", femtovolt::singular(), femtovolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", attovolt::singular(), attovolt::abbreviation()).as_str()
            //    commented units currently unsupported due to bug with uom. iliekturtles/uom#60
            //+ format!("{:^21}|{:^21}\n", zeptovolt::singular(), zeptovolt::abbreviation()).as_str()
            //+ format!("{:^21}|{:^21}\n", yoctovolt::singular(), yoctovolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", abvolt::singular(), abvolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", statvolt::singular(), statvolt::abbreviation()).as_str()
    }
}

//TODO: return a different error if the unit is of the wrong type rather than just unknown unit
impl TryFrom<IntermediateUnit> for ElectricPotential {
    type Error = UnitParsingError;
    #[expect(clippy::match_same_arms)]
    fn try_from(item: IntermediateUnit) -> Result<Self, Self::Error> {
        match item.original_unit.as_str() {
            //"YV" | "yottavolt" => Ok(Self {
            //    value: rational64::ElectricPotential::new::<yottavolt>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "YV" | "yottavolt" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Electric Potential".to_string(),
            }),
            //"ZV" | "zettavolt" => Ok(Self {
            //    value: rational64::ElectricPotential::new::<zettavolt>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "ZV" | "zettavolt" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Electric Potential".to_string(),
            }),
            "EV" | "exavolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<exavolt>(item.value),
                original_unit: item.original_unit,
            }),
            "PV" | "petavolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<petavolt>(item.value),
                original_unit: item.original_unit,
            }),
            "TV" | "teravolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<teravolt>(item.value),
                original_unit: item.original_unit,
            }),
            "GV" | "gigavolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<gigavolt>(item.value),
                original_unit: item.original_unit,
            }),
            "MV" | "megavolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<megavolt>(item.value),
                original_unit: item.original_unit,
            }),
            "kV" | "kilovolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<kilovolt>(item.value),
                original_unit: item.original_unit,
            }),
            "hV" | "hectovolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<hectovolt>(item.value),
                original_unit: item.original_unit,
            }),
            "daV" | "decavolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<decavolt>(item.value),
                original_unit: item.original_unit,
            }),
            "V" | "volt" => Ok(Self {
                value: rational64::ElectricPotential::new::<volt>(item.value),
                original_unit: item.original_unit,
            }),
            "dV" | "decivolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<decivolt>(item.value),
                original_unit: item.original_unit,
            }),
            "cV" | "centivolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<centivolt>(item.value),
                original_unit: item.original_unit,
            }),
            "mV" | "millivolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<millivolt>(item.value),
                original_unit: item.original_unit,
            }),
            "µV" | "microvolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<microvolt>(item.value),
                original_unit: item.original_unit,
            }),
            "nV" | "nanovolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<nanovolt>(item.value),
                original_unit: item.original_unit,
            }),
            "pV" | "picovolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<picovolt>(item.value),
                original_unit: item.original_unit,
            }),
            "fV" | "femtovolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<femtovolt>(item.value),
                original_unit: item.original_unit,
            }),
            "aV" | "attovolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<attovolt>(item.value),
                original_unit: item.original_unit,
            }),
            //"zV" | "zeptovolt" => Ok(Self {
            //    value: rational64::ElectricPotential::new::<zeptovolt>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "zV" | "zeptovolt" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Electric Potential".to_string(),
            }),
            //"yV" | "yoctovolt" => Ok(Self {
            //    value: rational64::ElectricPotential::new::<yoctovolt>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "yV" | "yoctovolt" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Electric Potential".to_string(),
            }),
            "abV" | "abvolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<abvolt>(item.value),
                original_unit: item.original_unit,
            }),
            "statV" | "statvolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<statvolt>(item.value),
                original_unit: item.original_unit,
            }),

            x => Err(UnitParsingError::UnknownUnit {
                unit_string: x.to_string(),
                quantity_type: "Electric Potential".to_string(),
            }),
        }
    }
}
