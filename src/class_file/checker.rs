use crate::class_file::error::{Error, Result};
use crate::class_file::structure::*;
use crate::class_file::structure::cp_accessor::*;

#[allow(unused)]
use super::descriptor::{MethodType, ReturnType, parse_field_type, parse_method_descriptor};

// utils
fn error<T>(message: String) -> Result<T> {
    Err(Error { message: format!("Class checking failed. {}", message) })
}

// checkers

pub fn check_magic(magic: &[u8; 4]) -> Result<()> {
    match magic {
        [0xca, 0xfe, 0xba, 0xbe] => Ok(()),
        _ => error("This is not a class file. The first byte array must be `cafebabe`".to_string())
    }
}

pub fn check_version(minor_version: u16, major_version: u16) -> Result<()> {
    match (major_version, minor_version) {
        (56..=61, 0 | 65535) => (),
        (56..=61, _) => return error(format!("invalid class file minor version.\
                The version of this input is major: {}, minor: {}.", major_version, minor_version))?,
        (45..=61, _) => (),
        _ => return error(format!(
            "Not supported class file version. \
                The version of this input is major: {}, minor: {}.\
                This JVM is version 17. Class file major versions 45 upto 61 are supported.", major_version, minor_version))?
    };
    Ok(())
}


fn check_constant_pool(constant_pool: &Vec<CpInfo>, major_version: u16) -> Result<()> {
    for i in 0..constant_pool.len() {
        let cp_index = (i + 1) as u16;
        match constant_pool[i] {
            CpInfo::Utf8(_) => {
                constant_pool.access_as_utf8(cp_index).bytes_as_string()?;
            }
            CpInfo::Integer(_) => {
                constant_pool.access_as_integer(cp_index).bytes_as_integer()?;
            }
            CpInfo::Float(_) => {
                constant_pool.access_as_float(cp_index).bytes_as_float()?;
            }
            CpInfo::Long(_) => {
                constant_pool.access_as_long(cp_index).bytes_as_long()?;
            }
            CpInfo::Double(_) => {
                constant_pool.access_as_double(cp_index).bytes_as_double()?;
            }
            CpInfo::Class(_) => {
                constant_pool.access_as_class(cp_index).name().info_or_err?;
            }
            CpInfo::String(_) => {
                constant_pool.access_as_string(cp_index).name().info_or_err?;
            }
            CpInfo::Fieldref(_) => {
                constant_pool.access_as_fieldref(cp_index).class().info_or_err?;
                let descriptor = constant_pool.access_as_fieldref(cp_index).name_and_type().descriptor().bytes_as_string()?;
                parse_field_type(&descriptor)?;
            }
            CpInfo::Methodref(_) => {
                constant_pool.access_as_methodref(cp_index).class().info_or_err?;
                let name = constant_pool.access_as_methodref(cp_index).name_and_type().name().bytes_as_string()?;
                let descriptor = constant_pool.access_as_methodref(cp_index).name_and_type().descriptor().bytes_as_string()?;
                if name.starts_with('<') {
                    if name != "<init>" { return error("A special method name <init> is expected, but not.".to_string()); }
                    match parse_method_descriptor(&descriptor) {
                        Ok(MethodType { return_type, .. }) if return_type == ReturnType::Void => (),
                        Ok(_) => return error("return type of <init> must be void.".to_string()),
                        Err(e) => return Err(e)
                    }
                };
            }
            CpInfo::InterfaceMethodref(_) => {
                constant_pool.access_as_interface_methodref(cp_index).class().info_or_err?;
                let descriptor = constant_pool.access_as_methodref(cp_index).name_and_type().descriptor().bytes_as_string()?;
                parse_field_type(&descriptor)?;
            }
            CpInfo::NameAndType(_) => {
                constant_pool.access_as_name_and_type(cp_index).name().info_or_err?;
                constant_pool.access_as_name_and_type(cp_index).descriptor().info_or_err?;
            }
            CpInfo::MethodHandle(_) => {
                let method_handle_accessor = constant_pool.access_as_method_handle(cp_index);
                let reference_kind = method_handle_accessor.reference_kind()?;
                if !(1 <= reference_kind && reference_kind <= 9) {
                    return error(format!("The reference_kind of CONSTANT_MethodHandle_info must be in the range 1 to 9. reference_kind: {}", reference_kind));
                }
                let reference = method_handle_accessor.reference()?;
                match (reference_kind, major_version) {
                    (1..=4, _) => match reference {
                        MethodHandleReference::Fieldref(_) => (),
                        MethodHandleReference::Methodref(_) => return error("When reference_kind is in the range 1 to 4, the constant_pool entry at the reference_index must be CONSTANT_Fieldref_info, but CONSTANT_Methodref_info found!".to_string()),
                        MethodHandleReference::InterfaceMethodref(_) => return error("When reference_kind is in the range 1 to 4, the constant_pool entry at the reference_index must be CONSTANT_Fieldref_info, but CONSTANT_InterfaceMethodref_info found!".to_string())
                    },
                    (5 | 8, _) => match reference {
                        MethodHandleReference::Fieldref(_) => return error("When reference_kind is 5 or 8, the constant_pool entry at the reference_index must be CONSTANT_Methodref_info, but CONSTANT_Fieldref_info found!".to_string()),
                        MethodHandleReference::Methodref(_) => (),
                        MethodHandleReference::InterfaceMethodref(_) => return error("When reference_kind is 5 or 8, the constant_pool entry at the reference_index must be CONSTANT_Methodref_info, but CONSTANT_InterfaceMethodref_info found!".to_string()),
                    },
                    (6 | 7, v) if v < 52 => match reference {
                        MethodHandleReference::Fieldref(_) => return error("When reference_kind is 6 or 7 and version is less than 52.0, the constant_pool entry at the reference_index must be CONSTANT_Methodref_info, but CONSTANT_Fieldref_info found!".to_string()),
                        MethodHandleReference::Methodref(_) => (),
                        MethodHandleReference::InterfaceMethodref(_) => return error("When reference_kind is 6 or 7 and version is less than 52.0, the constant_pool entry at the reference_index must be CONSTANT_Methodref_info, but CONSTANT_InterfaceMethodref_info found!".to_string()),
                    },
                    (6 | 7, v) if v >= 52 => match reference {
                        MethodHandleReference::Fieldref(_) => return error("When reference_kind is 6 or 7 and version is 52.0 or above, the constant_pool entry at the reference_index must be CONSTANT_Methodref_info or CONSTANT_InterfaceMethodref_info, but CONSTANT_Fieldref_info found!".to_string()),
                        MethodHandleReference::Methodref(_) => (),
                        MethodHandleReference::InterfaceMethodref(_) => (),
                    },
                    (9, _) => match reference {
                        MethodHandleReference::Fieldref(_) => return error("When reference_kind is 9, the constant_pool entry at the reference_index must be CONSTANT_InterfaceMethodref_info, but CONSTANT_Fieldref_info found!".to_string()),
                        MethodHandleReference::Methodref(_) => return error("When reference_kind is 9, the constant_pool entry at the reference_index must be CONSTANT_InterfaceMethodref_info, but CONSTANT_Methodref_info found!".to_string()),
                        MethodHandleReference::InterfaceMethodref(_) => (),
                    },
                    _ => return error(format!("The reference_kind of CONSTANT_MethodHandle_info must be in the range 1 to 9. reference_kind: {}", reference_kind))
                }
                match reference_kind {
                    5 | 6 | 7 | 9 => {
                        let method_name = match reference {
                            MethodHandleReference::Fieldref(_) => panic!(),
                            MethodHandleReference::Methodref(accessor) => accessor.name_and_type().name().bytes_as_string()?,
                            MethodHandleReference::InterfaceMethodref(accessor) => accessor.name_and_type().name().bytes_as_string()?,
                        };
                        if method_name == "<init>" || method_name == "<clinit>" {
                            return error(format!("When reference_kind is 5, 6, 7 or 9, the name of the method must not be <init> or <clinit>. name: {}", method_name));
                        }
                    }
                    8 => {
                        let method_name = match reference {
                            MethodHandleReference::Methodref(accessor) => accessor.name_and_type().name().bytes_as_string()?,
                            _ => panic!()
                        };
                        if method_name != "<init>" {
                            return error(format!("When reference_kind is 8, the name of the method must be <init>. name: {}", method_name));
                        }
                    }
                    _ => ()
                }
            }
            CpInfo::MethodType(_) => {
                let descriptor = constant_pool.access_as_method_type(cp_index).descriptor().bytes_as_string()?;
                parse_method_descriptor(&descriptor)?;
            }
            CpInfo::Dynamic(_) => {
                let accessor = constant_pool.access_as_dynamic(cp_index);
                // The value of the bootstrap_method_attr_index item must be a valid index into the bootstrap_methods array of the bootstrap method table of this class file (§4.7.23).
                let _bootstrap_method_attr_index = accessor.get_bootstrap_method_attr_index()?;
                // don't check here

                let name = accessor.name_and_type().name().bytes_as_string()?;
                parse_field_type(&name)?;
                let descriptor = accessor.name_and_type().descriptor().bytes_as_string()?;
                parse_method_descriptor(&descriptor)?;
            }
            CpInfo::InvokeDynamic(_) => {
                let accessor = constant_pool.access_as_invoke_dynamic(cp_index);
                // The value of the bootstrap_method_attr_index item must be a valid index into the bootstrap_methods array of the bootstrap method table of this class file (§4.7.23).
                let _bootstrap_method_attr_index = accessor.get_bootstrap_method_attr_index()?;
                // don't check here

                let name = accessor.name_and_type().name().bytes_as_string()?;
                parse_field_type(&name)?;
                let descriptor = accessor.name_and_type().descriptor().bytes_as_string()?;
                parse_method_descriptor(&descriptor)?;
            }
            CpInfo::Module(_) => {
                let _module_name = constant_pool.access_as_module(cp_index).name().bytes_as_string()?;
                // TODO: valid module name
            }
            CpInfo::Package(_) => {
                let _package_name = constant_pool.access_as_package(cp_index).name().bytes_as_string()?;
                // TODO: check valid package name
            }
        }
    }
    Ok(())
}


