use dimensioned::{f64prefixes, ucum};
use std::fmt;
/// `PaperSize` represents standard paper sizes,
/// along with options to declare custom sizes in various units.
/// The provided paper sizes are defined in portrait orientation,
/// with the long edge as the Y coordinate, or height, and
/// the short edge as the X coordinate or width.
/// Custom sizes should be defined similarly.
#[allow(clippy::module_name_repetitions)]
#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum PaperSize {
    /// ISO A0 paper size
    A0,
    /// ISO A1 paper size
    A1,
    /// ISO A2 paper size
    A2,
    /// ISO A3 paper size
    A3,
    /// ISO A4 paper size
    A4,
    /// ISO A5 paper size
    A5,
    /// ISO A6 paper size
    A6,
    /// Tabloid American paper size, portrait orientation of Ledger size
    Tabloid,
    /// Legal American paper size
    Legal,
    /// Letter American paper size
    Letter,
    /// ANSI A paper size
    AnsiA,
    /// ANSI B paper size
    AnsiB,
    /// ANSI C paper size
    AnsiC,
    /// ANSI D paper size
    AnsiD,
    /// ANSI E paper size
    AnsiE,
    /// ANSI Arch A paper size
    ArchA,
    /// ANSI Arch B paper size
    ArchB,
    /// ANSI Arch C paper size
    ArchC,
    /// ANSI Arch D paper size
    ArchD,
    /// ANSI Arch E paper size
    ArchE,
    /// custom paper size
    Custom(ucum::Meter<f64>, ucum::Meter<f64>),
}

impl PaperSize {
    /// `size` outputs the short and long side measurements of the specified
    /// paper size as a tuple in the specified order.
    #[allow(clippy::arithmetic_side_effects)]
    #[must_use]
    pub fn size(self) -> (ucum::Meter<f64>, ucum::Meter<f64>) {
        match self {
            // ISO paper sizes are specified in mm
            PaperSize::A0 => (
                841.0_f64 * ucum::M * f64prefixes::MILLI,
                1189.0_f64 * ucum::M * f64prefixes::MILLI,
            ),
            PaperSize::A1 => (
                594.0_f64 * ucum::M * f64prefixes::MILLI,
                841.0_f64 * ucum::M * f64prefixes::MILLI,
            ),
            PaperSize::A2 => (
                420.0_f64 * ucum::M * f64prefixes::MILLI,
                594.0_f64 * ucum::M * f64prefixes::MILLI,
            ),
            PaperSize::A3 => (
                297.0_f64 * ucum::M * f64prefixes::MILLI,
                420.0_f64 * ucum::M * f64prefixes::MILLI,
            ),
            PaperSize::A4 => (
                210.0_f64 * ucum::M * f64prefixes::MILLI,
                297.0_f64 * ucum::M * f64prefixes::MILLI,
            ),
            PaperSize::A5 => (
                148.0_f64 * ucum::M * f64prefixes::MILLI,
                210.0_f64 * ucum::M * f64prefixes::MILLI,
            ),
            PaperSize::A6 => (
                105.0_f64 * ucum::M * f64prefixes::MILLI,
                148.0_f64 * ucum::M * f64prefixes::MILLI,
            ),
            PaperSize::AnsiA => (8.5_f64 * ucum::IN_US, 11.0_f64 * ucum::IN_US),
            PaperSize::AnsiB => (11.0_f64 * ucum::IN_US, 17.0_f64 * ucum::IN_US),
            PaperSize::AnsiC => (17.0_f64 * ucum::IN_US, 22.0_f64 * ucum::IN_US),
            PaperSize::AnsiD => (22.0_f64 * ucum::IN_US, 34.0_f64 * ucum::IN_US),
            PaperSize::AnsiE => (34.0_f64 * ucum::IN_US, 44.0_f64 * ucum::IN_US),
            PaperSize::ArchA => (9.0_f64 * ucum::IN_US, 12.0_f64 * ucum::IN_US),
            PaperSize::ArchB => (12.0_f64 * ucum::IN_US, 18.0_f64 * ucum::IN_US),
            PaperSize::ArchC => (18.0_f64 * ucum::IN_US, 24.0_f64 * ucum::IN_US),
            PaperSize::ArchD => (24.0_f64 * ucum::IN_US, 36.0_f64 * ucum::IN_US),
            PaperSize::ArchE => (36.0_f64 * ucum::IN_US, 48.0_f64 * ucum::IN_US),
            PaperSize::Tabloid => PaperSize::AnsiB.size(),
            PaperSize::Legal => (8.5_f64 * ucum::IN_US, 14.0_f64 * ucum::IN_US),
            PaperSize::Letter => PaperSize::AnsiA.size(),
            PaperSize::Custom(short, long) => (short, long),
        }
    }
}

impl fmt::Display for PaperSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            //TODO: finish alt mode display implementation
            PaperSize::A0 => {
                if f.alternate() {
                    writeln!(f, "")?;
                } else {
                    writeln!(f, "A0")?;
                }
            }
            PaperSize::A1 => {
                writeln!(f, "A1")?;
            }
            PaperSize::A2 => {
                writeln!(f, "A2")?;
            }
            PaperSize::A3 => {
                writeln!(f, "A3")?;
            }
            PaperSize::A4 => {
                writeln!(f, "A4")?;
            }
            PaperSize::A5 => {
                writeln!(f, "A5")?;
            }
            PaperSize::A6 => {
                writeln!(f, "A6")?;
            }
            PaperSize::AnsiA => {
                writeln!(f, "ANSI A")?;
            }
            PaperSize::AnsiB => {
                writeln!(f, "ANSI B")?;
            }
            PaperSize::AnsiC => {
                writeln!(f, "ANSI C")?;
            }
            PaperSize::AnsiD => {
                writeln!(f, "ANSI D")?;
            }
            PaperSize::AnsiE => {
                writeln!(f, "ANSI E")?;
            }
            PaperSize::ArchA => {
                writeln!(f, "ARCH A")?;
            }
            PaperSize::ArchB => {
                writeln!(f, "ARCH B")?;
            }
            PaperSize::ArchC => {
                writeln!(f, "ARCH C")?;
            }
            PaperSize::ArchD => {
                writeln!(f, "ARCH D")?;
            }
            PaperSize::ArchE => {
                writeln!(f, "ARCH E")?;
            }
            PaperSize::Tabloid => {
                writeln!(f, "TABLOID")?;
            }
            PaperSize::Legal => {
                writeln!(f, "LEGAL")?;
            }
            PaperSize::Letter => {
                writeln!(f, "LETTER")?;
            }
            PaperSize::Custom(_, _) => {
                writeln!(f, "CUSTOM")?;
            }
        }
        Ok(())
    }
}
