use std::collections::HashMap;
use std::fmt;

use super::{Empty, Mergable, PartialEmpty};

/// `WireType` represents a particular type of wire
///
/// Not all fields have to be populated, and some are
/// mainly provided for logical reasons rather than
/// functional (model/part number/manufacturer part number
/// may all be equivalent in some cases)
#[derive(Debug, Default, PartialEq)]
pub struct WireType {
    /// Internal ID of `WireType`
    pub id: String,
    /// The manufacturer of the wire
    pub manufacturer: Option<String>,
    /// The model or type of wire
    pub model: Option<String>,
    /// The part number of this wire type
    pub part_number: Option<String>,
    /// The part number assigned by the manufacturer
    pub manufacturer_part_number: Option<String>,
    /// Where this wire was purchased from
    pub supplier: Option<String>,
    /// The part number the supplier uses
    pub supplier_part_number: Option<String>,
    /// The material the conductor or central element
    /// of the wire is made out of
    pub material: Option<String>,
    /// If the wire is insulated
    pub insulated: bool,
    /// What material the wire is insulated with
    pub insulation_material: Option<String>,
    /// The standard wire type code (THHN, XHHW, SIS, etc)
    pub wire_type_code: Option<String>,
    /// Conductor cross sectional area.
    /// specified in mm^2
    pub conductor_cross_sect_area: f64,
    /// Overall wire cross sectional area, incluidng insulation.
    /// specified in mm^2
    pub overall_cross_sect_area: f64,
    /// If conductor is stranded
    pub stranded: bool,
    /// How many strands is conductor made of
    pub num_strands: Option<u64>,
    /// cross sectional area of individual strand.
    /// specified in mm^2
    pub strand_cross_sect_area: Option<f64>,
    /// Insulation voltage rating.
    /// Specified in volts
    pub insul_volt_rating: Option<u64>,
    /// Insulation temperature rating.
    /// Specified in ℃
    pub insul_temp_rating: Option<u64>,
    /// Insulation Color
    pub insul_color: Option<String>,
}

impl WireType {
    /// Creates an empty instance of `WireType`
    #[allow(clippy::arithmetic_side_effects)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: String::new(),
            manufacturer: None,
            model: None,
            part_number: None,
            manufacturer_part_number: None,
            supplier: None,
            supplier_part_number: None,
            material: None,
            insulated: false,
            insulation_material: None,
            wire_type_code: None,
            conductor_cross_sect_area: 0.0,
            overall_cross_sect_area: 0.0,
            stranded: false,
            num_strands: None,
            strand_cross_sect_area: None,
            insul_volt_rating: None,
            insul_temp_rating: None,
            insul_color: None,
        }
    }
}

