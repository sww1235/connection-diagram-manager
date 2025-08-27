//! test

use uom::{
    num::Zero,
    si::{length::millimeter, rational64::Length},
};

fn main() {
    let test = Length::zero();

    //let test2: Length = ucum::Meter::default();

    println!("zero: {}", test.get::<millimeter>());
    //println!("default: {test2}");
}
