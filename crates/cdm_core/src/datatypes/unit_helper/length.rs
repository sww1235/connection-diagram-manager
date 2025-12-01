use serde::{Deserialize, Serialize};
#[expect(
    clippy::wildcard_imports,
    reason = "using wildcard imports here, as there are so many units from UOM and we want all of them"
)]
use uom::si::{Unit as _, length::*, rational64};

use super::IntermediateUnit;
use crate::error::UnitParsingError;

/// Struct representing `Length` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntermediateUnit")]
#[non_exhaustive]
pub struct Length {
    /// contained uom Unit
    pub value: rational64::Length,
    /// original unit in datafile
    pub original_unit: String,
}

impl Length {
    /// outputs all usable `Length` units allowed in configuration files in the form of `<unit
    /// name>: <unit abbreviation>`
    #[must_use]
    #[expect(clippy::string_add, reason = "easier and cleaner than one massive format string")]
    #[inline]
    pub fn output_units() -> String {
        let string1 = "Unit Name";
        let string2 = "Abbreviation";
        let dash_string = "-".repeat(21);
        format!("{string1:^21}|{string2:^21}\n{dash_string}|{dash_string}\n")
            //    commented units currently unsupported due to bug with uom. iliekturtles/uom#60
            //+ format!("{:^21}|{:^21}\n", yottameter::singular(), yottameter::abbreviation()).as_str()
            //+ format!("{:^21}|{:^21}\n", zettameter::singular(), zettameter::abbreviation()).as_str()
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
            //    commented units currently unsupported due to bug with uom. iliekturtles/uom#60
            //+ format!("{:^21}|{:^21}\n", zeptometer::singular(), zeptometer::abbreviation()).as_str()
            //+ format!("{:^21}|{:^21}\n", yoctometer::singular(), yoctometer::abbreviation()).as_str()
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

//TODO: return a different error if the unit is of the wrong type rather than just unknown unit
impl TryFrom<IntermediateUnit> for Length {
    type Error = UnitParsingError;
    #[expect(
        clippy::too_many_lines,
        clippy::match_same_arms,
        reason = "match same arms due to issues with underlying datatype for now, too many lines, thats the amount of units we \
                  have"
    )]
    #[inline]
    fn try_from(item: IntermediateUnit) -> Result<Self, Self::Error> {
        match item.original_unit.as_str() {
            //"Ym" | "yottameter" => Ok(Self {
            //    value: rational64::Length::new::<yottameter>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "Ym" | "yottameter" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Length".to_owned(),
            }),
            //"Zm" | "zettameter" => Ok(Self {
            //    value: rational64::Length::new::<zettameter>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "Zm" | "zettameter" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Length".to_owned(),
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
            //"zm" | "zeptometer" => Ok(Self {
            //    value: rational64::Length::new::<zeptometer>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "zm" | "zeptometer" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Length".to_owned(),
            }),
            //"ym" | "yoctometer" => Ok(Self {
            //    value: rational64::Length::new::<yoctometer>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "ym" | "yoctometer" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Length".to_owned(),
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
                unit_string: x.to_owned(),
                quantity_type: "Length".to_owned(),
            }),
        }
    }
}
