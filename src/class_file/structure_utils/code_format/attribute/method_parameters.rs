use crate::class_file::structure::attribute::*;
use crate::class_file::structure::attribute::method_parameters::*;
use super::padding;
use std::fmt;


impl fmt::Display for MethodParametersAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::MethodParameters(method_parameters::MethodParametersAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    parameters_count: {},\n", self.parameters_count)?;
        write!(f, "    parameters: vec![\n")?;
        self.parameters.iter().try_for_each(|parameter| {
            write!(f, "{},\n", padding(parameter.to_string(), 8))
        })?;
        write!(f, "    ]\n")?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "method_parameters::Parameter {{ name_index: {}, access_flags: {:#06x?} }},",
               self.name_index, self.access_flags)?;
        Ok(())
    }
}