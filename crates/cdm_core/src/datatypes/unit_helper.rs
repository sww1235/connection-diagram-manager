use num_rational::Rational64;
use serde::{Deserialize, Serialize};
#[expect(clippy::wildcard_imports)]
use uom::si::{Unit, area::*, electric_potential::*, length::*, rational64, temperature_interval::*};

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
#[serde(try_from = "IntermediateUnit")]
pub struct ElectricPotential {
    /// contained uom Unit
    pub value: rational64::ElectricPotential,
    /// original unit in datafile
    pub original_unit: String,
}

/// Struct representing `Length` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntermediateUnit")]
pub struct Length {
    /// contained uom Unit
    pub value: rational64::Length,
    /// original unit in datafile
    pub original_unit: String,
}

/// Struct representing `TemperatureInterval` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntermediateUnit")]
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
struct IntermediateUnit {
    /// raw value read in from file
    value: Rational64,
    /// original unit string in datafile
    original_unit: String,
}
impl Area {
    /// outputs all usable `Area` units allowed in configuration files in the form of `<unit name>:
    /// <unit abbreviation>`
    #[must_use]
    #[expect(clippy::string_add, clippy::too_many_lines)]
    pub fn output_units() -> String {
        // need to do this hack AFAIK so the spacing is the same
        let string1 = "Unit Name";
        let string2 = "Abbreviation";
        let dash_string = "-".repeat(21);
        format!("{string1:^21}|{string2:^21}\n{dash_string}|{dash_string}\n")
            + format!(
                "{:^21}|{:^21}\n",
                square_yottameter::singular(),
                square_yottameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_zettameter::singular(),
                square_zettameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_exameter::singular(),
                square_exameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_petameter::singular(),
                square_petameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_terameter::singular(),
                square_terameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_gigameter::singular(),
                square_gigameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_megameter::singular(),
                square_megameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_kilometer::singular(),
                square_kilometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_hectometer::singular(),
                square_hectometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_decameter::singular(),
                square_decameter::abbreviation()
            )
            .as_str()
            + format!("{:^21}|{:^21}\n", square_meter::singular(), square_meter::abbreviation()).as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_decimeter::singular(),
                square_decimeter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_centimeter::singular(),
                square_centimeter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_millimeter::singular(),
                square_millimeter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_micrometer::singular(),
                square_micrometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_nanometer::singular(),
                square_nanometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_picometer::singular(),
                square_picometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_femtometer::singular(),
                square_femtometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_attometer::singular(),
                square_attometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_zeptometer::singular(),
                square_zeptometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_yoctometer::singular(),
                square_yoctometer::abbreviation()
            )
            .as_str()
            + format!("{:^21}|{:^21}\n", acre::singular(), acre::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", are::singular(), are::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", barn::singular(), barn::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", circular_mil::singular(), circular_mil::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", hectare::singular(), hectare::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", square_foot::singular(), square_foot::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", square_inch::singular(), square_inch::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", square_mile::singular(), square_mile::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", square_yard::singular(), square_yard::abbreviation()).as_str()
    }
}

