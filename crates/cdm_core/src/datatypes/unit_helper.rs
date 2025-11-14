use num_rational::Rational64;
use serde::{
    Deserialize,
    Serialize,
};
use uom::si::{
    area::{
        acre,
        are,
        barn,
        circular_mil,
        hectare,
        square_attometer,
        square_centimeter,
        square_decameter,
        square_decimeter,
        square_exameter,
        square_femtometer,
        square_foot,
        square_gigameter,
        square_hectometer,
        square_inch,
        square_kilometer,
        square_megameter,
        square_meter,
        square_micrometer,
        square_mile,
        square_millimeter,
        square_nanometer,
        square_petameter,
        square_picometer,
        square_terameter,
        square_yard,
        square_yoctometer,
        square_yottameter,
        square_zeptometer,
        square_zettameter,
    },
    rational64,
};

use crate::error::UnitParsingError;

/// Struct representing `Area` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntermediateUnit")]
pub struct Area {
    /// contained uom Unit
    pub value: rational64::Area,
    /// original unit in datafile
    pub original_unit: String,
}

/// Struct representing `Cross Sectional Area` values of conductors
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntermediateUnit")]
pub struct CrossSectionalArea {
    /// contained uom Unit
    pub value: rational64::Area,
    /// original unit in datafile
    pub original_unit: String,
}

/// Struct representing `Electric Potential` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ElectricPotential {
    /// contained uom Unit
    pub value: rational64::ElectricPotential,
    /// original unit in datafile
    pub original_unit: String,
}

/// Struct representing `Length` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Length {
    /// contained uom Unit
    pub value: rational64::Length,
    /// original unit in datafile
    pub original_unit: String,
}

/// Struct representing `TemperatureInterval` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TemperatureInterval {
    /// contained uom Unit
    pub value: rational64::TemperatureInterval,
    /// original unit in datafile
    pub original_unit: String,
}

//TODO: implement to/from CrossSectionalArea to AWG
//
//TODO: implement to/from Area to CrossSectionalArea

/// Intermediate unit
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[expect(clippy::missing_docs_in_private_items)]
struct IntermediateUnit {
    value: Rational64,
    original_unit: String,
}

