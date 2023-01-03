use super::structure::*;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CheckError>;

#[derive(Error, Debug)]
#[error("Invalid class file. {message:}")]
pub struct CheckError {
    pub message: String,
}

// utils
fn error<T>(message: String) -> Result<T> {
    Err(CheckError { message })
}

//

// original constant_pool table is indexed from 1 to constant_pool_count - 1.
// Note that the Vec of this cp_infos structure is indexed from 0.
fn get_constant_pool_info(constant_pool: &Vec<CpInfo>, index: usize) -> Option<&CpInfo> {
    constant_pool.get(index - 1)
}

fn is_constant_utf8_info_entry(index: u16, constant_pool: &Vec<CpInfo>) -> Result<()> {
    match get_constant_pool_info(constant_pool, index as usize) {
        Some(CpInfo::ConstantUtf8Info { .. }) => Ok(()),
        Some(_) => error("This index must refer to CONSTANT_Utf8_info structure.".to_string()),
        _ => error("missing constant_pool entry.".to_string())
    }
}

fn is_constant_class_info_entry(index: u16, constant_pool: &Vec<CpInfo>) -> Result<()> {
    match get_constant_pool_info(constant_pool, index as usize) {
        Some(CpInfo::ConstantClassInfo { .. }) => Ok(()),
        Some(_) => error("This index must refer to CONSTANT_Utf8_info structure.".to_string()),
        _ => error("missing constant_pool entry.".to_string())
    }
}