impl CrossSectionalArea {
    /// outputs all usable `CrossSectionalArea` units allowed in configuration files in the form of
    /// `<unit name>: <unit abbreviation>`
    #[must_use]
    #[expect(clippy::string_add, clippy::too_many_lines)]
    pub fn output_units() -> String {
        let string1 = "Unit Name";
        let string2 = "Abbreviation";
        let dash_string = "-".repeat(21);
        format!("{string1:^21}|{string2:^21}\n{dash_string}|{dash_string}\n")
            + format!(
                "{:^21}|{:^21}\n",
                square_yottameter::singular(),
                square_yottameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_zettameter::singular(),
                square_zettameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_exameter::singular(),
                square_exameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_petameter::singular(),
                square_petameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_terameter::singular(),
                square_terameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_gigameter::singular(),
                square_gigameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_megameter::singular(),
                square_megameter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_kilometer::singular(),
                square_kilometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_hectometer::singular(),
                square_hectometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_decameter::singular(),
                square_decameter::abbreviation()
            )
            .as_str()
            + format!("{:^21}|{:^21}\n", square_meter::singular(), square_meter::abbreviation()).as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_decimeter::singular(),
                square_decimeter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_centimeter::singular(),
                square_centimeter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_millimeter::singular(),
                square_millimeter::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_micrometer::singular(),
                square_micrometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_nanometer::singular(),
                square_nanometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_picometer::singular(),
                square_picometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_femtometer::singular(),
                square_femtometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_attometer::singular(),
                square_attometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_zeptometer::singular(),
                square_zeptometer::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                square_yoctometer::singular(),
                square_yoctometer::abbreviation()
            )
            .as_str()
            + format!("{:^21}|{:^21}\n", acre::singular(), acre::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", are::singular(), are::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", barn::singular(), barn::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", circular_mil::singular(), circular_mil::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", hectare::singular(), hectare::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", square_foot::singular(), square_foot::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", square_inch::singular(), square_inch::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", square_mile::singular(), square_mile::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", square_yard::singular(), square_yard::abbreviation()).as_str()
    }
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
            + format!("{:^21}|{:^21}\n", yottavolt::singular(), yottavolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", zettavolt::singular(), zettavolt::abbreviation()).as_str()
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
            + format!("{:^21}|{:^21}\n", zeptovolt::singular(), zeptovolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", yoctovolt::singular(), yoctovolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", abvolt::singular(), abvolt::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", statvolt::singular(), statvolt::abbreviation()).as_str()
    }
}

impl Length {
    /// outputs all usable `Length` units allowed in configuration files in the form of `<unit
    /// name>: <unit abbreviation>`
    #[must_use]
    #[expect(clippy::string_add)]
    pub fn output_units() -> String {
        let string1 = "Unit Name";
        let string2 = "Abbreviation";
        let dash_string = "-".repeat(21);
        format!("{string1:^21}|{string2:^21}\n{dash_string}|{dash_string}\n")
            + format!("{:^21}|{:^21}\n", yottameter::singular(), yottameter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", zettameter::singular(), zettameter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", exameter::singular(), exameter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", petameter::singular(), petameter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", terameter::singular(), terameter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", gigameter::singular(), gigameter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", megameter::singular(), megameter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", kilometer::singular(), kilometer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", hectometer::singular(), hectometer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", decameter::singular(), decameter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", meter::singular(), meter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", decimeter::singular(), decimeter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", centimeter::singular(), centimeter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", millimeter::singular(), millimeter::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", micrometer::singular(), micrometer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", nanometer::singular(), nanometer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", picometer::singular(), picometer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", femtometer::singular(), femtometer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", attometer::singular(), attometer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", zeptometer::singular(), zeptometer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", yoctometer::singular(), yoctometer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", angstrom::singular(), angstrom::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", bohr_radius::singular(), bohr_radius::abbreviation()).as_str()
            + format!(
                "{:^21}|{:^21}\n",
                atomic_unit_of_length::singular(),
                atomic_unit_of_length::abbreviation()
            )
            .as_str()
            + format!(
                "{:^21}|{:^21}\n",
                astronomical_unit::singular(),
                astronomical_unit::abbreviation()
            )
            .as_str()
            + format!("{:^21}|{:^21}\n", chain::singular(), chain::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", fathom::singular(), fathom::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", foot::singular(), foot::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", foot_survey::singular(), foot_survey::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", inch::singular(), inch::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", light_year::singular(), light_year::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", microinch::singular(), microinch::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", micron::singular(), micron::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", mil::singular(), mil::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", mile::singular(), mile::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", mile_survey::singular(), mile_survey::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", nautical_mile::singular(), nautical_mile::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", parsec::singular(), parsec::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", pica_computer::singular(), pica_computer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", pica_printers::singular(), pica_printers::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", point_computer::singular(), point_computer::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", point_printers::singular(), point_printers::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", rod::singular(), rod::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", yard::singular(), yard::abbreviation()).as_str()
    }
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
            + format!("{:^21}|{:^21}\n", yottakelvin::singular(), yottakelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", zettakelvin::singular(), zettakelvin::abbreviation()).as_str()
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
            + format!("{:^21}|{:^21}\n", zeptokelvin::singular(), zeptokelvin::abbreviation()).as_str()
            + format!("{:^21}|{:^21}\n", yoctokelvin::singular(), yoctokelvin::abbreviation()).as_str()
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
            x => Err(UnitParsingError::UnknownUnit {
                unit_string: x.to_string(),
                unit_type: "Area".to_string(),
            }),
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
            x => Err(UnitParsingError::UnknownUnit {
                unit_string: x.to_string(),
                unit_type: "Cross Sectional Area".to_string(),
            }),
        }
    }
}

