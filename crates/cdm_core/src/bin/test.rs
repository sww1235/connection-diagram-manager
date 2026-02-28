//! test.

use uom::{
    num::Zero as _,
    si::{length::millimeter, rational64::Length},
};

#[expect(clippy::print_stdout, reason = "test binary")]
fn main() {
    let test = Length::zero();

    //let test2: Length = ucum::Meter::default();

    println!("zero: {}", test.get::<millimeter>());
    //println!("default: {test2}");
}
