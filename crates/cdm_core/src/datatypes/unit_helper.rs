use num_rational::Rational64;
use serde::{Deserialize, Serialize};

/// Helper types and functions for `Area` units.
pub mod area;
/// Helper types and functions for `Cross Sectional Area` units.
pub mod cross_sectional_area;
/// Helper types and functions for `Electric Potential` units.
pub mod electric_potential;
/// Helper types and functions for `Length` units.
pub mod length;
/// Helper types and functions for `Nominal Wire Size` units.
pub mod nominal_wire_size;
/// Helper types and functions for `Temperature Interval` units.
pub mod temperature_interval;

//TODO: implement to/from CrossSectionalArea to AWG
//
//TODO: implement to/from Area to CrossSectionalArea

/// Intermediate unit.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
struct IntermediateUnit {
    /// raw value read in from file.
    value: Rational64,
    /// original unit string in datafile.
    original_unit: String,
}

#[cfg(test)]
#[allow(clippy::too_many_lines)]
mod tests {
    use num_rational::Rational64;
    use pretty_assertions::assert_eq;
    use uom::si::{area::*, electric_potential::*, length::*, rational64, temperature_interval::*};

    use super::{
        IntermediateUnit,
        area::Area,
        cross_sectional_area::CrossSectionalArea,
        electric_potential::ElectricPotential,
        length::Length,
        temperature_interval::TemperatureInterval,
    };

    // using escape codes to work around issues with VIM and rustfmt. Also to provide a more robust
    // representation of the expected output
    //
    // TODO: When iliekturtles/uom#60 is fixed, restore deleted parts of strings from git
    #[test]
    fn test_area_unit_output() {
        let test_string = "\
\x20\x20\x20\x20\x20\x20Unit Name      |    Abbreviation\x20\x20\x20\x20\x20
---------------------|---------------------
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
\x20\x20 degree Celsius    |         °C\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20degree Fahrenheit  |         °F\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
\x20\x20 degree Rankine    |         °R\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20
";
        assert_eq!(test_string, TemperatureInterval::output_units());
    }