impl TryFrom<IntermediateUnit> for Area {
    type Error = UnitParsingError;
    #[expect(clippy::too_many_lines)]
    fn try_from(item: IntermediateUnit) -> Result<Self, Self::Error> {
        match item.original_unit.as_str() {
            "Ym²" | "square yottameter" => Ok(Self {
                value: rational64::Area::new::<square_yottameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Zm²" | "square zettameter" => Ok(Self {
                value: rational64::Area::new::<square_zettameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Em²" | "square exameter" => Ok(Self {
                value: rational64::Area::new::<square_exameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Pm²" | "square petameter" => Ok(Self {
                value: rational64::Area::new::<square_petameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Tm²" | "square terameter" => Ok(Self {
                value: rational64::Area::new::<square_terameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Gm²" | "square gigameter" => Ok(Self {
                value: rational64::Area::new::<square_gigameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Mm²" | "square megameter" => Ok(Self {
                value: rational64::Area::new::<square_megameter>(item.value),
                original_unit: item.original_unit,
            }),
            "km²" | "square kilometer" => Ok(Self {
                value: rational64::Area::new::<square_kilometer>(item.value),
                original_unit: item.original_unit,
            }),
            "hm²" | "square hectometer" => Ok(Self {
                value: rational64::Area::new::<square_hectometer>(item.value),
                original_unit: item.original_unit,
            }),
            "dam²" | "square decameter" => Ok(Self {
                value: rational64::Area::new::<square_decameter>(item.value),
                original_unit: item.original_unit,
            }),
            "m²" | "square meter" => Ok(Self {
                value: rational64::Area::new::<square_meter>(item.value),
                original_unit: item.original_unit,
            }),
            "dm²" | "square decimeter" => Ok(Self {
                value: rational64::Area::new::<square_decimeter>(item.value),
                original_unit: item.original_unit,
            }),
            "cm²" | "square centimeter" => Ok(Self {
                value: rational64::Area::new::<square_centimeter>(item.value),
                original_unit: item.original_unit,
            }),
            "mm²" | "square millimeter" => Ok(Self {
                value: rational64::Area::new::<square_millimeter>(item.value),
                original_unit: item.original_unit,
            }),
            "µm²" | "square micrometer" => Ok(Self {
                value: rational64::Area::new::<square_micrometer>(item.value),
                original_unit: item.original_unit,
            }),
            "nm²" | "square nanometer" => Ok(Self {
                value: rational64::Area::new::<square_nanometer>(item.value),
                original_unit: item.original_unit,
            }),
            "pm²" | "square picometer" => Ok(Self {
                value: rational64::Area::new::<square_picometer>(item.value),
                original_unit: item.original_unit,
            }),
            "fm²" | "square femtometer" => Ok(Self {
                value: rational64::Area::new::<square_femtometer>(item.value),
                original_unit: item.original_unit,
            }),
            "am²" | "square attometer" => Ok(Self {
                value: rational64::Area::new::<square_attometer>(item.value),
                original_unit: item.original_unit,
            }),
            "zm²" | "square zeptometer" => Ok(Self {
                value: rational64::Area::new::<square_zeptometer>(item.value),
                original_unit: item.original_unit,
            }),
            "ym²" | "square yoctometer" => Ok(Self {
                value: rational64::Area::new::<square_yoctometer>(item.value),
                original_unit: item.original_unit,
            }),
            "ac" | "acre" => Ok(Self {
                value: rational64::Area::new::<acre>(item.value),
                original_unit: item.original_unit,
            }),
            "a" | "are" => Ok(Self {
                value: rational64::Area::new::<are>(item.value),
                original_unit: item.original_unit,
            }),
            "b" | "barn" => Ok(Self {
                value: rational64::Area::new::<barn>(item.value),
                original_unit: item.original_unit,
            }),
            "cmil" | "circular mil" => Ok(Self {
                value: rational64::Area::new::<circular_mil>(item.value),
                original_unit: item.original_unit,
            }),
            "ha" | "hectare" => Ok(Self {
                value: rational64::Area::new::<hectare>(item.value),
                original_unit: item.original_unit,
            }),
            "ft²" | "square foot" => Ok(Self {
                value: rational64::Area::new::<square_foot>(item.value),
                original_unit: item.original_unit,
            }),
            "in²" | "square inch" => Ok(Self {
                value: rational64::Area::new::<square_inch>(item.value),
                original_unit: item.original_unit,
            }),
            "mi²" | "square mile" => Ok(Self {
                value: rational64::Area::new::<square_mile>(item.value),
                original_unit: item.original_unit,
            }),
            "yd²" | "square yard" => Ok(Self {
                value: rational64::Area::new::<square_yard>(item.value),
                original_unit: item.original_unit,
            }),
            x => Err(UnitParsingError::UnknownUnit(x.to_string())),
        }
    }
}
impl TryFrom<IntermediateUnit> for CrossSectionalArea {
    type Error = UnitParsingError;
    #[expect(clippy::too_many_lines)]
    fn try_from(item: IntermediateUnit) -> Result<Self, Self::Error> {
        match item.original_unit.as_str() {
            "Ym²" | "square yottameter" => Ok(Self {
                value: rational64::Area::new::<square_yottameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Zm²" | "square zettameter" => Ok(Self {
                value: rational64::Area::new::<square_zettameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Em²" | "square exameter" => Ok(Self {
                value: rational64::Area::new::<square_exameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Pm²" | "square petameter" => Ok(Self {
                value: rational64::Area::new::<square_petameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Tm²" | "square terameter" => Ok(Self {
                value: rational64::Area::new::<square_terameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Gm²" | "square gigameter" => Ok(Self {
                value: rational64::Area::new::<square_gigameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Mm²" | "square megameter" => Ok(Self {
                value: rational64::Area::new::<square_megameter>(item.value),
                original_unit: item.original_unit,
            }),
            "km²" | "square kilometer" => Ok(Self {
                value: rational64::Area::new::<square_kilometer>(item.value),
                original_unit: item.original_unit,
            }),
            "hm²" | "square hectometer" => Ok(Self {
                value: rational64::Area::new::<square_hectometer>(item.value),
                original_unit: item.original_unit,
            }),
            "dam²" | "square decameter" => Ok(Self {
                value: rational64::Area::new::<square_decameter>(item.value),
                original_unit: item.original_unit,
            }),
            "m²" | "square meter" => Ok(Self {
                value: rational64::Area::new::<square_meter>(item.value),
                original_unit: item.original_unit,
            }),
            "dm²" | "square decimeter" => Ok(Self {
                value: rational64::Area::new::<square_decimeter>(item.value),
                original_unit: item.original_unit,
            }),
            "cm²" | "square centimeter" => Ok(Self {
                value: rational64::Area::new::<square_centimeter>(item.value),
                original_unit: item.original_unit,
            }),
            "mm²" | "square millimeter" => Ok(Self {
                value: rational64::Area::new::<square_millimeter>(item.value),
                original_unit: item.original_unit,
            }),
            "µm²" | "square micrometer" => Ok(Self {
                value: rational64::Area::new::<square_micrometer>(item.value),
                original_unit: item.original_unit,
            }),
            "nm²" | "square nanometer" => Ok(Self {
                value: rational64::Area::new::<square_nanometer>(item.value),
                original_unit: item.original_unit,
            }),
            "pm²" | "square picometer" => Ok(Self {
                value: rational64::Area::new::<square_picometer>(item.value),
                original_unit: item.original_unit,
            }),
            "fm²" | "square femtometer" => Ok(Self {
                value: rational64::Area::new::<square_femtometer>(item.value),
                original_unit: item.original_unit,
            }),
            "am²" | "square attometer" => Ok(Self {
                value: rational64::Area::new::<square_attometer>(item.value),
                original_unit: item.original_unit,
            }),
            "zm²" | "square zeptometer" => Ok(Self {
                value: rational64::Area::new::<square_zeptometer>(item.value),
                original_unit: item.original_unit,
            }),
            "ym²" | "square yoctometer" => Ok(Self {
                value: rational64::Area::new::<square_yoctometer>(item.value),
                original_unit: item.original_unit,
            }),
            "ac" | "acre" => Ok(Self {
                value: rational64::Area::new::<acre>(item.value),
                original_unit: item.original_unit,
            }),
            "a" | "are" => Ok(Self {
                value: rational64::Area::new::<are>(item.value),
                original_unit: item.original_unit,
            }),
            "b" | "barn" => Ok(Self {
                value: rational64::Area::new::<barn>(item.value),
                original_unit: item.original_unit,
            }),
            "cmil" | "circular mil" => Ok(Self {
                value: rational64::Area::new::<circular_mil>(item.value),
                original_unit: item.original_unit,
            }),
            "ha" | "hectare" => Ok(Self {
                value: rational64::Area::new::<hectare>(item.value),
                original_unit: item.original_unit,
            }),
            "ft²" | "square foot" => Ok(Self {
                value: rational64::Area::new::<square_foot>(item.value),
                original_unit: item.original_unit,
            }),
            "in²" | "square inch" => Ok(Self {
                value: rational64::Area::new::<square_inch>(item.value),
                original_unit: item.original_unit,
            }),
            "mi²" | "square mile" => Ok(Self {
                value: rational64::Area::new::<square_mile>(item.value),
                original_unit: item.original_unit,
            }),
            "yd²" | "square yard" => Ok(Self {
                value: rational64::Area::new::<square_yard>(item.value),
                original_unit: item.original_unit,
            }),
            x => Err(UnitParsingError::UnknownUnit(x.to_string())),
        }
    }
}
