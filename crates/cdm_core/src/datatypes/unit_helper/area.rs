use serde::{Deserialize, Serialize};
#[expect(
    clippy::wildcard_imports,
    reason = "using wildcard imports here, as there are so many units from UOM and we want all of them"
)]
use uom::si::{Unit as _, area::*, rational64};

use super::IntermediateUnit;
use crate::error::UnitParsingError;

/// Struct representing `Area` values
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "IntermediateUnit")]
#[non_exhaustive]
pub struct Area {
    /// contained uom Unit
    pub value: rational64::Area,
    /// original unit in datafile
    pub original_unit: String,
}

impl Area {
    /// outputs all usable `Area` units allowed in configuration files in the form of `<unit name>:
    /// <unit abbreviation>`
    #[must_use]
    #[expect(clippy::string_add, reason = "easier and cleaner than one massive format string")]
    #[inline]
    pub fn output_units() -> String {
        // need to do this hack AFAIK so the spacing is the same
        let string1 = "Unit Name";
        let string2 = "Abbreviation";
        let dash_string = "-".repeat(21);
        format!("{string1:^21}|{string2:^21}\n{dash_string}|{dash_string}\n")
            //    commented units currently unsupported due to bug with uom. iliekturtles/uom#60
            //+ format!(
            //    "{:^21}|{:^21}\n",
            //    square_yottameter::singular(),
            //    square_yottameter::abbreviation()
            //)
            //.as_str()
            //+ format!(
            //    "{:^21}|{:^21}\n",
            //    square_zettameter::singular(),
            //    square_zettameter::abbreviation()
            //)
            //.as_str()
            //+ format!(
            //    "{:^21}|{:^21}\n",
            //    square_exameter::singular(),
            //    square_exameter::abbreviation()
            //)
            //.as_str()
            //+ format!(
            //    "{:^21}|{:^21}\n",
            //    square_petameter::singular(),
            //    square_petameter::abbreviation()
            //)
            //.as_str()
            //+ format!(
            //    "{:^21}|{:^21}\n",
            //    square_terameter::singular(),
            //    square_terameter::abbreviation()
            //)
            //.as_str()
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
            //    commented units currently unsupported due to bug with uom. iliekturtles/uom#60
            //+ format!(
            //    "{:^21}|{:^21}\n",
            //    square_picometer::singular(),
            //    square_picometer::abbreviation()
            //)
            //.as_str()
            //+ format!(
            //    "{:^21}|{:^21}\n",
            //    square_femtometer::singular(),
            //    square_femtometer::abbreviation()
            //)
            //.as_str()
            //+ format!(
            //    "{:^21}|{:^21}\n",
            //    square_attometer::singular(),
            //    square_attometer::abbreviation()
            //)
            //.as_str()
            //+ format!(
            //    "{:^21}|{:^21}\n",
            //    square_zeptometer::singular(),
            //    square_zeptometer::abbreviation()
            //)
            //.as_str()
            //+ format!(
            //    "{:^21}|{:^21}\n",
            //    square_yoctometer::singular(),
            //    square_yoctometer::abbreviation()
            //)
            //.as_str()
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

//TODO: return a different error if the unit is of the wrong type rather than just unknown unit
impl TryFrom<IntermediateUnit> for Area {
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
            //"Ym²" | "square yottameter" => Ok(Self {
            //    value: rational64::Area::new::<square_yottameter>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "Ym²" | "square yottameter" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Area".to_owned(),
            }),
            //"Zm²" | "square zettameter" => Ok(Self {
            //    value: rational64::Area::new::<square_zettameter>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "Zm²" | "square zettameter" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Area".to_owned(),
            }),
            //"Em²" | "square exameter" => Ok(Self {
            //    value: rational64::Area::new::<square_exameter>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "Em²" | "square exameter" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Area".to_owned(),
            }),
            //"Pm²" | "square petameter" => Ok(Self {
            //    value: rational64::Area::new::<square_petameter>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "Pm²" | "square petameter" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Area".to_owned(),
            }),
            //"Tm²" | "square terameter" => Ok(Self {
            //    value: rational64::Area::new::<square_terameter>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "Tm²" | "square terameter" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Area".to_owned(),
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
            //"pm²" | "square picometer" => Ok(Self {
            //    value: rational64::Area::new::<square_picometer>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "pm²" | "square picometer" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Area".to_owned(),
            }),
            //"fm²" | "square femtometer" => Ok(Self {
            //    value: rational64::Area::new::<square_femtometer>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "fm²" | "square femtometer" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Area".to_owned(),
            }),
            //"am²" | "square attometer" => Ok(Self {
            //    value: rational64::Area::new::<square_attometer>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "am²" | "square attometer" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Area".to_owned(),
            }),
            //"zm²" | "square zeptometer" => Ok(Self {
            //    value: rational64::Area::new::<square_zeptometer>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "zm²" | "square zeptometer" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Area".to_owned(),
            }),
            //"ym²" | "square yoctometer" => Ok(Self {
            //    value: rational64::Area::new::<square_yoctometer>(item.value),
            //    original_unit: item.original_unit,
            //}),
            // Unit unsupported due to iliekturtles/uom#60
            "ym²" | "square yoctometer" => Err(UnitParsingError::UnsupportedUnit {
                unit_string: item.original_unit,
                quantity_type: "Area".to_owned(),
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
                unit_string: x.to_owned(),
                quantity_type: "Area".to_owned(),
            }),
        }
    }
}