impl TryFrom<IntermediateUnit> for ElectricPotential {
    type Error = UnitParsingError;
    fn try_from(item: IntermediateUnit) -> Result<Self, Self::Error> {
        match item.original_unit.as_str() {
            "YV" | "yottavolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<yottavolt>(item.value),
                original_unit: item.original_unit,
            }),
            "ZV" | "zettavolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<zettavolt>(item.value),
                original_unit: item.original_unit,
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
            "zV" | "zeptovolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<zeptovolt>(item.value),
                original_unit: item.original_unit,
            }),
            "yV" | "yoctovolt" => Ok(Self {
                value: rational64::ElectricPotential::new::<yoctovolt>(item.value),
                original_unit: item.original_unit,
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
                unit_type: "Electric Potential".to_string(),
            }),
        }
    }
}

impl TryFrom<IntermediateUnit> for Length {
    type Error = UnitParsingError;
    #[expect(clippy::too_many_lines)]
    fn try_from(item: IntermediateUnit) -> Result<Self, Self::Error> {
        match item.original_unit.as_str() {
            "Ym" | "yottameter" => Ok(Self {
                value: rational64::Length::new::<yottameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Zm" | "zettameter" => Ok(Self {
                value: rational64::Length::new::<zettameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Em" | "exameter" => Ok(Self {
                value: rational64::Length::new::<exameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Pm" | "petameter" => Ok(Self {
                value: rational64::Length::new::<petameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Tm" | "terameter" => Ok(Self {
                value: rational64::Length::new::<terameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Gm" | "gigameter" => Ok(Self {
                value: rational64::Length::new::<gigameter>(item.value),
                original_unit: item.original_unit,
            }),
            "Mm" | "megameter" => Ok(Self {
                value: rational64::Length::new::<megameter>(item.value),
                original_unit: item.original_unit,
            }),
            "km" | "kilometer" => Ok(Self {
                value: rational64::Length::new::<kilometer>(item.value),
                original_unit: item.original_unit,
            }),
            "hm" | "hectometer" => Ok(Self {
                value: rational64::Length::new::<hectometer>(item.value),
                original_unit: item.original_unit,
            }),
            "dam" | "decameter" => Ok(Self {
                value: rational64::Length::new::<decameter>(item.value),
                original_unit: item.original_unit,
            }),
            "m" | "meter" => Ok(Self {
                value: rational64::Length::new::<meter>(item.value),
                original_unit: item.original_unit,
            }),
            "dm" | "decimeter" => Ok(Self {
                value: rational64::Length::new::<decimeter>(item.value),
                original_unit: item.original_unit,
            }),
            "cm" | "centimeter" => Ok(Self {
                value: rational64::Length::new::<centimeter>(item.value),
                original_unit: item.original_unit,
            }),
            "mm" | "millimeter" => Ok(Self {
                value: rational64::Length::new::<millimeter>(item.value),
                original_unit: item.original_unit,
            }),
            "µm" | "micrometer" => Ok(Self {
                value: rational64::Length::new::<micrometer>(item.value),
                original_unit: item.original_unit,
            }),
            "nm" | "nanometer" => Ok(Self {
                value: rational64::Length::new::<nanometer>(item.value),
                original_unit: item.original_unit,
            }),
            "pm" | "picometer" => Ok(Self {
                value: rational64::Length::new::<picometer>(item.value),
                original_unit: item.original_unit,
            }),
            "fm" | "femtometer" => Ok(Self {
                value: rational64::Length::new::<femtometer>(item.value),
                original_unit: item.original_unit,
            }),
            "am" | "attometer" => Ok(Self {
                value: rational64::Length::new::<attometer>(item.value),
                original_unit: item.original_unit,
            }),
            "zm" | "zeptometer" => Ok(Self {
                value: rational64::Length::new::<zeptometer>(item.value),
                original_unit: item.original_unit,
            }),
            "ym" | "yoctometer" => Ok(Self {
                value: rational64::Length::new::<yoctometer>(item.value),
                original_unit: item.original_unit,
            }),

            "Å" | "ångström" => Ok(Self {
                value: rational64::Length::new::<angstrom>(item.value),
                original_unit: item.original_unit,
            }),
            "a₀" | "bohr radius" => Ok(Self {
                value: rational64::Length::new::<bohr_radius>(item.value),
                original_unit: item.original_unit,
            }),
            "a.u. of length" | "atomic unit of length" => Ok(Self {
                value: rational64::Length::new::<atomic_unit_of_length>(item.value),
                original_unit: item.original_unit,
            }),
            "ua" | "astronomical unit" => Ok(Self {
                value: rational64::Length::new::<astronomical_unit>(item.value),
                original_unit: item.original_unit,
            }),
            "ch" | "chain" => Ok(Self {
                value: rational64::Length::new::<chain>(item.value),
                original_unit: item.original_unit,
            }),
            // abbreviation and singular unit name are the same currently
            "fathom" => Ok(Self {
                value: rational64::Length::new::<fathom>(item.value),
                original_unit: item.original_unit,
            }),
            // abbreviation and singular unit name are the same currently
            "fermi" => Ok(Self {
                value: rational64::Length::new::<fermi>(item.value),
                original_unit: item.original_unit,
            }),
            "ft" | "foot" => Ok(Self {
                value: rational64::Length::new::<foot>(item.value),
                original_unit: item.original_unit,
            }),
            "ft (U.S. survey)" | "foot (U.S. survey)" => Ok(Self {
                value: rational64::Length::new::<foot_survey>(item.value),
                original_unit: item.original_unit,
            }),
            "in" | "inch" => Ok(Self {
                value: rational64::Length::new::<inch>(item.value),
                original_unit: item.original_unit,
            }),
            "l. y." | "light year" => Ok(Self {
                value: rational64::Length::new::<light_year>(item.value),
                original_unit: item.original_unit,
            }),
            "μin" | "microinch" => Ok(Self {
                value: rational64::Length::new::<microinch>(item.value),
                original_unit: item.original_unit,
            }),
            "μ" | "micron" => Ok(Self {
                value: rational64::Length::new::<micron>(item.value),
                original_unit: item.original_unit,
            }),
            "0.001 in" | "mil" => Ok(Self {
                value: rational64::Length::new::<mil>(item.value),
                original_unit: item.original_unit,
            }),
            "mi" | "mile" => Ok(Self {
                value: rational64::Length::new::<mile>(item.value),
                original_unit: item.original_unit,
            }),
            "mi (U.S. survey)" | "mile (U.S. survey)" => Ok(Self {
                value: rational64::Length::new::<mile_survey>(item.value),
                original_unit: item.original_unit,
            }),
            "M" | "nautical mile" => Ok(Self {
                value: rational64::Length::new::<nautical_mile>(item.value),
                original_unit: item.original_unit,
            }),
            "pc" | "parsec" => Ok(Self {
                value: rational64::Length::new::<parsec>(item.value),
                original_unit: item.original_unit,
            }),
            "1/6 in (computer)" | "pica (computer)" => Ok(Self {
                value: rational64::Length::new::<pica_computer>(item.value),
                original_unit: item.original_unit,
            }),
            "1/6 in" | "pica (printer's)" => Ok(Self {
                value: rational64::Length::new::<pica_printers>(item.value),
                original_unit: item.original_unit,
            }),
            "1/72 in (computer)" | "point (computer)" => Ok(Self {
                value: rational64::Length::new::<point_computer>(item.value),
                original_unit: item.original_unit,
            }),
            "1/72 in" | "point (printer's)" => Ok(Self {
                value: rational64::Length::new::<point_printers>(item.value),
                original_unit: item.original_unit,
            }),
            "rd" | "rod" => Ok(Self {
                value: rational64::Length::new::<rod>(item.value),
                original_unit: item.original_unit,
            }),
            "yd" | "yard" => Ok(Self {
                value: rational64::Length::new::<yard>(item.value),
                original_unit: item.original_unit,
            }),

            x => Err(UnitParsingError::UnknownUnit {
                unit_string: x.to_string(),
                unit_type: "Length".to_string(),
            }),
        }
    }
}

