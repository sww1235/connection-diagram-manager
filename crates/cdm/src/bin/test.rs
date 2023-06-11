use dimensioned::ucum;

fn main() {
    let test = 0.0 * ucum::M;

    let test2: ucum::Meter<f64> = ucum::Meter::default();

    println!("zero: {test}");
    println!("default: {test2}");
}