impl Area {
    //TODO: actual error type
    //    fn check_type(self, type_: AreaUnit) -> Result<Self, String> {
    //        match (type_, self) {
    //
    //            (AreaUnit::SquareYottameter, unit) => {Ok, }
    //            (AreaUnit::SquareZettameter, unit)
    //            (AreaUnit::SquareExameter, unit)
    //            (AreaUnit::SquarePetameter, unit)
    //            (AreaUnit::SquareTerameter, unit)
    //            (AreaUnit::SquareGigameter, unit)
    //            (AreaUnit::SquareMegameter, unit)
    //            (AreaUnit::SquareKilometer, unit)
    //            (AreaUnit::SquareHectometer, unit)
    //            (AreaUnit::SquareDecameter, unit)
    //            (AreaUnit::SquareMeter, unit)
    //            (AreaUnit::SquareDecimeter, unit)
    //            (AreaUnit::SquareCentimeter, unit)
    //            (AreaUnit::SquareMillimeter, unit)
    //            (AreaUnit::SquareMicrometer, unit)
    //            (AreaUnit::SquareNanometer, unit)
    //            (AreaUnit::SquarePicometer, unit)
    //            (AreaUnit::SquareFemtometer, unit)
    //            (AreaUnit::SquareAttometer, unit)
    //            (AreaUnit::SquareZeptometer, unit)
    //            (AreaUnit::SquareYoctometer, unit)
    //            (AreaUnit::Acre, unit)
    //            (AreaUnit::Are, unit)
    //            (AreaUnit::Barn, unit)
    //            (AreaUnit::CircularMil, unit)
    //            (AreaUnit::Hectare, unit)
    //            (AreaUnit::SquareFoot, unit)
    //            (AreaUnit::SquareInch, unit)
    //            (AreaUnit::SquareMile, unit)
    //            (AreaUnit::SquareYard, unit)
    //        }
    //    }
}

// https://stackoverflow.com/a/69769813
// where this custom Serde hack came from
/// Units enum for selecting type of unit. This is a hack until UOM gets proper support for
/// selecting units when serializing and de-serializing
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[expect(missing_docs)]
#[non_exhaustive]
pub enum AreaUnit {
    SquareYottameter,
    SquareZettameter,
    SquareExameter,
    SquarePetameter,
    SquareTerameter,
    SquareGigameter,
    SquareMegameter,
    SquareKilometer,
    SquareHectometer,
    SquareDecameter,
    #[default]
    SquareMeter,
    SquareDecimeter,
    SquareCentimeter,
    SquareMillimeter,
    SquareMicrometer,
    SquareNanometer,
    SquarePicometer,
    SquareFemtometer,
    SquareAttometer,
    SquareZeptometer,
    SquareYoctometer,
    Acre,
    Are,
    Barn,
    CircularMil,
    Hectare,
    SquareFoot,
    SquareInch,
    SquareMile,
    SquareYard,
}
