use std::fmt;

use num_rational::Rational64;
use uom::si::{
    length::{inch, millimeter},
    rational64::Length,
};

/// `PaperSize` represents standard paper sizes,
/// along with options to declare custom sizes in various units.
/// The provided paper sizes are defined in portrait orientation,
/// with the long edge as the Y coordinate, or height, and
/// the short edge as the X coordinate or width.
/// Custom sizes should be defined similarly.
#[expect(clippy::module_name_repetitions)]
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
    Custom(Length, Length),
}

impl PaperSize {
    /// `size` outputs the short and long side measurements of the specified
    /// paper size as a tuple in the specified order.
    #[must_use]
    pub fn size(self) -> (Length, Length) {
        match self {
            // ISO paper sizes are specified in mm
            PaperSize::A0 => (
                Length::new::<millimeter>(Rational64::from_integer(841)),
                Length::new::<millimeter>(Rational64::from_integer(1189)),
            ),
            PaperSize::A1 => (
                Length::new::<millimeter>(Rational64::from_integer(594)),
                Length::new::<millimeter>(Rational64::from_integer(841)),
            ),
            PaperSize::A2 => (
                Length::new::<millimeter>(Rational64::from_integer(420)),
                Length::new::<millimeter>(Rational64::from_integer(594)),
            ),
            PaperSize::A3 => (
                Length::new::<millimeter>(Rational64::from_integer(297)),
                Length::new::<millimeter>(Rational64::from_integer(420)),
            ),
            PaperSize::A4 => (
                Length::new::<millimeter>(Rational64::from_integer(210)),
                Length::new::<millimeter>(Rational64::from_integer(297)),
            ),
            PaperSize::A5 => (
                Length::new::<millimeter>(Rational64::from_integer(148)),
                Length::new::<millimeter>(Rational64::from_integer(210)),
            ),
            PaperSize::A6 => (
                Length::new::<millimeter>(Rational64::from_integer(105)),
                Length::new::<millimeter>(Rational64::from_integer(148)),
            ),
            PaperSize::AnsiA | PaperSize::Letter => (
                Length::new::<inch>(Rational64::new(17, 2)),
                Length::new::<inch>(Rational64::from_integer(11)),
            ),
            PaperSize::AnsiB | PaperSize::Tabloid => (
                Length::new::<inch>(Rational64::from_integer(11)),
                Length::new::<inch>(Rational64::from_integer(17)),
            ),
            PaperSize::AnsiC => (
                Length::new::<inch>(Rational64::from_integer(17)),
                Length::new::<inch>(Rational64::from_integer(22)),
            ),
            PaperSize::AnsiD => (
                Length::new::<inch>(Rational64::from_integer(22)),
                Length::new::<inch>(Rational64::from_integer(34)),
            ),
            PaperSize::AnsiE => (
                Length::new::<inch>(Rational64::from_integer(34)),
                Length::new::<inch>(Rational64::from_integer(44)),
            ),
            PaperSize::ArchA => (
                Length::new::<inch>(Rational64::from_integer(9)),
                Length::new::<inch>(Rational64::from_integer(12)),
            ),
            PaperSize::ArchB => (
                Length::new::<inch>(Rational64::from_integer(12)),
                Length::new::<inch>(Rational64::from_integer(18)),
            ),
            PaperSize::ArchC => (
                Length::new::<inch>(Rational64::from_integer(18)),
                Length::new::<inch>(Rational64::from_integer(24)),
            ),
            PaperSize::ArchD => (
                Length::new::<inch>(Rational64::from_integer(24)),
                Length::new::<inch>(Rational64::from_integer(36)),
            ),
            PaperSize::ArchE => (
                Length::new::<inch>(Rational64::from_integer(36)),
                Length::new::<inch>(Rational64::from_integer(48)),
            ),
            PaperSize::Legal => (
                Length::new::<inch>(Rational64::new(17, 2)),
                Length::new::<inch>(Rational64::from_integer(14)),
            ),
            PaperSize::Custom(short, long) => (short, long),
        }
    }
}

