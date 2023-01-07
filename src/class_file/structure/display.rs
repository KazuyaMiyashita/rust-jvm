use std::fmt;
use std::fmt::Formatter;
use crate::class_file::structure::*;

fn padding(str: String, n: usize) -> String {
    str.lines().map(|x| format!("{}{}", " ".repeat(n), x)).collect::<Vec<String>>().join("\n")
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // ClassFile {
        write!(f, "ClassFile {{\n")?;

        // magic: [0xca, 0xfe, 0xba, 0xbe],
        let magic_vec: String = self.magic.iter().map(|x| format!("{:#04x?}", x)).collect::<Vec<String>>().join(", ");
        write!(f, "    magic: [{}],\n", magic_vec)?;

        // minor_version: 0,
        write!(f, "    minor_version: {},\n", self.minor_version)?;

        // major_version: 61,
        write!(f, "    major_version: {},\n", self.major_version)?;

        // constant_pool_count 13,
        write!(f, "    constant_pool_count: {},\n", self.constant_pool_count)?;

        // constant_pool: vec![
        //     CpInfo::Methodref(ConstantMethodrefInfo { tag: 0x0a, class_index: 0x02, name_and_type_index: 0x03 }),
        //     CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x10, bytes: "java/lang/Object".as_bytes().to_vec() }),
        // ],
        write!(f, "    constant_pool: vec![\n")?;
        self.constant_pool.iter().try_for_each(|cp_info| {
            write!(f, "        {},\n", cp_info)
        })?;
        write!(f, "    ],\n")?;

        // access_flags: 0x0001,
        write!(f, "    access_flags: {:#06x?},\n", self.access_flags)?;

        // this_class: 1,
        write!(f, "    this_class: {},\n", self.this_class)?;

        // super_class: 2,
        write!(f, "    super_class: {},\n", self.super_class)?;

        // interfaces_count: 1,
        write!(f, "    interfaces_count: {},\n", self.interfaces_count)?;

        // interfaces: vec![],
        write!(f, "    interfaces: vec![],\n")?; // TODO

        // fields_count: 1,
        write!(f, "    fields_count: {},\n", self.fields_count)?;

        // fields: vec![],
        write!(f, "    fields: vec![\n")?;
        self.fields.iter().try_for_each(|field| {
            write!(f, "{},\n", padding(field.to_string(), 8))
        })?;
        write!(f, "    ],\n")?;


        // methods_count: 1,
        write!(f, "    methods_count: {},\n", self.methods_count)?;

        // methods_count: vec![],
        write!(f, "    methods: vec![\n")?; // TODO

        self.methods.iter().try_for_each(|method| {
            write!(f, "{},\n", padding(method.to_string(), 8))
        })?;
        write!(f, "    ],\n")?;

        // attributes_count: 1,
        write!(f, "    attributes_count: {},\n", self.attributes_count)?;

        // attributes: vec![],
        write!(f, "    attributes: vec![\n")?;
        self.attributes.iter().try_for_each(|attribute| {
            write!(f, "{},\n", padding(attribute.to_string(), 8))
        })?;
        write!(f, "    ],\n")?;

        write!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for CpInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = match self {
            CpInfo::Utf8(info) => {
                format!("CpInfo::Utf8(ConstantUtf8Info {{ tag: {}, length: {}, bytes: \"{}\".as_bytes().to_vec() }})",
                        info.tag, info.length, String::from_utf8(info.bytes.clone()).unwrap()
                )
            }
            CpInfo::Integer(info) => {
                format!("CpInfo::Integer(ConstantIntegerInfo {{ tag: {}, bytes: {}_i32.to_be_bytes() }})",
                        info.tag, i32::from_be_bytes(info.bytes)
                )
            }
            CpInfo::Float(info) => {
                format!("CpInfo::Float(ConstantFloatInfo {{ tag: {}, bytes: {}_f32.to_be_bytes() }})",
                        info.tag, f32::from_be_bytes(info.bytes)
                )
            }
            CpInfo::Long(info) => {
                format!("CpInfo::Long(ConstantLongInfo {{ tag: {}, high_bytes: {}_i64.to_be_bytes()[0..=3], low_bytes: {}_i64.to_be_bytes()[4..=7] }})",
                        info.tag,
                        i64::from_be_bytes([info.high_bytes, info.low_bytes].concat().try_into().unwrap()),
                        i64::from_be_bytes([info.high_bytes, info.low_bytes].concat().try_into().unwrap())
                )
            }
            CpInfo::Double(info) => {
                format!("CpInfo::Double(ConstantDoubleInfo {{ tag: {}, high_bytes: {}_f64.to_be_bytes()[0..=3], low_bytes: {}_f64.to_be_bytes()[4..=7] }})",
                        info.tag,
                        f64::from_be_bytes([info.high_bytes, info.low_bytes].concat().try_into().unwrap()),
                        f64::from_be_bytes([info.high_bytes, info.low_bytes].concat().try_into().unwrap())
                )
            }
            CpInfo::Class(info) => {
                format!("CpInfo::Class(ConstantClassInfo {{ tag: {}, name_index: {} }})",
                        info.tag, info.name_index
                )
            }
            CpInfo::String(info) => {
                format!("CpInfo::String(ConstantStringInfo {{ tag: {}, string_index: {} }})",
                        info.tag, info.string_index
                )
            }
            CpInfo::Fieldref(info) => {
                format!("CpInfo::Fieldref(ConstantFieldrefInfo {{ tag: {}, class_index: {}, name_and_type_index: {} }})",
                        info.tag, info.class_index, info.name_and_type_index
                )
            }
            CpInfo::Methodref(info) => {
                format!("CpInfo::Methodref(ConstantMethodrefInfo {{ tag: {}, class_index: {}, name_and_type_index: {} }})",
                        info.tag, info.class_index, info.name_and_type_index
                )
            }
            CpInfo::InterfaceMethodref(info) => {
                format!("CpInfo::InterfaceMethodref(ConstantInterfaceMethodrefInfo {{ tag: {}, class_index: {}, name_and_type_index: {} }})",
                        info.tag, info.class_index, info.name_and_type_index
                )
            }
            CpInfo::NameAndType(info) => {
                format!("CpInfo::NameAndType(ConstantNameAndTypeInfo {{ tag: {}, name_index: {}, descriptor_index: {} }})",
                        info.tag, info.name_index, info.descriptor_index
                )
            }
            CpInfo::MethodHandle(info) => {
                format!("CpInfo::MethodHandle(ConstantMethodHandleInfo {{ tag: {}, reference_kind: {}, reference_index: {} }})",
                        info.tag, info.reference_kind, info.reference_index
                )
            }
            CpInfo::MethodType(info) => {
                format!("CpInfo::MethodType(ConstantMethodTypeInfo {{ tag: {}, descriptor_index: {}, }})",
                        info.tag, info.descriptor_index
                )
            }
            CpInfo::Dynamic(info) => {
                format!("CpInfo::Dynamic(ConstantDynamicInfo {{ tag: {}, bootstrap_method_attr_index: {}, name_and_type_index: {} }})",
                        info.tag, info.bootstrap_method_attr_index, info.name_and_type_index
                )
            }
            CpInfo::InvokeDynamic(info) => {
                format!("CpInfo::InvokeDynamic(ConstantInvokeDynamicInfo {{ tag: {}, bootstrap_method_attr_index: {}, name_and_type_index: {} }})",
                        info.tag, info.bootstrap_method_attr_index, info.name_and_type_index
                )
            }
            CpInfo::Module(info) => {
                format!("CpInfo::Module(ConstantModuleInfo {{ tag: {}, name_index: {} }})",
                        info.tag, info.name_index
                )
            }
            CpInfo::Package(info) => {
                format!("CpInfo::Package(ConstantPackageInfo {{ tag: {}, name_index: {} }})",
                        info.tag, info.name_index
                )
            }
        };
        write!(f, "{}", str)
    }
}