impl Mergable for WireType {
    #[allow(clippy::too_many_lines)]
    // TODO: see if this can be split up
    fn merge_prompt(
        &mut self,
        other: &Self,
        prompt_fn: fn(HashMap<String, [String; 2]>) -> HashMap<String, bool>,
    ) {
        //TODO: maybe check for partial_empty/empty here on other
        let mut input_map: HashMap<String, [String; 2]> = HashMap::new();
        if self.id != other.id {
            panic! {"attempting to merge structs with different IDs. This shouldn't have happened."}
        }
        if self.manufacturer != other.manufacturer {
            input_map.insert(
                "Manufacturer".to_string(),
                [
                    {
                        if let Some(manufacturer) = self.manufacturer.clone() {
                            manufacturer
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(manufacturer) = other.manufacturer.clone() {
                            manufacturer
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.model != other.model {
            input_map.insert(
                "Model".to_string(),
                [
                    {
                        if let Some(model) = self.model.clone() {
                            model
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(model) = other.model.clone() {
                            model
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.part_number != other.part_number {
            input_map.insert(
                "Part Number".to_string(),
                [
                    {
                        if let Some(part_number) = self.part_number.clone() {
                            part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(part_number) = other.part_number.clone() {
                            part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.manufacturer_part_number != other.manufacturer_part_number {
            input_map.insert(
                "Manufacturer Part Number".to_string(),
                [
                    {
                        if let Some(manufacturer_part_number) =
                            self.manufacturer_part_number.clone()
                        {
                            manufacturer_part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(manufacturer_part_number) =
                            other.manufacturer_part_number.clone()
                        {
                            manufacturer_part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.supplier != other.supplier {
            input_map.insert(
                "Supplier".to_string(),
                [
                    {
                        if let Some(supplier) = self.supplier.clone() {
                            supplier
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(supplier) = other.supplier.clone() {
                            supplier
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.supplier_part_number != other.supplier_part_number {
            input_map.insert(
                "Supplier Part Number".to_string(),
                [
                    {
                        if let Some(supplier_part_number) = self.supplier_part_number.clone() {
                            supplier_part_number
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(supplier_part_number) = other.supplier_part_number.clone() {
                            supplier_part_number
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.material != other.material {
            input_map.insert(
                "Material".to_string(),
                [
                    {
                        if let Some(material) = self.material.clone() {
                            material
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(material) = other.material.clone() {
                            material
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.insulated != other.insulated {
            input_map.insert(
                "Insulated".to_string(),
                [self.insulated.to_string(), other.insulated.to_string()],
            );
        }
        if self.insulation_material != other.insulation_material {
            input_map.insert(
                "Insulation Material".to_string(),
                [
                    {
                        if let Some(insulation_material) = self.insulation_material.clone() {
                            insulation_material
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(insulation_material) = other.insulation_material.clone() {
                            insulation_material
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.wire_type_code != other.wire_type_code {
            input_map.insert(
                "Wire Type Code".to_string(),
                [
                    {
                        if let Some(wire_type_code) = self.wire_type_code.clone() {
                            wire_type_code
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(wire_type_code) = other.wire_type_code.clone() {
                            wire_type_code
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.conductor_cross_sect_area != other.conductor_cross_sect_area {
            input_map.insert(
                "Conductor Cross Sectional Area".to_string(),
                [
                    self.conductor_cross_sect_area.to_string(),
                    other.conductor_cross_sect_area.to_string(),
                ],
            );
        }
        if self.overall_cross_sect_area != other.overall_cross_sect_area {
            input_map.insert(
                "Overall Cross Sectional Area".to_string(),
                [
                    self.overall_cross_sect_area.to_string(),
                    other.overall_cross_sect_area.to_string(),
                ],
            );
        }
        if self.stranded != other.stranded {
            input_map.insert(
                "Stranded".to_string(),
                [self.stranded.to_string(), other.stranded.to_string()],
            );
        }
        if self.num_strands != other.num_strands {
            input_map.insert(
                "Number of Strands".to_string(),
                [
                    {
                        if let Some(num_strands) = self.num_strands {
                            num_strands.to_string()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(num_strands) = other.num_strands {
                            num_strands.to_string()
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.strand_cross_sect_area != other.strand_cross_sect_area {
            input_map.insert(
                "Strand Cross Sectional Area".to_string(),
                [
                    {
                        if let Some(strand_cross_sect_area) = self.strand_cross_sect_area {
                            strand_cross_sect_area.to_string()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(strand_cross_sect_area) = other.strand_cross_sect_area {
                            strand_cross_sect_area.to_string()
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.insul_volt_rating != other.insul_volt_rating {
            input_map.insert(
                "Insulation Voltage Rating".to_string(),
                [
                    {
                        if let Some(insul_volt_rating) = self.insul_volt_rating {
                            insul_volt_rating.to_string()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(insul_volt_rating) = other.insul_volt_rating {
                            insul_volt_rating.to_string()
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.insul_temp_rating != other.insul_temp_rating {
            input_map.insert(
                "Insulation Temperature Rating".to_string(),
                [
                    {
                        if let Some(insul_temp_rating) = self.insul_temp_rating {
                            insul_temp_rating.to_string()
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(insul_temp_rating) = other.insul_temp_rating {
                            insul_temp_rating.to_string()
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }
        if self.insul_color != other.insul_color {
            input_map.insert(
                "Insulation Color".to_string(),
                [
                    {
                        if let Some(insul_color) = self.insul_color.clone() {
                            insul_color
                        } else {
                            String::new()
                        }
                    },
                    {
                        if let Some(insul_color) = other.insul_color.clone() {
                            insul_color
                        } else {
                            String::new()
                        }
                    },
                ],
            );
        }

        let results = prompt_fn(input_map);
        // false means don't replace value in self struct
        if results["Manufacturer"] {
            self.manufacturer = other.manufacturer.clone();
        }
        if results["Model"] {
            self.model = other.model.clone();
        }
        if results["Part Number"] {
            self.part_number = other.part_number.clone();
        }
        if results["Manufacturer Part Number"] {
            self.manufacturer_part_number = other.manufacturer_part_number.clone();
        }
        if results["Supplier"] {
            self.supplier = other.supplier.clone();
        }
        if results["Supplier Part Number"] {
            self.supplier_part_number = other.supplier_part_number.clone();
        }
        if results["Material"] {
            self.material = other.material.clone();
        }
        if results["Insulated"] {
            self.insulated = other.insulated;
        }
        if results["Insulation Material"] {
            self.insulation_material = other.insulation_material.clone();
        }
        if results["Wire Type Code"] {
            self.wire_type_code = other.wire_type_code.clone();
        }
        if results["Conductor Cross Sectional Area"] {
            self.conductor_cross_sect_area = other.conductor_cross_sect_area;
        }
        if results["Overall_Cross Sectional Area"] {
            self.overall_cross_sect_area = other.overall_cross_sect_area;
        }
        if results["Stranded"] {
            self.stranded = other.stranded;
        }
        if results["Number of Strands"] {
            self.num_strands = other.num_strands;
        }
        if results["Strand Cross Sectional Area"] {
            self.strand_cross_sect_area = other.strand_cross_sect_area;
        }
        if results["Insulation Voltage Rating"] {
            self.insul_volt_rating = other.insul_volt_rating;
        }
        if results["Insulation Temperature Rating"] {
            self.insul_temp_rating = other.insul_temp_rating;
        }
        if results["Insulation Color"] {
            self.insul_color = other.insul_color.clone();
        }
    }
}

impl Empty for WireType {
    fn is_empty(&self) -> bool {
        self == &Self::new()
    }
}

impl PartialEmpty for WireType {
    fn is_partial_empty(&self) -> bool {
        let tester = Self::new();
        self.manufacturer == tester.manufacturer
            && self.model == tester.model
            && self.part_number == tester.part_number
            && self.manufacturer_part_number == tester.manufacturer_part_number
            && self.supplier == tester.supplier
            && self.supplier_part_number == tester.supplier_part_number
            && self.material == tester.material
            && self.insulated == tester.insulated
            && self.insulation_material == tester.insulation_material
            && self.wire_type_code == tester.wire_type_code
            && self.conductor_cross_sect_area == tester.conductor_cross_sect_area
            && self.overall_cross_sect_area == tester.overall_cross_sect_area
            && self.stranded == tester.stranded
            && self.num_strands == tester.num_strands
            && self.strand_cross_sect_area == tester.strand_cross_sect_area
            && self.insul_volt_rating == tester.insul_volt_rating
            && self.insul_temp_rating == tester.insul_temp_rating
            && self.insul_color == tester.insul_color
    }
}

impl fmt::Display for WireType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Wire Type:")?;
        if let Some(manufacturer) = &self.manufacturer {
            writeln!(f, "Manufacturer: {manufacturer}")?;
        }
        if let Some(model) = &self.model {
            writeln!(f, "Model: {model}")?;
        }
        if let Some(part_number) = &self.part_number {
            writeln!(f, "Part Number: {part_number}")?;
        }
        if let Some(manufacturer_part_number) = &self.manufacturer_part_number {
            writeln!(f, "Manufacturer Part Number: {manufacturer_part_number}")?;
        }
        if let Some(supplier) = &self.supplier {
            writeln!(f, "Supplier: {supplier}")?;
        }
        if let Some(supplier_part_number) = &self.supplier_part_number {
            writeln!(f, "Supplier Part Number: {supplier_part_number}")?;
        }
        if let Some(material) = &self.material {
            writeln!(f, "Conductor Material: {material}")?;
        }
        writeln!(f, "Insulated: {}", &self.insulated)?;
        if let Some(insulation_material) = &self.insulation_material {
            writeln!(f, "Insulation Material: {insulation_material}")?;
        }
        if let Some(wire_type_code) = &self.wire_type_code {
            writeln!(f, "Wire Type Code: {wire_type_code}")?;
        }
        if f.alternate() {
            //TODO: implement mm^2 to AWG conversion, with auht and kcm display
            writeln!(
                f,
                "Conductor Cross Sectional Area: {} AWG",
                self.conductor_cross_sect_area
            )?;
        } else {
            writeln!(
                f,
                "Conductor Cross Sectional Area: {:.2} mm^2",
                self.conductor_cross_sect_area
            )?;
        }
        writeln!(
            f,
            "Overall Cross Sectional Area: {:.2} mm^2",
            self.overall_cross_sect_area
        )?;
        writeln!(f, "Stranded: {}", &self.stranded)?;
        if let Some(num_strands) = &self.num_strands {
            writeln!(f, "Number of Strands: {num_strands}")?;
        }
        if let Some(strand_cross_sect_area) = &self.strand_cross_sect_area {
            if f.alternate() {
                //TODO: implement mm^2 to AWG conversion, with auht and kcm display
                writeln!(
                    f,
                    "Strand Cross Sectional Area: {strand_cross_sect_area} AWG"
                )?;
            } else {
                writeln!(
                    f,
                    "Strand Cross Sectional Area: {strand_cross_sect_area:.2} mm^2"
                )?;
            }
        }
        if let Some(insul_volt_rating) = &self.insul_volt_rating {
            writeln!(f, "Insulation Voltage Rating: {insul_volt_rating}V")?;
        }
        if let Some(insul_temp_rating) = &self.insul_temp_rating {
            writeln!(f, "Insulation Temperature Rating: {insul_temp_rating}℃")?;
        }
        Ok(())
    }
}