impl fmt::Display for PaperSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PaperSize::A0 => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::A0.size().0.get::<millimeter>(),
                        PaperSize::A0.size().1.get::<millimeter>()
                    )?;
                } else {
                    writeln!(f, "A0")?;
                }
            }
            PaperSize::A1 => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::A1.size().0.get::<millimeter>(),
                        PaperSize::A1.size().1.get::<millimeter>()
                    )?;
                } else {
                    writeln!(f, "A1")?;
                }
            }
            PaperSize::A2 => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::A2.size().0.get::<millimeter>(),
                        PaperSize::A2.size().1.get::<millimeter>()
                    )?;
                } else {
                    writeln!(f, "A2")?;
                }
            }
            PaperSize::A3 => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::A3.size().0.get::<millimeter>(),
                        PaperSize::A3.size().1.get::<millimeter>()
                    )?;
                } else {
                    writeln!(f, "A3")?;
                }
            }
            PaperSize::A4 => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::A4.size().0.get::<millimeter>(),
                        PaperSize::A4.size().1.get::<millimeter>()
                    )?;
                } else {
                    writeln!(f, "A4")?;
                }
            }
            PaperSize::A5 => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::A5.size().0.get::<millimeter>(),
                        PaperSize::A5.size().1.get::<millimeter>()
                    )?;
                } else {
                    writeln!(f, "A5")?;
                }
            }
            PaperSize::A6 => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::A6.size().0.get::<millimeter>(),
                        PaperSize::A6.size().1.get::<millimeter>()
                    )?;
                } else {
                    writeln!(f, "A6")?;
                }
            }
            PaperSize::AnsiA => {
                if f.alternate() {
                    writeln!(
                        f,
                        "{}×{}",
                        PaperSize::AnsiA.size().0.get::<inch>(),
                        PaperSize::AnsiA.size().1.get::<inch>()
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
                        PaperSize::AnsiB.size().0.get::<inch>(),
                        PaperSize::AnsiB.size().1.get::<inch>()
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
                        PaperSize::AnsiC.size().0.get::<inch>(),
                        PaperSize::AnsiC.size().1.get::<inch>()
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
                        PaperSize::AnsiD.size().0.get::<inch>(),
                        PaperSize::AnsiD.size().1.get::<inch>()
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
                        PaperSize::AnsiE.size().0.get::<inch>(),
                        PaperSize::AnsiE.size().1.get::<inch>()
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
                        PaperSize::ArchA.size().0.get::<inch>(),
                        PaperSize::ArchA.size().1.get::<inch>()
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
                        PaperSize::ArchB.size().0.get::<inch>(),
                        PaperSize::ArchB.size().1.get::<inch>()
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
                        PaperSize::ArchC.size().0.get::<inch>(),
                        PaperSize::ArchC.size().1.get::<inch>()
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
                        PaperSize::ArchD.size().0.get::<inch>(),
                        PaperSize::ArchD.size().1.get::<inch>()
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
                        PaperSize::ArchE.size().0.get::<inch>(),
                        PaperSize::ArchE.size().1.get::<inch>()
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
                        PaperSize::Tabloid.size().0.get::<inch>(),
                        PaperSize::Tabloid.size().1.get::<inch>()
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
                        PaperSize::Legal.size().0.get::<inch>(),
                        PaperSize::Legal.size().1.get::<inch>()
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
                        PaperSize::Letter.size().0.get::<inch>(),
                        PaperSize::Letter.size().1.get::<inch>()
                    )?;
                } else {
                    writeln!(f, "LETTER")?;
                }
            }
            PaperSize::Custom(x, y) => {
                if f.alternate() {
                    //writeln!(f, "{}×{}", x, y)?;
                } else {
                    writeln!(f, "CUSTOM")?;
                }
            }
        }
        Ok(())
    }
}
