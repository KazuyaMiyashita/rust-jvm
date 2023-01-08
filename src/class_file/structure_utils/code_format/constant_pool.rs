use crate::class_file::structure::constant_pool::*;
use std::fmt;

impl fmt::Display for CpInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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