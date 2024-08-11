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
            PaperSize::A0 => {
                if f.alternate() {
                    writeln!(f, "{}×{}", PaperSize::A0.size().0, PaperSize::A0.size().1)?;
                } else {
                    writeln!(f, "A0")?;
                }
            }
            PaperSize::A1 => {
                if f.alternate() {
                    writeln!(f, "{}×{}", PaperSize::A1.size().0, PaperSize::A1.size().1)?;
                } else {
                    writeln!(f, "A1")?;
                }
            }
            PaperSize::A2 => {
                if f.alternate() {
                    writeln!(f, "{}×{}", PaperSize::A2.size().0, PaperSize::A2.size().1)?;
                } else {
                    writeln!(f, "A2")?;
                }
            }
            PaperSize::A3 => {
                if f.alternate() {
                    writeln!(f, "{}×{}", PaperSize::A3.size().0, PaperSize::A3.size().1)?;
                } else {
                    writeln!(f, "A3")?;
                }
            }
            PaperSize::A4 => {
                if f.alternate() {
                    writeln!(f, "{}×{}", PaperSize::A4.size().0, PaperSize::A4.size().1)?;
                } else {
                    writeln!(f, "A4")?;
                }
            }
            PaperSize::A5 => {
                if f.alternate() {
                    writeln!(f, "{}×{}", PaperSize::A5.size().0, PaperSize::A5.size().1)?;
                } else {
                    writeln!(f, "A5")?;
                }
            }
            PaperSize::A6 => {
                if f.alternate() {
                    writeln!(f, "{}×{}", PaperSize::A6.size().0, PaperSize::A6.size().1)?;
                } else {
                    writeln!(f, "A6")?;
                }
            }
            PaperSize::AnsiA => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::AnsiA.size().0,
                        PaperSize::AnsiA.size().1
                    )?;
                } else {
                    writeln!(f, "ANSI A")?;
                }
            }
            PaperSize::AnsiB => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::AnsiB.size().0,
                        PaperSize::AnsiB.size().1
                    )?;
                } else {
                    writeln!(f, "ANSI B")?;
                }
            }
            PaperSize::AnsiC => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::AnsiC.size().0,
                        PaperSize::AnsiC.size().1
                    )?;
                } else {
                    writeln!(f, "ANSI C")?;
                }
            }
            PaperSize::AnsiD => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::AnsiD.size().0,
                        PaperSize::AnsiD.size().1
                    )?;
                } else {
                    writeln!(f, "ANSI D")?;
                }
            }
            PaperSize::AnsiE => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::AnsiE.size().0,
                        PaperSize::AnsiE.size().1
                    )?;
                } else {
                    writeln!(f, "ANSI E")?;
                }
            }
            PaperSize::ArchA => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::ArchA.size().0,
                        PaperSize::ArchA.size().1
                    )?;
                } else {
                    writeln!(f, "ARCH A")?;
                }
            }
            PaperSize::ArchB => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::ArchB.size().0,
                        PaperSize::ArchB.size().1
                    )?;
                } else {
                    writeln!(f, "ARCH B")?;
                }
            }
            PaperSize::ArchC => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::ArchC.size().0,
                        PaperSize::ArchC.size().1
                    )?;
                } else {
                    writeln!(f, "ARCH C")?;
                }
            }
            PaperSize::ArchD => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::ArchD.size().0,
                        PaperSize::ArchD.size().1
                    )?;
                } else {
                    writeln!(f, "ARCH D")?;
                }
            }
            PaperSize::ArchE => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::ArchE.size().0,
                        PaperSize::ArchE.size().1
                    )?;
                } else {
                    writeln!(f, "ARCH E")?;
                }
            }
            PaperSize::Tabloid => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::Tabloid.size().0,
                        PaperSize::Tabloid.size().1
                    )?;
                } else {
                    writeln!(f, "TABLOID")?;
                }
            }
            PaperSize::Legal => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::Legal.size().0,
                        PaperSize::Legal.size().1
                    )?;
                } else {
                    writeln!(f, "LEGAL")?;
                }
            }
            PaperSize::Letter => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::Letter.size().0,
                        PaperSize::Letter.size().1
                    )?;
                } else {
                    writeln!(f, "LETTER")?;
                }
            }
            PaperSize::Custom(x, y) => {
                if f.alternate() {
                    writeln!(f, "{}×{}", x, y)?;
                } else {
                    writeln!(f, "CUSTOM")?;
                }
            }
        }
        Ok(())
    }
}