impl fmt::Display for FieldsInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FieldsInfo {{\n")?;
        write!(f, "    access_flags: {:#06x?},\n", self.access_flags)?;
        write!(f, "    name_index: {},\n", self.name_index)?;
        write!(f, "    descriptor_index: {},\n", self.descriptor_index)?;
        write!(f, "    attributes_count: {},\n", self.attributes_count)?;
        write!(f, "    attributes: vec![\n")?;
        self.attributes.iter().try_for_each(|attributes| {
            write!(f, "{},\n", padding(attributes.to_string(), 8))
        })?;
        write!(f, "    ]\n")?;
        write!(f, "}}")?;

        Ok(())
    }
}

impl fmt::Display for MethodInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MethodInfo {{\n")?;
        write!(f, "    access_flags: {:#06x?},\n", self.access_flags)?;
        write!(f, "    name_index: {},\n", self.name_index)?;
        write!(f, "    descriptor_index: {},\n", self.descriptor_index)?;
        write!(f, "    attributes_count: {},\n", self.attributes_count)?;
        write!(f, "    attributes: vec![\n")?;
        self.attributes.iter().try_for_each(|attributes| {
            write!(f, "{},\n", padding(attributes.to_string(), 8))
        })?;
        write!(f, "    ]\n")?;
        write!(f, "}}")?;