    #[test]
    fn test_area_try_from() {
        //01_000_000_000_000_000_000
        //18_446_744_073_709_551_615
        let test_units = vec![
            //IntermediateUnit {
            //    value: Rational64::new(5, 2),
            //    original_unit: "Ym²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 2),
            //    original_unit: "square yottameter".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "Zm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square zettameter".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "Em²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square exameter".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "Pm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square petameter".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "Tm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square terameter".to_string(),
            //},
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Gm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square gigameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Mm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square megameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "km²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square kilometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "hm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square hectometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "dam²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square decameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "m²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square meter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "dm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square decimeter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "cm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square centimeter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square millimeter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "µm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square micrometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "nm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square nanometer".to_string(),
            },
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "pm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square picometer".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "fm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square femtometer".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "am²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square attometer".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square zeptometer".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "ym²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square yoctometer".to_string(),
            //},
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ac".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "acre".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "a".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "are".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "b".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "barn".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "cmil".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "circular mil".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ha".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "hectare".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ft²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square foot".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "in²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square inch".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mi²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square mile".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yd²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square yard".to_string(),
            },
        ];

        let comparision_units = vec![
            //Area {
            //    value: rational64::Area::new::<square_yottameter>(Rational64::new(5, 2)),
            //    original_unit: "Ym²".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_yottameter>(Rational64::new(5, 2)),
            //    original_unit: "square yottameter".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_zettameter>(Rational64::new(5, 1)),
            //    original_unit: "Zm²".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_zettameter>(Rational64::new(5, 1)),
            //    original_unit: "square zettameter".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_exameter>(Rational64::new(5, 1)),
            //    original_unit: "Em²".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_exameter>(Rational64::new(5, 1)),
            //    original_unit: "square exameter".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_petameter>(Rational64::new(5, 1)),
            //    original_unit: "Pm²".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_petameter>(Rational64::new(5, 1)),
            //    original_unit: "square petameter".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_terameter>(Rational64::new(5, 1)),
            //    original_unit: "Tm²".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_terameter>(Rational64::new(5, 1)),
            //    original_unit: "square terameter".to_string(),
            //},
            Area {
                value: rational64::Area::new::<square_gigameter>(Rational64::new(5, 1)),
                original_unit: "Gm²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_gigameter>(Rational64::new(5, 1)),
                original_unit: "square gigameter".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_megameter>(Rational64::new(5, 1)),
                original_unit: "Mm²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_megameter>(Rational64::new(5, 1)),
                original_unit: "square megameter".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_kilometer>(Rational64::new(5, 1)),
                original_unit: "km²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_kilometer>(Rational64::new(5, 1)),
                original_unit: "square kilometer".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_hectometer>(Rational64::new(5, 1)),
                original_unit: "hm²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_hectometer>(Rational64::new(5, 1)),
                original_unit: "square hectometer".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_decameter>(Rational64::new(5, 1)),
                original_unit: "dam²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_decameter>(Rational64::new(5, 1)),
                original_unit: "square decameter".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_meter>(Rational64::new(5, 1)),
                original_unit: "m²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_meter>(Rational64::new(5, 1)),
                original_unit: "square meter".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_decimeter>(Rational64::new(5, 1)),
                original_unit: "dm²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_decimeter>(Rational64::new(5, 1)),
                original_unit: "square decimeter".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_centimeter>(Rational64::new(5, 1)),
                original_unit: "cm²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_centimeter>(Rational64::new(5, 1)),
                original_unit: "square centimeter".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_millimeter>(Rational64::new(5, 1)),
                original_unit: "mm²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_millimeter>(Rational64::new(5, 1)),
                original_unit: "square millimeter".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_micrometer>(Rational64::new(5, 1)),
                original_unit: "µm²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_micrometer>(Rational64::new(5, 1)),
                original_unit: "square micrometer".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_nanometer>(Rational64::new(5, 1)),
                original_unit: "nm²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_nanometer>(Rational64::new(5, 1)),
                original_unit: "square nanometer".to_string(),
            },
            //Area {
            //    value: rational64::Area::new::<square_picometer>(Rational64::new(5, 1)),
            //    original_unit: "pm²".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_picometer>(Rational64::new(5, 1)),
            //    original_unit: "square picometer".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_femtometer>(Rational64::new(5, 1)),
            //    original_unit: "fm²".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_femtometer>(Rational64::new(5, 1)),
            //    original_unit: "square femtometer".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_attometer>(Rational64::new(5, 1)),
            //    original_unit: "am²".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_attometer>(Rational64::new(5, 1)),
            //    original_unit: "square attometer".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_zeptometer>(Rational64::new(5, 1)),
            //    original_unit: "zm²".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_zeptometer>(Rational64::new(5, 1)),
            //    original_unit: "square zeptometer".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_yoctometer>(Rational64::new(5, 1)),
            //    original_unit: "ym²".to_string(),
            //},
            //Area {
            //    value: rational64::Area::new::<square_yoctometer>(Rational64::new(5, 1)),
            //    original_unit: "square yoctometer".to_string(),
            //},
            Area {
                value: rational64::Area::new::<acre>(Rational64::new(5, 1)),
                original_unit: "ac".to_string(),
            },
            Area {
                value: rational64::Area::new::<acre>(Rational64::new(5, 1)),
                original_unit: "acre".to_string(),
            },
            Area {
                value: rational64::Area::new::<are>(Rational64::new(5, 1)),
                original_unit: "a".to_string(),
            },
            Area {
                value: rational64::Area::new::<are>(Rational64::new(5, 1)),
                original_unit: "are".to_string(),
            },
            Area {
                value: rational64::Area::new::<barn>(Rational64::new(5, 1)),
                original_unit: "b".to_string(),
            },
            Area {
                value: rational64::Area::new::<barn>(Rational64::new(5, 1)),
                original_unit: "barn".to_string(),
            },
            Area {
                value: rational64::Area::new::<circular_mil>(Rational64::new(5, 1)),
                original_unit: "cmil".to_string(),
            },
            Area {
                value: rational64::Area::new::<circular_mil>(Rational64::new(5, 1)),
                original_unit: "circular mil".to_string(),
            },
            Area {
                value: rational64::Area::new::<hectare>(Rational64::new(5, 1)),
                original_unit: "ha".to_string(),
            },
            Area {
                value: rational64::Area::new::<hectare>(Rational64::new(5, 1)),
                original_unit: "hectare".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_foot>(Rational64::new(5, 1)),
                original_unit: "ft²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_foot>(Rational64::new(5, 1)),
                original_unit: "square foot".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_inch>(Rational64::new(5, 1)),
                original_unit: "in²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_inch>(Rational64::new(5, 1)),
                original_unit: "square inch".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_mile>(Rational64::new(5, 1)),
                original_unit: "mi²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_mile>(Rational64::new(5, 1)),
                original_unit: "square mile".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_yard>(Rational64::new(5, 1)),
                original_unit: "yd²".to_string(),
            },
            Area {
                value: rational64::Area::new::<square_yard>(Rational64::new(5, 1)),
                original_unit: "square yard".to_string(),
            },
        ];
        for (test, comparision) in test_units.into_iter().zip(comparision_units.into_iter()) {
            match Area::try_from(test) {
                Ok(x) => assert_eq!(x, comparision),
                Err(x) => panic!("{x}"),
            }
        }
    }

    #[test]
    #[should_panic(expected = "Unknown unit string")]
    fn test_area_try_from_invalid_unit() {
        // assuming a user had a typo in their library file and missed the ²
        let test_value = IntermediateUnit {
            value: Rational64::new(5, 1),
            original_unit: "mm".to_string(),
        };
        let comparision_value = Area {
            original_unit: "mm²".to_string(),
            value: rational64::Area::new::<square_millimeter>(Rational64::new(5, 1)),
        };

        match Area::try_from(test_value) {
            Ok(x) => assert_eq!(x, comparision_value),
            Err(x) => panic!("{x}"),
        }
    }
    #[test]
    #[should_panic(expected = "Unsupported unit string")]
    fn test_area_unsupported_units() {
        let test_units = vec![
            IntermediateUnit {
                value: Rational64::new(5, 2),
                original_unit: "Ym²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 2),
                original_unit: "square yottameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Zm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square zettameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Em²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square exameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Pm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square petameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Tm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square terameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "pm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square picometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "fm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square femtometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "am²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square attometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square zeptometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ym²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square yoctometer".to_string(),
            },
        ];

        for test in test_units.into_iter() {
            match Area::try_from(test) {
                Ok(x) => continue,
                Err(x) => panic!("{x}"),
            }
        }
    }

    #[test]
    fn test_cross_sectional_area_try_from() {
        //01_000_000_000_000_000_000
        //18_446_744_073_709_551_615
        let test_units = vec![
            //IntermediateUnit {
            //    value: Rational64::new(5, 2),
            //    original_unit: "Ym²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 2),
            //    original_unit: "square yottameter".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "Zm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square zettameter".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "Em²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square exameter".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "Pm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square petameter".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "Tm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square terameter".to_string(),
            //},
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Gm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square gigameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Mm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square megameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "km²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square kilometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "hm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square hectometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "dam²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square decameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "m²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square meter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "dm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square decimeter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "cm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square centimeter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square millimeter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "µm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square micrometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "nm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square nanometer".to_string(),
            },
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "pm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square picometer".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "fm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square femtometer".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "am²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square attometer".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zm²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square zeptometer".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "ym²".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "square yoctometer".to_string(),
            //},
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ac".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "acre".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "a".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "are".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "b".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "barn".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "cmil".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "circular mil".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ha".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "hectare".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ft²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square foot".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "in²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square inch".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mi²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square mile".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yd²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square yard".to_string(),
            },
        ];

        let comparision_units = vec![
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_yottameter>(Rational64::new(5, 2)),
            //    original_unit: "Ym²".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_yottameter>(Rational64::new(5, 2)),
            //    original_unit: "square yottameter".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_zettameter>(Rational64::new(5, 1)),
            //    original_unit: "Zm²".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_zettameter>(Rational64::new(5, 1)),
            //    original_unit: "square zettameter".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_exameter>(Rational64::new(5, 1)),
            //    original_unit: "Em²".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_exameter>(Rational64::new(5, 1)),
            //    original_unit: "square exameter".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_petameter>(Rational64::new(5, 1)),
            //    original_unit: "Pm²".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_petameter>(Rational64::new(5, 1)),
            //    original_unit: "square petameter".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_terameter>(Rational64::new(5, 1)),
            //    original_unit: "Tm²".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_terameter>(Rational64::new(5, 1)),
            //    original_unit: "square terameter".to_string(),
            //},
            CrossSectionalArea {
                value: rational64::Area::new::<square_gigameter>(Rational64::new(5, 1)),
                original_unit: "Gm²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_gigameter>(Rational64::new(5, 1)),
                original_unit: "square gigameter".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_megameter>(Rational64::new(5, 1)),
                original_unit: "Mm²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_megameter>(Rational64::new(5, 1)),
                original_unit: "square megameter".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_kilometer>(Rational64::new(5, 1)),
                original_unit: "km²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_kilometer>(Rational64::new(5, 1)),
                original_unit: "square kilometer".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_hectometer>(Rational64::new(5, 1)),
                original_unit: "hm²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_hectometer>(Rational64::new(5, 1)),
                original_unit: "square hectometer".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_decameter>(Rational64::new(5, 1)),
                original_unit: "dam²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_decameter>(Rational64::new(5, 1)),
                original_unit: "square decameter".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_meter>(Rational64::new(5, 1)),
                original_unit: "m²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_meter>(Rational64::new(5, 1)),
                original_unit: "square meter".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_decimeter>(Rational64::new(5, 1)),
                original_unit: "dm²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_decimeter>(Rational64::new(5, 1)),
                original_unit: "square decimeter".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_centimeter>(Rational64::new(5, 1)),
                original_unit: "cm²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_centimeter>(Rational64::new(5, 1)),
                original_unit: "square centimeter".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_millimeter>(Rational64::new(5, 1)),
                original_unit: "mm²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_millimeter>(Rational64::new(5, 1)),
                original_unit: "square millimeter".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_micrometer>(Rational64::new(5, 1)),
                original_unit: "µm²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_micrometer>(Rational64::new(5, 1)),
                original_unit: "square micrometer".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_nanometer>(Rational64::new(5, 1)),
                original_unit: "nm²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_nanometer>(Rational64::new(5, 1)),
                original_unit: "square nanometer".to_string(),
            },
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_picometer>(Rational64::new(5, 1)),
            //    original_unit: "pm²".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_picometer>(Rational64::new(5, 1)),
            //    original_unit: "square picometer".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_femtometer>(Rational64::new(5, 1)),
            //    original_unit: "fm²".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_femtometer>(Rational64::new(5, 1)),
            //    original_unit: "square femtometer".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_attometer>(Rational64::new(5, 1)),
            //    original_unit: "am²".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_attometer>(Rational64::new(5, 1)),
            //    original_unit: "square attometer".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_zeptometer>(Rational64::new(5, 1)),
            //    original_unit: "zm²".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_zeptometer>(Rational64::new(5, 1)),
            //    original_unit: "square zeptometer".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_yoctometer>(Rational64::new(5, 1)),
            //    original_unit: "ym²".to_string(),
            //},
            //CrossSectionalArea {
            //    value: rational64::Area::new::<square_yoctometer>(Rational64::new(5, 1)),
            //    original_unit: "square yoctometer".to_string(),
            //},
            CrossSectionalArea {
                value: rational64::Area::new::<acre>(Rational64::new(5, 1)),
                original_unit: "ac".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<acre>(Rational64::new(5, 1)),
                original_unit: "acre".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<are>(Rational64::new(5, 1)),
                original_unit: "a".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<are>(Rational64::new(5, 1)),
                original_unit: "are".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<barn>(Rational64::new(5, 1)),
                original_unit: "b".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<barn>(Rational64::new(5, 1)),
                original_unit: "barn".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<circular_mil>(Rational64::new(5, 1)),
                original_unit: "cmil".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<circular_mil>(Rational64::new(5, 1)),
                original_unit: "circular mil".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<hectare>(Rational64::new(5, 1)),
                original_unit: "ha".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<hectare>(Rational64::new(5, 1)),
                original_unit: "hectare".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_foot>(Rational64::new(5, 1)),
                original_unit: "ft²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_foot>(Rational64::new(5, 1)),
                original_unit: "square foot".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_inch>(Rational64::new(5, 1)),
                original_unit: "in²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_inch>(Rational64::new(5, 1)),
                original_unit: "square inch".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_mile>(Rational64::new(5, 1)),
                original_unit: "mi²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_mile>(Rational64::new(5, 1)),
                original_unit: "square mile".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_yard>(Rational64::new(5, 1)),
                original_unit: "yd²".to_string(),
            },
            CrossSectionalArea {
                value: rational64::Area::new::<square_yard>(Rational64::new(5, 1)),
                original_unit: "square yard".to_string(),
            },
        ];
        for (test, comparision) in test_units.into_iter().zip(comparision_units.into_iter()) {
            match CrossSectionalArea::try_from(test) {
                Ok(x) => assert_eq!(x, comparision),
                Err(x) => panic!("{x}"),
            }
        }
    }

    #[test]
    #[should_panic(expected = "Unknown unit string")]
    fn test_cross_sectional_area_try_from_invalid_unit() {
        // assuming a user had a typo in their library file and missed the ²
        let test_value = IntermediateUnit {
            value: Rational64::new(5, 1),
            original_unit: "mm".to_string(),
        };
        let comparision_value = CrossSectionalArea {
            original_unit: "mm²".to_string(),
            value: rational64::Area::new::<square_millimeter>(Rational64::new(5, 1)),
        };

        match CrossSectionalArea::try_from(test_value) {
            Ok(x) => assert_eq!(x, comparision_value),
            Err(x) => panic!("{x}"),
        }
    }

    #[test]
    #[should_panic(expected = "Unsupported unit string")]
    fn test_cross_sectional_area_unsupported_units() {
        let test_units = vec![
            IntermediateUnit {
                value: Rational64::new(5, 2),
                original_unit: "Ym²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 2),
                original_unit: "square yottameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Zm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square zettameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Em²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square exameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Pm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square petameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Tm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square terameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "pm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square picometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "fm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square femtometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "am²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square attometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zm²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square zeptometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ym²".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "square yoctometer".to_string(),
            },
        ];

        for test in test_units.into_iter() {
            match CrossSectionalArea::try_from(test) {
                Ok(x) => continue,
                Err(x) => panic!("{x}"),
            }
        }
    }

    #[test]
    fn test_electric_potential_try_from() {
        //01_000_000_000_000_000_000
        //18_446_744_073_709_551_615
        let test_units = vec![
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "YV".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "yottavolt".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "ZV".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zettavolt".to_string(),
            //},
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "EV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "exavolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "PV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "petavolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "TV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "teravolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "GV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "gigavolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "MV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "megavolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "kV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "kilovolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "hV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "hectovolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "daV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "decavolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "V".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "volt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "dV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "decivolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "cV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "centivolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "millivolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "µV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "microvolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "nV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "nanovolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "pV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "picovolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "fV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "femtovolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "aV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "attovolt".to_string(),
            },
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zV".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zeptovolt".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "yV".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "yoctovolt".to_string(),
            //},
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "abV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "abvolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "statV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "statvolt".to_string(),
            },
        ];

        let comparision_units = vec![
            //ElectricPotential {
            //    value: rational64::ElectricPotential::new::<yottavolt>(Rational64::new(5, 1)),
            //    original_unit: "YV".to_string(),
            //},
            //ElectricPotential {
            //    value: rational64::ElectricPotential::new::<yottavolt>(Rational64::new(5, 1)),
            //    original_unit: "yottavolt".to_string(),
            //},
            //ElectricPotential {
            //    value: rational64::ElectricPotential::new::<zettavolt>(Rational64::new(5, 1)),
            //    original_unit: "ZV".to_string(),
            //},
            //ElectricPotential {
            //    value: rational64::ElectricPotential::new::<zettavolt>(Rational64::new(5, 1)),
            //    original_unit: "zettavolt".to_string(),
            //},
            ElectricPotential {
                value: rational64::ElectricPotential::new::<exavolt>(Rational64::new(5, 1)),
                original_unit: "EV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<exavolt>(Rational64::new(5, 1)),
                original_unit: "exavolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<petavolt>(Rational64::new(5, 1)),
                original_unit: "PV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<petavolt>(Rational64::new(5, 1)),
                original_unit: "petavolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<teravolt>(Rational64::new(5, 1)),
                original_unit: "TV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<teravolt>(Rational64::new(5, 1)),
                original_unit: "teravolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<gigavolt>(Rational64::new(5, 1)),
                original_unit: "GV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<gigavolt>(Rational64::new(5, 1)),
                original_unit: "gigavolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<megavolt>(Rational64::new(5, 1)),
                original_unit: "MV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<megavolt>(Rational64::new(5, 1)),
                original_unit: "megavolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<kilovolt>(Rational64::new(5, 1)),
                original_unit: "kV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<kilovolt>(Rational64::new(5, 1)),
                original_unit: "kilovolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<hectovolt>(Rational64::new(5, 1)),
                original_unit: "hV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<hectovolt>(Rational64::new(5, 1)),
                original_unit: "hectovolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<decavolt>(Rational64::new(5, 1)),
                original_unit: "daV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<decavolt>(Rational64::new(5, 1)),
                original_unit: "decavolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<volt>(Rational64::new(5, 1)),
                original_unit: "V".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<volt>(Rational64::new(5, 1)),
                original_unit: "volt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<decivolt>(Rational64::new(5, 1)),
                original_unit: "dV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<decivolt>(Rational64::new(5, 1)),
                original_unit: "decivolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<centivolt>(Rational64::new(5, 1)),
                original_unit: "cV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<centivolt>(Rational64::new(5, 1)),
                original_unit: "centivolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<millivolt>(Rational64::new(5, 1)),
                original_unit: "mV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<millivolt>(Rational64::new(5, 1)),
                original_unit: "millivolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<microvolt>(Rational64::new(5, 1)),
                original_unit: "µV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<microvolt>(Rational64::new(5, 1)),
                original_unit: "microvolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<nanovolt>(Rational64::new(5, 1)),
                original_unit: "nV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<nanovolt>(Rational64::new(5, 1)),
                original_unit: "nanovolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<picovolt>(Rational64::new(5, 1)),
                original_unit: "pV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<picovolt>(Rational64::new(5, 1)),
                original_unit: "picovolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<femtovolt>(Rational64::new(5, 1)),
                original_unit: "fV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<femtovolt>(Rational64::new(5, 1)),
                original_unit: "femtovolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<attovolt>(Rational64::new(5, 1)),
                original_unit: "aV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<attovolt>(Rational64::new(5, 1)),
                original_unit: "attovolt".to_string(),
            },
            //ElectricPotential {
            //    value: rational64::ElectricPotential::new::<zeptovolt>(Rational64::new(5, 1)),
            //    original_unit: "zV".to_string(),
            //},
            //ElectricPotential {
            //    value: rational64::ElectricPotential::new::<zeptovolt>(Rational64::new(5, 1)),
            //    original_unit: "zeptovolt".to_string(),
            //},
            //ElectricPotential {
            //    value: rational64::ElectricPotential::new::<yoctovolt>(Rational64::new(5, 1)),
            //    original_unit: "yV".to_string(),
            //},
            //ElectricPotential {
            //    value: rational64::ElectricPotential::new::<yoctovolt>(Rational64::new(5, 1)),
            //    original_unit: "yoctovolt".to_string(),
            //},
            ElectricPotential {
                value: rational64::ElectricPotential::new::<abvolt>(Rational64::new(5, 1)),
                original_unit: "abV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<abvolt>(Rational64::new(5, 1)),
                original_unit: "abvolt".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<statvolt>(Rational64::new(5, 1)),
                original_unit: "statV".to_string(),
            },
            ElectricPotential {
                value: rational64::ElectricPotential::new::<statvolt>(Rational64::new(5, 1)),
                original_unit: "statvolt".to_string(),
            },
        ];
        for (test, comparision) in test_units.into_iter().zip(comparision_units.into_iter()) {
            match ElectricPotential::try_from(test) {
                Ok(x) => assert_eq!(x, comparision),
                Err(x) => panic!("{x}"),
            }
        }
    }

    #[test]
    #[should_panic(expected = "Unknown unit string")]
    fn test_electric_potential_try_from_invalid_unit() {
        let test_value = IntermediateUnit {
            value: Rational64::new(5, 1),
            original_unit: "mm".to_string(),
        };
        let comparision_value = ElectricPotential {
            original_unit: "mV".to_string(),
            value: rational64::ElectricPotential::new::<millivolt>(Rational64::new(5, 1)),
        };

        match ElectricPotential::try_from(test_value) {
            Ok(x) => assert_eq!(x, comparision_value),
            Err(x) => panic!("{x}"),
        }
    }
    #[test]
    #[should_panic(expected = "Unsupported unit string")]
    fn test_electric_potential_unsupported_units() {
        let test_units = vec![
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "YV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yottavolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ZV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zettavolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zeptovolt".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yV".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yoctovolt".to_string(),
            },
        ];

        for test in test_units.into_iter() {
            match ElectricPotential::try_from(test) {
                Ok(x) => continue,
                Err(x) => panic!("{x}"),
            }
        }
    }

    #[test]
    fn test_length_try_from() {
        //01_000_000_000_000_000_000
        //18_446_744_073_709_551_615
        let test_units = vec![
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "Ym".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "yottameter".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "Zm".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zettameter".to_string(),
            //},
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Em".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "exameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Pm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "petameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Tm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "terameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Gm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "gigameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Mm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "megameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "km".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "kilometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "hm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "hectometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "dam".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "decameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "m".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "meter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "dm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "decimeter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "cm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "centimeter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "millimeter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "µm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "micrometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "nm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "nanometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "pm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "picometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "fm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "femtometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "am".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "attometer".to_string(),
            },
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zm".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zeptometer".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "ym".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "yoctometer".to_string(),
            //},
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Å".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ångström".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "a₀".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "bohr radius".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "a.u. of length".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "atomic unit of length".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ua".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "astronomical unit".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ch".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "chain".to_string(),
            },
            // abbreviation and singular unit name are the same currently
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "fathom".to_string(),
            },
            // abbreviation and singular unit name are the same currently
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "fermi".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ft".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "foot".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ft (U.S. survey)".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "foot (U.S. survey)".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "in".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "inch".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "l. y.".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "light year".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "μin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "microinch".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "μ".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "micron".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "0.001 in".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mil".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mi".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mile".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mi (U.S. survey)".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mile (U.S. survey)".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "M".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "nautical mile".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "pc".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "parsec".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "1/6 in (computer)".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "pica (computer)".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "1/6 in".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "pica (printer's)".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "1/72 in (computer)".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "point (computer)".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "1/72 in".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "point (printer's)".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "rd".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "rod".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yd".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yard".to_string(),
            },
        ];

        let comparision_units = vec![
            //Length {
            //    value: rational64::Length::new::<yottameter>(Rational64::new(5, 1)),
            //    original_unit: "Ym".to_string(),
            //},
            //Length {
            //    value: rational64::Length::new::<yottameter>(Rational64::new(5, 1)),
            //    original_unit: "yottameter".to_string(),
            //},
            //Length {
            //    value: rational64::Length::new::<zettameter>(Rational64::new(5, 1)),
            //    original_unit: "Zm".to_string(),
            //},
            //Length {
            //    value: rational64::Length::new::<zettameter>(Rational64::new(5, 1)),
            //    original_unit: "zettameter".to_string(),
            //},
            Length {
                value: rational64::Length::new::<exameter>(Rational64::new(5, 1)),
                original_unit: "Em".to_string(),
            },
            Length {
                value: rational64::Length::new::<exameter>(Rational64::new(5, 1)),
                original_unit: "exameter".to_string(),
            },
            Length {
                value: rational64::Length::new::<petameter>(Rational64::new(5, 1)),
                original_unit: "Pm".to_string(),
            },
            Length {
                value: rational64::Length::new::<petameter>(Rational64::new(5, 1)),
                original_unit: "petameter".to_string(),
            },
            Length {
                value: rational64::Length::new::<terameter>(Rational64::new(5, 1)),
                original_unit: "Tm".to_string(),
            },
            Length {
                value: rational64::Length::new::<terameter>(Rational64::new(5, 1)),
                original_unit: "terameter".to_string(),
            },
            Length {
                value: rational64::Length::new::<gigameter>(Rational64::new(5, 1)),
                original_unit: "Gm".to_string(),
            },
            Length {
                value: rational64::Length::new::<gigameter>(Rational64::new(5, 1)),
                original_unit: "gigameter".to_string(),
            },
            Length {
                value: rational64::Length::new::<megameter>(Rational64::new(5, 1)),
                original_unit: "Mm".to_string(),
            },
            Length {
                value: rational64::Length::new::<megameter>(Rational64::new(5, 1)),
                original_unit: "megameter".to_string(),
            },
            Length {
                value: rational64::Length::new::<kilometer>(Rational64::new(5, 1)),
                original_unit: "km".to_string(),
            },
            Length {
                value: rational64::Length::new::<kilometer>(Rational64::new(5, 1)),
                original_unit: "kilometer".to_string(),
            },
            Length {
                value: rational64::Length::new::<hectometer>(Rational64::new(5, 1)),
                original_unit: "hm".to_string(),
            },
            Length {
                value: rational64::Length::new::<hectometer>(Rational64::new(5, 1)),
                original_unit: "hectometer".to_string(),
            },
            Length {
                value: rational64::Length::new::<decameter>(Rational64::new(5, 1)),
                original_unit: "dam".to_string(),
            },
            Length {
                value: rational64::Length::new::<decameter>(Rational64::new(5, 1)),
                original_unit: "decameter".to_string(),
            },
            Length {
                value: rational64::Length::new::<meter>(Rational64::new(5, 1)),
                original_unit: "m".to_string(),
            },
            Length {
                value: rational64::Length::new::<meter>(Rational64::new(5, 1)),
                original_unit: "meter".to_string(),
            },
            Length {
                value: rational64::Length::new::<decimeter>(Rational64::new(5, 1)),
                original_unit: "dm".to_string(),
            },
            Length {
                value: rational64::Length::new::<decimeter>(Rational64::new(5, 1)),
                original_unit: "decimeter".to_string(),
            },
            Length {
                value: rational64::Length::new::<centimeter>(Rational64::new(5, 1)),
                original_unit: "cm".to_string(),
            },
            Length {
                value: rational64::Length::new::<centimeter>(Rational64::new(5, 1)),
                original_unit: "centimeter".to_string(),
            },
            Length {
                value: rational64::Length::new::<millimeter>(Rational64::new(5, 1)),
                original_unit: "mm".to_string(),
            },
            Length {
                value: rational64::Length::new::<millimeter>(Rational64::new(5, 1)),
                original_unit: "millimeter".to_string(),
            },
            Length {
                value: rational64::Length::new::<micrometer>(Rational64::new(5, 1)),
                original_unit: "µm".to_string(),
            },
            Length {
                value: rational64::Length::new::<micrometer>(Rational64::new(5, 1)),
                original_unit: "micrometer".to_string(),
            },
            Length {
                value: rational64::Length::new::<nanometer>(Rational64::new(5, 1)),
                original_unit: "nm".to_string(),
            },
            Length {
                value: rational64::Length::new::<nanometer>(Rational64::new(5, 1)),
                original_unit: "nanometer".to_string(),
            },
            Length {
                value: rational64::Length::new::<picometer>(Rational64::new(5, 1)),
                original_unit: "pm".to_string(),
            },
            Length {
                value: rational64::Length::new::<picometer>(Rational64::new(5, 1)),
                original_unit: "picometer".to_string(),
            },
            Length {
                value: rational64::Length::new::<femtometer>(Rational64::new(5, 1)),
                original_unit: "fm".to_string(),
            },
            Length {
                value: rational64::Length::new::<femtometer>(Rational64::new(5, 1)),
                original_unit: "femtometer".to_string(),
            },
            Length {
                value: rational64::Length::new::<attometer>(Rational64::new(5, 1)),
                original_unit: "am".to_string(),
            },
            Length {
                value: rational64::Length::new::<attometer>(Rational64::new(5, 1)),
                original_unit: "attometer".to_string(),
            },
            //Length {
            //    value: rational64::Length::new::<zeptometer>(Rational64::new(5, 1)),
            //    original_unit: "zm".to_string(),
            //},
            //Length {
            //    value: rational64::Length::new::<zeptometer>(Rational64::new(5, 1)),
            //    original_unit: "zeptometer".to_string(),
            //},
            //Length {
            //    value: rational64::Length::new::<yoctometer>(Rational64::new(5, 1)),
            //    original_unit: "ym".to_string(),
            //},
            //Length {
            //    value: rational64::Length::new::<yoctometer>(Rational64::new(5, 1)),
            //    original_unit: "yoctometer".to_string(),
            //},
            Length {
                value: rational64::Length::new::<angstrom>(Rational64::new(5, 1)),
                original_unit: "Å".to_string(),
            },
            Length {
                value: rational64::Length::new::<angstrom>(Rational64::new(5, 1)),
                original_unit: "ångström".to_string(),
            },
            Length {
                value: rational64::Length::new::<bohr_radius>(Rational64::new(5, 1)),
                original_unit: "a₀".to_string(),
            },
            Length {
                value: rational64::Length::new::<bohr_radius>(Rational64::new(5, 1)),
                original_unit: "bohr radius".to_string(),
            },
            Length {
                value: rational64::Length::new::<atomic_unit_of_length>(Rational64::new(5, 1)),
                original_unit: "a.u. of length".to_string(),
            },
            Length {
                value: rational64::Length::new::<atomic_unit_of_length>(Rational64::new(5, 1)),
                original_unit: "atomic unit of length".to_string(),
            },
            Length {
                value: rational64::Length::new::<astronomical_unit>(Rational64::new(5, 1)),
                original_unit: "ua".to_string(),
            },
            Length {
                value: rational64::Length::new::<astronomical_unit>(Rational64::new(5, 1)),
                original_unit: "astronomical unit".to_string(),
            },
            Length {
                value: rational64::Length::new::<chain>(Rational64::new(5, 1)),
                original_unit: "ch".to_string(),
            },
            Length {
                value: rational64::Length::new::<chain>(Rational64::new(5, 1)),
                original_unit: "chain".to_string(),
            },
            // abbreviation and singular unit name are the same currently
            Length {
                value: rational64::Length::new::<fathom>(Rational64::new(5, 1)),
                original_unit: "fathom".to_string(),
            },
            // abbreviation and singular unit name are the same currently
            Length {
                value: rational64::Length::new::<fermi>(Rational64::new(5, 1)),
                original_unit: "fermi".to_string(),
            },
            Length {
                value: rational64::Length::new::<foot>(Rational64::new(5, 1)),
                original_unit: "ft".to_string(),
            },
            Length {
                value: rational64::Length::new::<foot>(Rational64::new(5, 1)),
                original_unit: "foot".to_string(),
            },
            Length {
                value: rational64::Length::new::<foot_survey>(Rational64::new(5, 1)),
                original_unit: "ft (U.S. survey)".to_string(),
            },
            Length {
                value: rational64::Length::new::<foot_survey>(Rational64::new(5, 1)),
                original_unit: "foot (U.S. survey)".to_string(),
            },
            Length {
                value: rational64::Length::new::<inch>(Rational64::new(5, 1)),
                original_unit: "in".to_string(),
            },
            Length {
                value: rational64::Length::new::<inch>(Rational64::new(5, 1)),
                original_unit: "inch".to_string(),
            },
            Length {
                value: rational64::Length::new::<light_year>(Rational64::new(5, 1)),
                original_unit: "l. y.".to_string(),
            },
            Length {
                value: rational64::Length::new::<light_year>(Rational64::new(5, 1)),
                original_unit: "light year".to_string(),
            },
            Length {
                value: rational64::Length::new::<microinch>(Rational64::new(5, 1)),
                original_unit: "μin".to_string(),
            },
            Length {
                value: rational64::Length::new::<microinch>(Rational64::new(5, 1)),
                original_unit: "microinch".to_string(),
            },
            Length {
                value: rational64::Length::new::<micron>(Rational64::new(5, 1)),
                original_unit: "μ".to_string(),
            },
            Length {
                value: rational64::Length::new::<micron>(Rational64::new(5, 1)),
                original_unit: "micron".to_string(),
            },
            Length {
                value: rational64::Length::new::<mil>(Rational64::new(5, 1)),
                original_unit: "0.001 in".to_string(),
            },
            Length {
                value: rational64::Length::new::<mil>(Rational64::new(5, 1)),
                original_unit: "mil".to_string(),
            },
            Length {
                value: rational64::Length::new::<mile>(Rational64::new(5, 1)),
                original_unit: "mi".to_string(),
            },
            Length {
                value: rational64::Length::new::<mile>(Rational64::new(5, 1)),
                original_unit: "mile".to_string(),
            },
            Length {
                value: rational64::Length::new::<mile_survey>(Rational64::new(5, 1)),
                original_unit: "mi (U.S. survey)".to_string(),
            },
            Length {
                value: rational64::Length::new::<mile_survey>(Rational64::new(5, 1)),
                original_unit: "mile (U.S. survey)".to_string(),
            },
            Length {
                value: rational64::Length::new::<nautical_mile>(Rational64::new(5, 1)),
                original_unit: "M".to_string(),
            },
            Length {
                value: rational64::Length::new::<nautical_mile>(Rational64::new(5, 1)),
                original_unit: "nautical mile".to_string(),
            },
            Length {
                value: rational64::Length::new::<parsec>(Rational64::new(5, 1)),
                original_unit: "pc".to_string(),
            },
            Length {
                value: rational64::Length::new::<parsec>(Rational64::new(5, 1)),
                original_unit: "parsec".to_string(),
            },
            Length {
                value: rational64::Length::new::<pica_computer>(Rational64::new(5, 1)),
                original_unit: "1/6 in (computer)".to_string(),
            },
            Length {
                value: rational64::Length::new::<pica_computer>(Rational64::new(5, 1)),
                original_unit: "pica (computer)".to_string(),
            },
            Length {
                value: rational64::Length::new::<pica_printers>(Rational64::new(5, 1)),
                original_unit: "1/6 in".to_string(),
            },
            Length {
                value: rational64::Length::new::<pica_printers>(Rational64::new(5, 1)),
                original_unit: "pica (printer's)".to_string(),
            },
            Length {
                value: rational64::Length::new::<point_computer>(Rational64::new(5, 1)),
                original_unit: "1/72 in (computer)".to_string(),
            },
            Length {
                value: rational64::Length::new::<point_computer>(Rational64::new(5, 1)),
                original_unit: "point (computer)".to_string(),
            },
            Length {
                value: rational64::Length::new::<point_printers>(Rational64::new(5, 1)),
                original_unit: "1/72 in".to_string(),
            },
            Length {
                value: rational64::Length::new::<point_printers>(Rational64::new(5, 1)),
                original_unit: "point (printer's)".to_string(),
            },
            Length {
                value: rational64::Length::new::<rod>(Rational64::new(5, 1)),
                original_unit: "rd".to_string(),
            },
            Length {
                value: rational64::Length::new::<rod>(Rational64::new(5, 1)),
                original_unit: "rod".to_string(),
            },
            Length {
                value: rational64::Length::new::<yard>(Rational64::new(5, 1)),
                original_unit: "yd".to_string(),
            },
            Length {
                value: rational64::Length::new::<yard>(Rational64::new(5, 1)),
                original_unit: "yard".to_string(),
            },
        ];
        for (test, comparision) in test_units.into_iter().zip(comparision_units.into_iter()) {
            match Length::try_from(test) {
                Ok(x) => assert_eq!(x, comparision),
                Err(x) => panic!("{x}"),
            }
        }
    }

    #[test]
    #[should_panic(expected = "Unknown unit string")]
    fn test_length_try_from_invalid_unit() {
        // assuming a user had a typo in their library file and missed the ²
        let test_value = IntermediateUnit {
            value: Rational64::new(5, 1),
            original_unit: "mm²".to_string(),
        };
        let comparision_value = Length {
            original_unit: "mm".to_string(),
            value: rational64::Length::new::<millimeter>(Rational64::new(5, 1)),
        };

        match Length::try_from(test_value) {
            Ok(x) => assert_eq!(x, comparision_value),
            Err(x) => panic!("{x}"),
        }
    }
    #[test]
    #[should_panic(expected = "Unsupported unit string")]
    fn test_length_unsupported_units() {
        let test_units = vec![
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Ym".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yottameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "Zm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zettameter".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zm".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zeptometer".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ym".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yoctometer".to_string(),
            },
        ];

        for test in test_units.into_iter() {
            match Length::try_from(test) {
                Ok(x) => continue,
                Err(x) => panic!("{x}"),
            }
        }
    }

    #[test]
    fn test_temp_interval_try_from() {
        //01_000_000_000_000_000_000
        //18_446_744_073_709_551_615
        let test_units = vec![
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "YK".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "yottakelvin".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "ZK".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zettakelvin".to_string(),
            //},
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "EK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "exakelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "PK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "petakelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "TK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "terakelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "GK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "gigakelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "MK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "megakelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "kK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "kilokelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "hK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "hectokelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "daK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "decakelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "K".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "kelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "dK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "decikelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "cK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "centikelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "mK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "millikelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "µK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "microkelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "nK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "nanokelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "pK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "picokelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "fK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "femtokelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "aK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "attokelvin".to_string(),
            },
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zK".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "zeptokelvin".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "yK".to_string(),
            //},
            //IntermediateUnit {
            //    value: Rational64::new(5, 1),
            //    original_unit: "yoctokelvin".to_string(),
            //},
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "°C".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "degree Celsius".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "°F".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "degree Fahrenheit".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "°R".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "degree Rankine".to_string(),
            },
        ];

        let comparision_units = vec![
            //TemperatureInterval {
            //    value: rational64::TemperatureInterval::new::<yottakelvin>(Rational64::new(5, 1)),
            //    original_unit: "YK".to_string(),
            //},
            //TemperatureInterval {
            //    value: rational64::TemperatureInterval::new::<yottakelvin>(Rational64::new(5, 1)),
            //    original_unit: "yottakelvin".to_string(),
            //},
            //TemperatureInterval {
            //    value: rational64::TemperatureInterval::new::<zettakelvin>(Rational64::new(5, 1)),
            //    original_unit: "ZK".to_string(),
            //},
            //TemperatureInterval {
            //    value: rational64::TemperatureInterval::new::<zettakelvin>(Rational64::new(5, 1)),
            //    original_unit: "zettakelvin".to_string(),
            //},
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<exakelvin>(Rational64::new(5, 1)),
                original_unit: "EK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<exakelvin>(Rational64::new(5, 1)),
                original_unit: "exakelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<petakelvin>(Rational64::new(5, 1)),
                original_unit: "PK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<petakelvin>(Rational64::new(5, 1)),
                original_unit: "petakelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<terakelvin>(Rational64::new(5, 1)),
                original_unit: "TK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<terakelvin>(Rational64::new(5, 1)),
                original_unit: "terakelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<gigakelvin>(Rational64::new(5, 1)),
                original_unit: "GK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<gigakelvin>(Rational64::new(5, 1)),
                original_unit: "gigakelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<megakelvin>(Rational64::new(5, 1)),
                original_unit: "MK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<megakelvin>(Rational64::new(5, 1)),
                original_unit: "megakelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<kilokelvin>(Rational64::new(5, 1)),
                original_unit: "kK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<kilokelvin>(Rational64::new(5, 1)),
                original_unit: "kilokelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<hectokelvin>(Rational64::new(5, 1)),
                original_unit: "hK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<hectokelvin>(Rational64::new(5, 1)),
                original_unit: "hectokelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<decakelvin>(Rational64::new(5, 1)),
                original_unit: "daK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<decakelvin>(Rational64::new(5, 1)),
                original_unit: "decakelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<kelvin>(Rational64::new(5, 1)),
                original_unit: "K".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<kelvin>(Rational64::new(5, 1)),
                original_unit: "kelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<decikelvin>(Rational64::new(5, 1)),
                original_unit: "dK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<decikelvin>(Rational64::new(5, 1)),
                original_unit: "decikelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<centikelvin>(Rational64::new(5, 1)),
                original_unit: "cK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<centikelvin>(Rational64::new(5, 1)),
                original_unit: "centikelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<millikelvin>(Rational64::new(5, 1)),
                original_unit: "mK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<millikelvin>(Rational64::new(5, 1)),
                original_unit: "millikelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<microkelvin>(Rational64::new(5, 1)),
                original_unit: "µK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<microkelvin>(Rational64::new(5, 1)),
                original_unit: "microkelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<nanokelvin>(Rational64::new(5, 1)),
                original_unit: "nK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<nanokelvin>(Rational64::new(5, 1)),
                original_unit: "nanokelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<picokelvin>(Rational64::new(5, 1)),
                original_unit: "pK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<picokelvin>(Rational64::new(5, 1)),
                original_unit: "picokelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<femtokelvin>(Rational64::new(5, 1)),
                original_unit: "fK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<femtokelvin>(Rational64::new(5, 1)),
                original_unit: "femtokelvin".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<attokelvin>(Rational64::new(5, 1)),
                original_unit: "aK".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<attokelvin>(Rational64::new(5, 1)),
                original_unit: "attokelvin".to_string(),
            },
            //TemperatureInterval {
            //    value: rational64::TemperatureInterval::new::<zeptokelvin>(Rational64::new(5, 1)),
            //    original_unit: "zK".to_string(),
            //},
            //TemperatureInterval {
            //    value: rational64::TemperatureInterval::new::<zeptokelvin>(Rational64::new(5, 1)),
            //    original_unit: "zeptokelvin".to_string(),
            //},
            //TemperatureInterval {
            //    value: rational64::TemperatureInterval::new::<yoctokelvin>(Rational64::new(5, 1)),
            //    original_unit: "yK".to_string(),
            //},
            //TemperatureInterval {
            //    value: rational64::TemperatureInterval::new::<yoctokelvin>(Rational64::new(5, 1)),
            //    original_unit: "yoctokelvin".to_string(),
            //},
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<degree_celsius>(Rational64::new(5, 1)),
                original_unit: "°C".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<degree_celsius>(Rational64::new(5, 1)),
                original_unit: "degree Celsius".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<degree_fahrenheit>(Rational64::new(5, 1)),
                original_unit: "°F".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<degree_fahrenheit>(Rational64::new(5, 1)),
                original_unit: "degree Fahrenheit".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<degree_rankine>(Rational64::new(5, 1)),
                original_unit: "°R".to_string(),
            },
            TemperatureInterval {
                value: rational64::TemperatureInterval::new::<degree_rankine>(Rational64::new(5, 1)),
                original_unit: "degree Rankine".to_string(),
            },
        ];
        for (test, comparision) in test_units.into_iter().zip(comparision_units.into_iter()) {
            match TemperatureInterval::try_from(test) {
                Ok(x) => assert_eq!(x, comparision),
                Err(x) => panic!("{x}"),
            }
        }
    }

    #[test]
    #[should_panic(expected = "Unknown unit string")]
    fn test_temp_interval_try_from_invalid_unit() {
        // assuming a user had a typo in their library file and missed the ²
        let test_value = IntermediateUnit {
            value: Rational64::new(5, 1),
            original_unit: "mm".to_string(),
        };
        let comparision_value = TemperatureInterval {
            original_unit: "mK".to_string(),
            value: rational64::TemperatureInterval::new::<millikelvin>(Rational64::new(5, 1)),
        };

        match TemperatureInterval::try_from(test_value) {
            Ok(x) => assert_eq!(x, comparision_value),
            Err(x) => panic!("{x}"),
        }
    }

    #[test]
    #[should_panic(expected = "Unsupported unit string")]
    fn test_temp_interval_unsupported_units() {
        let test_units = vec![
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "YK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yottakelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "ZK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zettakelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "zeptokelvin".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yK".to_string(),
            },
            IntermediateUnit {
                value: Rational64::new(5, 1),
                original_unit: "yoctokelvin".to_string(),
            },
        ];

        for test in test_units.into_iter() {
            match TemperatureInterval::try_from(test) {
                Ok(x) => continue,
                Err(x) => panic!("{x}"),
            }
        }
    }
}
