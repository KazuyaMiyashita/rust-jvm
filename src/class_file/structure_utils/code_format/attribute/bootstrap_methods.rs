use crate::class_file::structure::attribute::*;
use crate::class_file::structure::attribute::bootstrap_methods::*;
use super::padding;
use std::fmt;

impl fmt::Display for BootstrapMethodsAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::BootstrapMethods(bootstrap_methods::BootstrapMethodsAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    num_bootstrap_methods: {},\n", self.num_bootstrap_methods)?;
        write!(f, "    bootstrap_methods: vec![\n")?;
        self.bootstrap_methods.iter().try_for_each(|bootstrap_method| {
            write!(f, "{},\n", padding(bootstrap_method.to_string(), 8))
        })?;
        write!(f, "    ]\n")?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for BootstrapMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bootstrap_methods::BootstrapMethod {{\n")?;
        write!(f, "    bootstrap_method_ref: {},\n", self.bootstrap_method_ref)?;
        write!(f, "    num_bootstrap_arguments: {},\n", self.num_bootstrap_arguments)?;
        let bootstrap_arguments_str: String = self.bootstrap_arguments.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    bootstrap_arguments: vec![{}],\n", bootstrap_arguments_str)?;
        write!(f, "}})")?;
        Ok(())
    }
}
