//! test

use dimensioned::ucum;

fn main() {
    #[allow(clippy::arithmetic_side_effects)]
    let test = 0.0_f64 * ucum::M;

    let test2: ucum::Meter<f64> = ucum::Meter::default();

    println!("zero: {test}");
    println!("default: {test2}");
}