        Ok(())
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::ConstantValue(ConstantValueAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    constantvalue_index: {},\n", self.constantvalue_index)?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for CodeAttributeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Parameter {{ name_index: {}, access_flags: {:#06x?} }},",
            self.name_index, self.access_flags)?;
        Ok(())
    }
}

impl fmt::Display for ModuleAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
        write!(f, "    ]\n")?;
        write!(f, "    exports_count: {},\n", self.exports_count)?;
        write!(f, "    exports: vec![\n")?;
        self.exports.iter().try_for_each(|x| {
            write!(f, "{},\n", padding(x.to_string(), 8))
        })?;
        write!(f, "    ]\n")?;
        write!(f, "    opens_count: {},\n", self.opens_count)?;
        write!(f, "    opens: vec![\n")?;
        self.opens.iter().try_for_each(|x| {
            write!(f, "{},\n", padding(x.to_string(), 8))
        })?;
        write!(f, "    ]\n")?;
        write!(f, "    uses_count: {},\n", self.uses_count)?;
        let uses_index_str: String = self.uses_index.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    uses_index: vec![{}]\n", uses_index_str)?;
        write!(f, "    provides_count: {},\n", self.provides_count)?;
        write!(f, "    provides: vec![\n")?;
        self.provides.iter().try_for_each(|x| {
            write!(f, "{},\n", padding(x.to_string(), 8))
        })?;
        write!(f, "    ]\n")?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for Require {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Require {{\n")?;
        write!(f, "    requires_index: {},\n", self.requires_index)?;
        write!(f, "    requires_flags: {},\n", self.requires_flags)?;
        write!(f, "    requires_to_count: {},\n", self.requires_to_count)?;
        let requires_to_index_str: String = self.requires_to_index.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    requires_to_index: vec![{}],\n", requires_to_index_str)?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for Export {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Export {{\n")?;
        write!(f, "    exports_index: {},\n", self.exports_index)?;
        write!(f, "    exports_flags: {},\n", self.exports_flags)?;
        write!(f, "    exports_to_count: {},\n", self.exports_to_count)?;
        let exports_to_index_str: String = self.exports_to_index.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    exports_to_index: vec![{}],\n", exports_to_index_str)?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for Open {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Open {{\n")?;
        write!(f, "    opens_index: {},\n", self.opens_index)?;
        write!(f, "    opens_flags: {},\n", self.opens_flags)?;
        write!(f, "    opens_to_count: {},\n", self.opens_to_count)?;
        let opens_to_index_str: String = self.opens_to_index.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    opens_to_index: vec![{}],\n", opens_to_index_str)?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl fmt::Display for Provide {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Provide {{\n")?;
        write!(f, "    provides_index: {},\n", self.provides_index)?;
        write!(f, "    provides_with_count: {},\n", self.provides_with_count)?;
        let provides_with_index_str: String = self.provides_with_index.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "    provides_with_index: vec![{}],\n", provides_with_index_str)?;
        write!(f, "}})")?;
        Ok(())
    }
}