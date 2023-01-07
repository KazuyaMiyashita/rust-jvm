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

impl fmt::Display for AttributeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AttributeInfo {{\n")?;
        write!(f, "    attribute_name_index: {:#06x?},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        let info_vec: String = self.info.iter().map(|x| format!("{:#04x?}", x)).collect::<Vec<String>>().join(", ");
        write!(f, "    info: vec![{}],\n", info_vec)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for CodeAttributeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "CodeAttributeInfo {{\n")?;
        write!(f, "    attribute_name_index: {:#06x?},\n", self.attribute_name_index)?;
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
        write!(f, "}}")?;
        Ok(())
    }
}