// original constant_pool table is indexed from 1 to constant_pool_count - 1.
// Note that the Vec of this cp_infos structure is indexed from 0.
fn get_constant_pool_info(constant_pool: &Vec<CpInfo>, index: usize) -> Option<&CpInfo> {
    constant_pool.get(index - 1)
}

fn is_constant_utf8_info_entry(index: u16, constant_pool: &Vec<CpInfo>) -> Result<()> {
    match get_constant_pool_info(constant_pool, index as usize) {
        Some(CpInfo::Utf8(..)) => Ok(()),
        Some(_) => error("This index must refer to CONSTANT_Utf8_info structure.".to_string()),
        _ => error("missing constant_pool entry.".to_string())
    }
}

fn check_fields(fields: &Vec<FieldsInfo>, constant_pool: &Vec<CpInfo>) -> Result<()> {
    fields.iter().try_for_each(|field| {
        is_constant_utf8_info_entry(field.name_index, constant_pool)
    })
}

fn check_methods(methods: &Vec<MethodInfo>, constant_pool: &Vec<CpInfo>) -> Result<()> {
    methods.iter().try_for_each(|method_info| {
        is_constant_utf8_info_entry(method_info.name_index, constant_pool)?;
        method_info.attributes.iter().try_for_each(|attribute| {
            is_constant_utf8_info_entry(attribute.attribute_name_index, constant_pool)
        })
    })
}

fn check_attributes(attributes: &Vec<AttributeInfo>, constant_pool: &Vec<CpInfo>) -> Result<()> {
    attributes.iter().try_for_each(|attribute| {
        is_constant_utf8_info_entry(attribute.attribute_name_index, constant_pool)
    })
}


// 4.8. Format Checking
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.8
pub fn check_class_file(class_file: &ClassFile) -> Result<()> {
    let _version = check_version(class_file.minor_version, class_file.major_version)?;

    check_constant_pool(&class_file.constant_pool, class_file.major_version)?;

    check_attributes(&class_file.attributes, &class_file.constant_pool)?;
    check_fields(&class_file.fields, &class_file.constant_pool)?;
    check_methods(&class_file.methods, &class_file.constant_pool)?;

    Ok(())
}
