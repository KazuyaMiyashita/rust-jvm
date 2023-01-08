use crate::class_file::structure::attribute::*;
use crate::class_file::structure::attribute::constant_value::*;
use std::fmt;

impl fmt::Display for ConstantValueAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::ConstantValue(constant_value::ConstantValueAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    constantvalue_index: {},\n", self.constantvalue_index)?;
        write!(f, "}})")?;
        Ok(())
    }
}