impl TryFrom<IntermediateUnit> for TemperatureInterval {
    type Error = UnitParsingError;
    fn try_from(item: IntermediateUnit) -> Result<Self, Self::Error> {
        match item.original_unit.as_str() {
            "YK" | "yottakelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<yottakelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "ZK" | "zettakelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<zettakelvin>(item.value),
                original_unit: item.original_unit,
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
            "zK" | "zeptokelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<zeptokelvin>(item.value),
                original_unit: item.original_unit,
            }),
            "yK" | "yoctokelvin" => Ok(Self {
                value: rational64::TemperatureInterval::new::<yoctokelvin>(item.value),
                original_unit: item.original_unit,
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
                unit_type: "Temperature Interval".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::{Area, CrossSectionalArea, ElectricPotential, Length, TemperatureInterval};

    // using escape codes to work around issues with VIM and rustfmt. Also to provide a more robust
    // representation of the expected output
    #[test]
    fn test_area_unit_output() {
        let test_string = "\
\x20\x20\x20\x20\x20\x20Unit Name      |    Abbreviation\x20\x20\x20\x20\x20
---------------------|---------------------
\x20\x20square yottameter  |         Ym²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square zettameter  |         Zm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20 square exameter   |         Em²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square petameter   |         Pm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square terameter   |         Tm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square gigameter   |         Gm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square megameter   |         Mm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square kilometer   |         km²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square hectometer  |         hm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square decameter   |        dam²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20  square meter     |         m²\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square decimeter   |         dm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square centimeter  |         cm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square millimeter  |         mm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square micrometer  |         µm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square nanometer   |         nm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square picometer   |         pm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square femtometer  |         fm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square attometer   |         am²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square zeptometer  |         zm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square yoctometer  |         ym²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20      acre         |         ac\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20       are         |          a\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20      barn         |          b\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20  circular mil     |        cmil\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20     hectare       |         ha\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   square foot     |         ft²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   square inch     |         in²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   square mile     |         mi²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   square yard     |         yd²\x20\x20\x20\x20\x20\x20\x20\x20\x20
";
        assert_eq!(test_string, Area::output_units());
    }

    // using escape codes to work around issues with VIM and rustfmt. Also to provide a more robust
    // representation of the expected output
    #[test]
    fn test_cross_sectional_area_unit_output() {
        let test_string = "\
\x20\x20\x20\x20\x20\x20Unit Name      |    Abbreviation\x20\x20\x20\x20\x20
---------------------|---------------------
\x20\x20square yottameter  |         Ym²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square zettameter  |         Zm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20square exameter   |         Em²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square petameter   |         Pm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square terameter   |         Tm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square gigameter   |         Gm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square megameter   |         Mm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square kilometer   |         km²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square hectometer  |         hm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square decameter   |        dam²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20  square meter     |         m²\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square decimeter   |         dm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square centimeter  |         cm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square millimeter  |         mm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square micrometer  |         µm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square nanometer   |         nm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square picometer   |         pm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square femtometer  |         fm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square attometer   |         am²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square zeptometer  |         zm²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20square yoctometer  |         ym²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20      acre         |         ac\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20       are         |          a\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20      barn         |          b\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20  circular mil     |        cmil\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20     hectare       |         ha\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   square foot     |         ft²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   square inch     |         in²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   square mile     |         mi²\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   square yard     |         yd²\x20\x20\x20\x20\x20\x20\x20\x20\x20
";
        assert_eq!(test_string, CrossSectionalArea::output_units());
    }
    // using escape codes to work around issues with VIM and rustfmt. Also to provide a more robust
    // representation of the expected output
    #[test]
    fn test_electric_potential_unit_output() {
        let test_string = "\
\x20\x20\x20\x20\x20\x20Unit Name      |    Abbreviation\x20\x20\x20\x20\x20
---------------------|---------------------
\x20\x20\x20\x20\x20\x20yottavolt      |         YV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20zettavolt      |         ZV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20 exavolt       |         EV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20petavolt       |         PV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20teravolt       |         TV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20gigavolt       |         GV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20megavolt       |         MV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20kilovolt       |         kV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20hectovolt      |         hV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20decavolt       |         daV\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20  volt         |          V\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20decivolt       |         dV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20centivolt      |         cV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20millivolt      |         mV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20microvolt      |         µV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20nanovolt       |         nV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20picovolt       |         pV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20femtovolt      |         fV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20attovolt       |         aV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20zeptovolt      |         zV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20yoctovolt      |         yV\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20 abvolt        |         abV\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20\x20statvolt       |        statV\x20\x20\x20\x20\x20\x20\x20\x20
";
        assert_eq!(test_string, ElectricPotential::output_units());
    }

    // using escape codes to work around issues with VIM and rustfmt. Also to provide a more robust
    // representation of the expected output
    #[test]
    fn test_length_unit_output() {
        let test_string = "\
\x20\x20\x20\x20\x20\x20Unit Name      |    Abbreviation\x20\x20\x20\x20\x20
---------------------|---------------------
\x20\x20\x20\x20\x20yottameter      |         Ym\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20zettameter      |         Zm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 exameter       |         Em\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 petameter      |         Pm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 terameter      |         Tm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 gigameter      |         Gm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 megameter      |         Mm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 kilometer      |         km\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20hectometer      |         hm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 decameter      |         dam\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20   meter        |          m\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 decimeter      |         dm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20centimeter      |         cm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20millimeter      |         mm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20micrometer      |         µm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 nanometer      |         nm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 picometer      |         pm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20femtometer      |         fm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 attometer      |         am\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20zeptometer      |         zm\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20yoctometer      |         ym\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20 ångström       |          Å\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20\x20\x20\x20bohr radius     |         a₀\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
atomic unit of length|   a.u. of length\x20\x20\x20\x20
\x20 astronomical unit  |         ua\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20       chain        |         ch\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20      fathom        |       fathom\x20\x20\x20\x20\x20\x20\x20\x20
\x20       foot         |         ft\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20foot (U.S. survey)  |  ft (U.S. survey)\x20\x20\x20
\x20       inch         |         in\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20    light year      |        l. y.\x20\x20\x20\x20\x20\x20\x20\x20
\x20     microinch      |         μin\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20      micron        |          μ\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20        mil         |      0.001 in\x20\x20\x20\x20\x20\x20\x20
\x20       mile         |         mi\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20mile (U.S. survey)  |  mi (U.S. survey)\x20\x20\x20
\x20   nautical mile    |          M\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20      parsec        |         pc\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20  pica (computer)   |  1/6 in (computer)\x20\x20
\x20 pica (printer's)   |       1/6 in\x20\x20\x20\x20\x20\x20\x20\x20
\x20 point (computer)   | 1/72 in (computer)\x20\x20
\x20 point (printer's)  |       1/72 in\x20\x20\x20\x20\x20\x20\x20
\x20        rod         |         rd\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20       yard         |         yd\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
";
        assert_eq!(test_string, Length::output_units());
    }
    // using escape codes to work around issues with VIM and rustfmt. Also to provide a more robust
    // representation of the expected output
    #[test]
    fn test_temperature_interval_unit_output() {
        let test_string = "\
\x20\x20\x20\x20\x20\x20Unit Name      |    Abbreviation\x20\x20\x20\x20\x20
---------------------|---------------------
\x20\x20   yottakelvin     |         YK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   zettakelvin     |         ZK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20    exakelvin      |         EK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   petakelvin      |         PK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   terakelvin      |         TK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   gigakelvin      |         GK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   megakelvin      |         MK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   kilokelvin      |         kK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   hectokelvin     |         hK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   decakelvin      |         daK\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20     kelvin        |          K\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   decikelvin      |         dK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   centikelvin     |         cK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   millikelvin     |         mK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   microkelvin     |         µK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   nanokelvin      |         nK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   picokelvin      |         pK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   femtokelvin     |         fK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   attokelvin      |         aK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   zeptokelvin     |         zK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20   yoctokelvin     |         yK\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20 degree Celsius    |         °C\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20degree Fahrenheit  |         °F\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20 degree Rankine    |         °R\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
";
        assert_eq!(test_string, TemperatureInterval::output_units());
    }
}
