use crate::class_file::structure::attribute::*;
use crate::class_file::structure::attribute::module::*;
use super::padding;
use std::fmt;



impl fmt::Display for ModuleAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::Module(module::ModuleAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    module_name_index: {},\n", self.module_name_index)?;
        write!(f, "    module_flags: {},\n", self.module_flags)?;
        write!(f, "    module_version_index: {},\n", self.module_version_index)?;
        write!(f, "    requires_count: {},\n", self.requires_count)?;
        write!(f, "    requires: vec![\n")?;
        self.requires.iter().try_for_each(|x| {
            write!(f, "{},\n", padding(x.to_string(), 8))
        })?;
        write!(f, "    ],\n")?;
        write!(f, "    exports_count: {},\n", self.exports_count)?;
        write!(f, "    exports: vec![\n")?;
        self.exports.iter().try_for_each(|x| {
            write!(f, "{},\n", padding(x.to_string(), 8))
        })?;
        write!(f, "    ],\n")?;
        write!(f, "    opens_count: {},\n", self.opens_count)?;
        write!(f, "    opens: vec![\n")?;
        self.opens.iter().try_for_each(|x| {
            write!(f, "{},\n", padding(x.to_string(), 8))
        })?;
        write!(f, "    ],\n")?;
        write!(f, "    uses_count: {},\n", self.uses_count)?;
        let uses_index_str: String = self.uses_index.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    uses_index: vec![{}],\n", uses_index_str)?;
        write!(f, "    provides_count: {},\n", self.provides_count)?;
        write!(f, "    provides: vec![\n")?;
        self.provides.iter().try_for_each(|x| {
            write!(f, "{},\n", padding(x.to_string(), 8))
        })?;
        write!(f, "    ],\n")?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for Require {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module::Require {{\n")?;
        write!(f, "    requires_index: {},\n", self.requires_index)?;
        write!(f, "    requires_flags: {},\n", self.requires_flags)?;
        write!(f, "    requires_version_index: {},\n", self.requires_version_index)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for Export {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module::Export {{\n")?;
        write!(f, "    exports_index: {},\n", self.exports_index)?;
        write!(f, "    exports_flags: {},\n", self.exports_flags)?;
        write!(f, "    exports_to_count: {},\n", self.exports_to_count)?;
        let exports_to_index_str: String = self.exports_to_index.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    exports_to_index: vec![{}],\n", exports_to_index_str)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for Open {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module::Open {{\n")?;
        write!(f, "    opens_index: {},\n", self.opens_index)?;
        write!(f, "    opens_flags: {},\n", self.opens_flags)?;
        write!(f, "    opens_to_count: {},\n", self.opens_to_count)?;
        let opens_to_index_str: String = self.opens_to_index.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    opens_to_index: vec![{}],\n", opens_to_index_str)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for Provide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module::Provide {{\n")?;
        write!(f, "    provides_index: {},\n", self.provides_index)?;
        write!(f, "    provides_with_count: {},\n", self.provides_with_count)?;
        let provides_with_index_str: String = self.provides_with_index.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    provides_with_index: vec![{}],\n", provides_with_index_str)?;
        write!(f, "}}")?;
        Ok(())
    }
}