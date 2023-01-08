mod bootstrap_methods;
mod const_value;
mod stack_map_table;
mod method_parameters;
mod code;
mod module;

use crate::class_file::structure::attribute::*;
use super::padding;
use std::fmt;


impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Attribute::General(attribute) => attribute.fmt(f),
            Attribute::ConstantValue(attribute) => attribute.fmt(f),
            Attribute::Code(attribute) => attribute.fmt(f),
            Attribute::StackMapTable(attribute) => attribute.fmt(f),
            Attribute::BootstrapMethods(attribute) => attribute.fmt(f),
            Attribute::MethodParameters(attribute) => attribute.fmt(f),
            Attribute::Module(attribute) => attribute.fmt(f),
        }
    }
}

impl fmt::Display for AttributeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::General(AttributeInfo {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        let info_vec: String = self.info.iter().map(|x| format!("{:#04x?}", x)).collect::<Vec<String>>().join(", ");
        write!(f, "    info: vec![{}],\n", info_vec)?;
        write!(f, "}})")?;
        Ok(())
    }
}
