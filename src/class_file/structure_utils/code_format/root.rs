use crate::class_file::structure::root::*;
use super::padding;
use std::fmt;

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl fmt::Display for FieldsInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