fn is_name_and_type_info_entry(index: u16, constant_pool: &Vec<CpInfo>) -> Result<()> {
    match get_constant_pool_info(constant_pool, index as usize) {
        Some(CpInfo::ConstantNameAndTypeInfo { .. }) => Ok(()),
        Some(_) => error("This index must refer to CONSTANT_Utf8_info structure.".to_string()),
        _ => error("missing constant_pool entry.".to_string())
    }
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

pub fn check_constant_pool(constant_pool: &Vec<CpInfo>) -> Result<()> {
    for cp_info in constant_pool {
        match cp_info {
            CpInfo::ConstantUtf8Info { .. } => { todo!() }
            CpInfo::ConstantIntegerInfo { .. } => { todo!() }
            CpInfo::ConstantFloatInfo { .. } => { todo!() }
            CpInfo::ConstantLongInfo { .. } => { todo!() }
            CpInfo::ConstantDoubleInfo { .. } => { todo!() }
            CpInfo::ConstantClassInfo { tag, name_index } => {
                // 4.4.1. The CONSTANT_Class_info Structure
                // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.1
                // > tag
                // > The tag item has the value CONSTANT_Class (7).
                if *tag != CONSTANT_CLASS { return error("The tag item has the value CONSTANT_Class (7).".to_string()); };
                // > name_index
                // > The value of the name_index item must be a valid index into the constant_pool table.
                // > The constant_pool entry at that index must be a CONSTANT_Utf8_info structure (§4.4.7)
                is_constant_utf8_info_entry(*name_index, constant_pool)?;
            }
            CpInfo::ConstantStringInfo { .. } => { todo!() }
            CpInfo::ConstantFieldrefInfo { tag, class_index, name_and_type_index } => {
                // 4.4.2. The CONSTANT_Fieldref_info, CONSTANT_Methodref_info, and CONSTANT_InterfaceMethodref_info Structures
                // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.2

                // > tag
                // > The tag item of a CONSTANT_Fieldref_info structure has the value CONSTANT_Fieldref (9).
                if *tag != CONSTANT_FIELDREF { return error("The tag item of a CONSTANT_Fieldref_info structure has the value CONSTANT_Fieldref (9).".to_string()); }

                // > class_index
                // > The value of the class_index item must be a valid index into the constant_pool table.
                // > The constant_pool entry at that index must be a CONSTANT_Class_info structure (§4.4.1)
                // > representing a class or interface type that has the field or method as a member.
                is_constant_class_info_entry(*class_index, constant_pool)?;
                // > In a CONSTANT_Fieldref_info structure, the class_index item may be either a class type or an interface type.
                // QUESTION: Is there a way to confirm this?

                // > name_and_type_index
                // > The value of the name_and_type_index item must be a valid index into the constant_pool table.
                // > The constant_pool entry at that index must be a CONSTANT_NameAndType_info structure (§4.4.6).
                // > This constant_pool entry indicates the name and descriptor of the field or method.
                is_name_and_type_info_entry(*name_and_type_index, constant_pool)?;
                // > In a CONSTANT_Fieldref_info structure, the indicated descriptor must be a field descriptor (§4.3.2).
                match get_constant_pool_info(constant_pool, *name_and_type_index as usize) {
                    Some(CpInfo::ConstantNameAndTypeInfo { descriptor_index, .. }) => {
                        match get_constant_pool_info(constant_pool, *descriptor_index as usize) {
                            Some(CpInfo::ConstantUtf8Info { bytes, .. }) => {
                                let str = std::str::from_utf8(bytes).map_err(|e| CheckError { message: e.to_string() })?;
                                is_valid_field_descriptor(str.to_string())?
                            }
                            _ => return error("".to_string())
                        }
                    }
                    Some(_) => return error("the constant_pool entry of CONSTANT_Fieldref_info's name_and_type_index must be CONSTANT_NameAndType_info.".to_string()),
                    _ => return error("missing constant_pool entry of CONSTANT_Fieldref_info's name_and_type_index.".to_string()),
                }
                todo!()
            }
            CpInfo::ConstantMethodrefInfo { tag, class_index, name_and_type_index } => {
                // 4.4.2. The CONSTANT_Fieldref_info, CONSTANT_Methodref_info, and CONSTANT_InterfaceMethodref_info Structures
                // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.2

                // > tag
                // > The tag item of a CONSTANT_Methodref_info structure has the value CONSTANT_Methodref (10).
                if *tag != CONSTANT_METHODREF { return error("The tag item of a CONSTANT_Methodref_info structure has the value CONSTANT_Methodref (10).".to_string()); }

                // > class_index
                // > The value of the class_index item must be a valid index into the constant_pool table.
                // > The constant_pool entry at that index must be a CONSTANT_Class_info structure (§4.4.1)
                // > representing a class or interface type that has the field or method as a member.
                is_constant_class_info_entry(*class_index, constant_pool)?;
                // > In a CONSTANT_Methodref_info structure, the class_index item must be a class type, not an interface type.
                // QUESTION: Is there a way to confirm this?

                // > name_and_type_index
                // > The value of the name_and_type_index item must be a valid index into the constant_pool table.
                // > The constant_pool entry at that index must be a CONSTANT_NameAndType_info structure (§4.4.6).
                // > This constant_pool entry indicates the name and descriptor of the field or method.
                is_name_and_type_info_entry(*name_and_type_index, constant_pool)?;
                // > If the name of the method in a CONSTANT_Methodref_info structure begins with a '<' ('\u003c'),
                // > then the name must be the special name <init>, representing an instance initialization method (§2.9.1).
                // > The return type of such a method must be void.

                todo!()
            }
            CpInfo::ConstantInterfaceMethodrefInfo { .. } => { todo!() }
            CpInfo::ConstantNameAndTypeInfo { .. } => { todo!() }
            CpInfo::ConstantMethodHandleInfo { .. } => { todo!() }
            CpInfo::ConstantMethodTypeInfo { .. } => { todo!() }
            CpInfo::ConstantDynamicInfo { .. } => { todo!() }
            CpInfo::ConstantInvokeDynamicInfo { .. } => { todo!() }
            CpInfo::ConstantModuleInfo { .. } => { todo!() }
            CpInfo::ConstantPackageInfo { .. } => {}
        }
    }
    Ok(())
}

// 4.2.1. Binary Class and Interface Names
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.2.1
#[allow(unused_variables)]
fn is_valid_internal_class_name(bytes: String) -> Result<()> {
    // println!("class name: {}", bytes);
    // todo!()
    Ok(())
}

// 4.3.2. Field Descriptors
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.3.2
fn is_valid_field_descriptor(bytes: String) -> Result<()> {
    let (head, tail) = bytes.split_at(1);
    match head {
        "B" | "C" | "D" | "F" | "I" | "J" | "S" | "Z" => Ok(()),
        "L" => {
            // Excluding the last character `;`, check if it's the internal form of the class name.
            tail.to_string().pop();
            let (class_name, last) = tail.split_at(tail.len() - 1);
            is_valid_internal_class_name(class_name.to_string())?;
            if last != ";" { return error("ObjectType of field descriptor must end with `;`".to_string()); }
            Ok(())
        }
        "[" => is_valid_field_descriptor(tail.to_string()),
        _ => return error("Invalid field descriptor.".to_string())
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
    let version = check_version(class_file.minor_version, class_file.major_version)?;

    let constant_pool = check_constant_pool(&class_file.constant_pool)?;

    check_attributes(&class_file.attributes, &class_file.constant_pool)?;
    check_fields(&class_file.fields, &class_file.constant_pool)?;
    check_methods(&class_file.methods, &class_file.constant_pool)?;

    // All predefined attributes (§4.7) must be of the proper length, except for StackMapTable,
    // RuntimeVisibleAnnotations, RuntimeInvisibleAnnotations, RuntimeVisibleParameterAnnotations,
    // RuntimeInvisibleParameterAnnotations, RuntimeVisibleTypeAnnotations, RuntimeInvisibleTypeAnnotations, and AnnotationDefault.
    // - TODO

    // The class file must not be truncated or have extra bytes at the end.
    // - This is done with Reader::<ClassFile>

    // -- ConstantFieldrefInfo, ConstantMethodrefInfo, ConstantInterfaceMethodrefInfo に関して
    // class_index は constant_pool table の有効なインデックスである必要がある。


    // QUESTION:
    // さらに、 ConstantFieldrefInfo の場合、その class_index が指し示すものはクラス型またはインターフェイス型、
    // ConstantMethodrefInfo の場合、その class_index が指し示すものはクラス型でありインターフェース型ではない、
    // ConstantInterfaceMethodrefInfo の場合、 class_index が指し示すものはインターフェース型でありクラス型ではないとあるが、
    // これは検証する必要があるのだろうか？(検証が可能なのだろうか？)
    // ConstantClassInfo はクラスまたはインターフェースを表すために利用されるものだが、
    // クラスかインターフェースかを区別する識別子は ConstantClassInfo の中にはない。
    //
    // In a CONSTANT_Fieldref_info structure, the class_index item may be either a class type or an interface type.
    // In a CONSTANT_Methodref_info structure, the class_index item must be a class type, not an interface type.
    // In a CONSTANT_InterfaceMethodref_info structure, the class_index item must be an interface type, not a class type.
    // cf. https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.2

    // All field references and method references in the constant pool must have valid names, valid classes, and valid descriptors (§4.3).
    // -- TODO..

    Ok(())
}