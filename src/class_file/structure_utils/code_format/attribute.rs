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
            Attribute::NestHost(attribute) => attribute.fmt(f),
            Attribute::NestMembers(attribute) => attribute.fmt(f),
            Attribute::PermittedSubclasses(attribute) => attribute.fmt(f)
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

impl fmt::Display for ConstantValueAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::ConstantValue(ConstantValueAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    constantvalue_index: {},\n", self.constantvalue_index)?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for CodeAttributeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::Code(CodeAttributeInfo {{\n")?;
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

impl fmt::Display for StackMapTableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::Code(StackMapTableAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    number_of_entries: {},\n", self.number_of_entries)?;
        write!(f, "    entries: vec![\n")?;
        self.entries.iter().try_for_each(|entries| {
            write!(f, "{},\n", padding(entries.to_string(), 8))
        })?;
        write!(f, "    ]\n")?;
        write!(f, "}}")?;

        Ok(())
    }
}

impl fmt::Display for StackMapFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            StackMapFrame::SameFrame { frame_type } => {
                format!("StackMapFrame::SameFrame {{ frame_type: {} }}", frame_type)
            }
            StackMapFrame::SameLocals1StackItemFrame { frame_type, stack } => {
                let stack_str: String = stack.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                format!("StackMapFrame::SameLocals1StackItemFrame {{ frame_type: {}, stack: vec![{}] }}", frame_type, stack_str)
            }
            StackMapFrame::SameLocals1StackItemFrameExtended { frame_type, offset_delta, stack } => {
                let stack_str: String = stack.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                format!("StackMapFrame::SameLocals1StackItemFrameExtended {{ frame_type: {}, offset_delta: {}, stack: vec![{}] }}", frame_type, offset_delta, stack_str)
            }
            StackMapFrame::ChopFrame { frame_type, offset_delta } => {
                format!("StackMapFrame::ChopFrame {{ frame_type: {}, offset_delta: {} }}", frame_type, offset_delta)
            }
            StackMapFrame::SameFrameExtended { frame_type, offset_delta } => {
                format!("StackMapFrame::SameFrameExtended {{ frame_type: {}, offset_delta: {} }}", frame_type, offset_delta)
            }
            StackMapFrame::AppendFrame { frame_type, offset_delta, locals } => {
                let locals_str: String = locals.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                format!("StackMapFrame::AppendFrame {{ frame_type: {}, offset_delta: {}, locals: vec![{}] }}", frame_type, offset_delta, locals_str)
            }
            StackMapFrame::FullFrame { frame_type, offset_delta, number_of_locals, locals, number_of_stack_items, stack } => {
                let locals_str: String = locals.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                let stack_str: String = stack.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                format!("StackMapFrame::FullFrame {{ frame_type: {}, offset_delta: {}, number_of_locals: {}, locals: vec![{}], number_of_stack_items: {}, stack_str: vec![{}] }}", frame_type, offset_delta, number_of_locals, locals_str, number_of_stack_items, stack_str)
            }
        };
        write!(f, "{}", str)
    }
}

impl fmt::Display for VerificationTypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            VerificationTypeInfo::TopVariableInfo { tag } => {
                format!("VerificationTypeInfo::TopVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::IntegerVariableInfo { tag } => {
                format!("VerificationTypeInfo::IntegerVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::FloatVariableInfo { tag } => {
                format!("VerificationTypeInfo::FloatVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::DoubleVariableInfo { tag } => {
                format!("VerificationTypeInfo::DoubleVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::LongVariableInfo { tag } => {
                format!("VerificationTypeInfo::LongVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::NullVariableInfo { tag } => {
                format!("VerificationTypeInfo::NullVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::UninitializedThisVariableInfo { tag } => {
                format!("VerificationTypeInfo::UninitializedThisVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::ObjectVariableInfo { tag, cpool_index } => {
                format!("VerificationTypeInfo::ObjectVariableInfo {{ tag: {}, cpool_index: {} }}", tag, cpool_index)
            }
            VerificationTypeInfo::UninitializedVariableInfo { tag, offset } => {
                format!("VerificationTypeInfo::UninitializedVariableInfo {{ tag: {}, offset: {} }}", tag, offset)
            }
        };
        write!(f, "{}", str)
    }
}

impl fmt::Display for BootstrapMethodsAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::BootstrapMethods(BootstrapMethodsAttribute {{\n")?;
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
        write!(f, "BootstrapMethod {{\n")?;
        write!(f, "    bootstrap_method_ref: {},\n", self.bootstrap_method_ref)?;
        write!(f, "    num_bootstrap_arguments: {},\n", self.num_bootstrap_arguments)?;
        let bootstrap_arguments_str: String = self.bootstrap_arguments.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    bootstrap_arguments: vec![{}],\n", bootstrap_arguments_str)?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for MethodParametersAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::MethodParameters(MethodParametersAttribute {{\n")?;
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
        write!(f, "Parameter {{ name_index: {}, access_flags: {:#06x?} }},",
               self.name_index, self.access_flags)?;
        Ok(())
    }
}

impl fmt::Display for ModuleAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::Module(ModuleAttribute {{\n")?;
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
        write!(f, "Require {{\n")?;
        write!(f, "    requires_index: {},\n", self.requires_index)?;
        write!(f, "    requires_flags: {},\n", self.requires_flags)?;
        write!(f, "    requires_version_index: {},\n", self.requires_version_index)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for Export {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Export {{\n")?;
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
        write!(f, "Open {{\n")?;
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
        write!(f, "Provide {{\n")?;
        write!(f, "    provides_index: {},\n", self.provides_index)?;
        write!(f, "    provides_with_count: {},\n", self.provides_with_count)?;
        let provides_with_index_str: String = self.provides_with_index.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    provides_with_index: vec![{}],\n", provides_with_index_str)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for NestHostAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::NestHost(NestHostAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    host_class_index: {},\n", self.host_class_index)?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for NestMembersAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::NestMembers(NestMembersAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    number_of_classes: {},\n", self.number_of_classes)?;
        write!(f, "    classes: vec![{}],\n", self.classes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for PermittedSubclassesAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::PermittedSubclasses(PermittedSubclassesAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    number_of_classes: {},\n", self.number_of_classes)?;
        write!(f, "    classes: vec![{}],\n", self.classes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "}})")?;
        Ok(())
    }
}
