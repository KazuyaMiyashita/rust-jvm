use crate::class_file::structure::attribute::*;
use crate::class_file::structure::attribute::code::*;
use super::padding;
use std::fmt;

impl fmt::Display for CodeAttributeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::Code(code::CodeAttributeInfo {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    max_stack: {},\n", self.max_stack)?;
        write!(f, "    max_locals: {},\n", self.max_locals)?;
        write!(f, "    code_length: {},\n", self.code_length)?;
        let code_vec: String = self.code.iter().map(|x| format!("{:#04x?}", x)).collect::<Vec<String>>().join(", ");
        write!(f, "    code: vec![{}],\n", code_vec)?;
        write!(f, "    exception_table_length: {},\n", self.exception_table_length)?;
        let exception_table_vec: String = self.exception_table.iter().map(|x| format!("{:#04x?}", x)).collect::<Vec<String>>().join(", ");
        write!(f, "    exception_table: vec![{}],\n", exception_table_vec)?;
        write!(f, "    attributes_count: {},\n", self.attributes_count)?;
        write!(f, "    attributes: vec![\n")?;
        self.attributes.iter().try_for_each(|attributes| {
            write!(f, "{},\n", padding(attributes.to_string(), 8))
        })?;
        write!(f, "    ]\n")?;
        write!(f, "}})")?;
        Ok(())
    }
}