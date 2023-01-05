use super::structure::*;
#[allow(unused)]
use super::descriptor::{MethodType, ReturnType, parse_field_type, parse_method_descriptor};
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
        Some(CpInfo::Utf8(..)) => Ok(()),
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

trait ConstantMethodrefInfoResolver {
    fn resolve_class_info<'a>(&'a self, constant_pool: &'a Vec<CpInfo>) -> Option<&ConstantClassInfo>;
}

impl ConstantMethodrefInfoResolver for ConstantMethodrefInfo {
    fn resolve_class_info<'a>(&'a self, constant_pool: &'a Vec<CpInfo>) -> Option<&ConstantClassInfo> {
        match get_constant_pool_info(constant_pool, self.class_index as usize) {
            Some(CpInfo::Class(info)) => Some(info),
            _ => None
        }
    }
}

trait GetUtf8String {
    fn get_utf8_string(&self) -> String;
}
impl GetUtf8String for ConstantUtf8Info {
    fn get_utf8_string(&self) -> String {
        String::from_utf8(self.bytes.clone()).unwrap()
    }
}

fn check_constant_class_info(info: &ConstantClassInfo, constant_pool: &Vec<CpInfo>, is_validated: Vec<bool>) -> Result<Vec<bool>> {
    if info.tag != CONSTANT_CLASS { return error("The tag item has the value CONSTANT_Class (7).".to_string()); };
    let (name, is_validated) = check_each_cp(info.name_index as usize, constant_pool, is_validated)?;
    match name {
        CpInfo::Utf8(..) => (),
        _ => return error("The name_index must refer to CONSTANT_Utf8_info structure.".to_string())
    }
    Ok(is_validated)
}

fn check_constant_string_info(info: &ConstantStringInfo, constant_pool: &Vec<CpInfo>, is_validated: Vec<bool>) -> Result<Vec<bool>> {
    if info.tag != CONSTANT_STRING { return error("The tag item has the value CONSTANT_String (8).".to_string()); }
    let (string, is_validated) = check_each_cp(info.string_index as usize, constant_pool, is_validated)?;
    match string {
        CpInfo::Utf8(..) => (),
        _ => return error("The string_index must refer to CONSTANT_Utf8_info structure.".to_string())?
    }
    Ok(is_validated)
}

fn check_constant_fieldref_info(info: &ConstantFieldrefInfo, constant_pool: &Vec<CpInfo>, is_validated: Vec<bool>) -> Result<Vec<bool>> {
    if info.tag != CONSTANT_FIELDREF { return error("The tag item of a CONSTANT_Fieldref_info structure has the value CONSTANT_Fieldref (9).".to_string()); }
    let (_, is_validated): (&ConstantClassInfo, _) = match check_each_cp(info.class_index as usize, constant_pool, is_validated)? {
        (CpInfo::Class(info), is_validated) => (info, is_validated),
        _ => return error("The class_index must refer to CONSTANT_Class_info structure.".to_string())
    };
    let (name_and_type_info, is_validated): (&ConstantNameAndTypeInfo,_)  = match check_each_cp(info.name_and_type_index as usize, constant_pool, is_validated)? {
        (CpInfo::NameAndType(info), is_validated) => (info, is_validated),
        _ => return error("The class_index must refer to CONSTANT_NameAndType_info structure.".to_string())
    };
    let (descriptor_info, is_validated): (&ConstantUtf8Info, _) = match check_each_cp(name_and_type_info.descriptor_index as usize, constant_pool, is_validated)? {
        (CpInfo::Utf8(info), is_validated) => (info, is_validated),
        _ => return error("The descriptor_index must refer to CONSTANT_NameAndType_info structure.".to_string())
    };
    let descriptor = descriptor_info.get_utf8_string();
    parse_field_type(&descriptor).or_else(|e| error(e))?;
    Ok(is_validated)
}

fn check_constant_methodref_info(info: &ConstantMethodrefInfo, constant_pool: &Vec<CpInfo>, is_validated: Vec<bool>) -> Result<Vec<bool>> {
    if info.tag != CONSTANT_METHODREF { return error("The tag item of a CONSTANT_Methodref_info structure has the value CONSTANT_Methodref (10).".to_string()); }

    let (_, is_validated): (&ConstantClassInfo, _) = match check_each_cp(info.class_index as usize, constant_pool, is_validated)? {
        (CpInfo::Class(info), is_validated) => (info, is_validated),
        _ => return error("The class_index must refer to CONSTANT_Class_info structure.".to_string())
    };
    let (name_and_type_info, is_validated): (&ConstantNameAndTypeInfo,_)  = match check_each_cp(info.name_and_type_index as usize, constant_pool, is_validated)? {
        (CpInfo::NameAndType(info), is_validated) => (info, is_validated),
        _ => return error("The class_index must refer to CONSTANT_NameAndType_info structure.".to_string())
    };

    let (name_info, is_validated): (&ConstantUtf8Info, _) = match check_each_cp(name_and_type_info.name_index as usize, constant_pool, is_validated)? {
        (CpInfo::Utf8(info), is_validated) => (info, is_validated),
        _ => return error("The class_index must refer to CONSTANT_NameAndType_info structure.".to_string())
    };

    let (descriptor_info, is_validated): (&ConstantUtf8Info, _) = match check_each_cp(name_and_type_info.descriptor_index as usize, constant_pool, is_validated)? {
        (CpInfo::Utf8(info), is_validated) => (info, is_validated),
        _ => return error("The descriptor_index must refer to CONSTANT_NameAndType_info structure.".to_string())
    };

    let name = name_info.get_utf8_string();
    let descriptor = descriptor_info.get_utf8_string();

    if name.starts_with('<') {
        if name != "<init>" { return error("A special method name <init> is expected, but not.".to_string()); }
        match parse_method_descriptor(&descriptor) {
            Ok(MethodType { return_type, .. }) if return_type == ReturnType::Void => (),
            Ok(_) => return error("return type of <init> must be void.".to_string()),
            Err(e) => return error(e)
        }
    };
    Ok(is_validated)
}

fn check_constant_interface_methodref_info(info: &ConstantInterfaceMethodrefInfo, constant_pool: &Vec<CpInfo>, is_validated: Vec<bool>) -> Result<Vec<bool>> {
    if info.tag != CONSTANT_INTERFACE_METHODREF { return error("The tag item of a CONSTANT_InterfaceMethodref_info structure has the value CONSTANT_InterfaceMethodref (11).".to_string()); }

    let (_, is_validated): (&ConstantClassInfo, _) = match check_each_cp(info.class_index as usize, constant_pool, is_validated)? {
        (CpInfo::Class(info), is_validated) => (info, is_validated),
        _ => return error("The class_index must refer to CONSTANT_Class_info structure.".to_string())
    };
    let (name_and_type_info, is_validated): (&ConstantNameAndTypeInfo,_)  = match check_each_cp(info.name_and_type_index as usize, constant_pool, is_validated)? {
        (CpInfo::NameAndType(info), is_validated) => (info, is_validated),
        _ => return error("The class_index must refer to CONSTANT_NameAndType_info structure.".to_string())
    };

    let (descriptor_info, is_validated): (&ConstantUtf8Info, _) = match check_each_cp(name_and_type_info.descriptor_index as usize, constant_pool, is_validated)? {
        (CpInfo::Utf8(info), is_validated) => (info, is_validated),
        _ => return error("The descriptor_index must refer to CONSTANT_NameAndType_info structure.".to_string())
    };

    let descriptor = descriptor_info.get_utf8_string();

    parse_field_type(&descriptor).or_else(|e| error(e))?;
    Ok(is_validated)
}

// 4.4. The Constant Pool
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4
fn check_each_cp<'a>(index: usize, constant_pool: &'a Vec<CpInfo>, is_validated: Vec<bool>) -> Result<(&'a CpInfo, Vec<bool>)> {
    if constant_pool.get(index).is_none() { return error("missing constant_pool entry.".to_string()); }
    if is_validated[index] { return Ok((&constant_pool[index], is_validated.clone())); }

    let cp_info = &constant_pool[index];
    let is_validated = match cp_info {
        CpInfo::Utf8(info) => { todo!() }
        CpInfo::Integer(info) => { todo!() }
        CpInfo::Float(info) => { todo!() }
        CpInfo::Long(info) => { todo!() }
        CpInfo::Double(info) => { todo!() }
        CpInfo::Class(info) => check_constant_class_info(info, constant_pool, is_validated)?,
        CpInfo::String(info) => check_constant_string_info(info, constant_pool, is_validated)?,
        CpInfo::Fieldref(info) => check_constant_fieldref_info(info, constant_pool, is_validated)?,
        CpInfo::Methodref(info) => check_constant_methodref_info(info, constant_pool, is_validated)?,
        CpInfo::InterfaceMethodref(info) => check_constant_interface_methodref_info(info, constant_pool, is_validated)?,
        CpInfo::NameAndType(info) => { todo!() }
        CpInfo::MethodHandle(info) => { todo!() }
        CpInfo::MethodType(info) => { todo!() }
        CpInfo::Dynamic(info) => { todo!() }
        CpInfo::InvokeDynamic(info) => { todo!() }
        CpInfo::Module(info) => { todo!() }
        CpInfo::Package(info) => { todo!() }
    };

    let mut is_validated = is_validated.to_owned();
    is_validated[index] = true;
    Ok((cp_info, is_validated.clone()))
}

pub fn check_constant_pool(constant_pool: &Vec<CpInfo>) -> Result<()> {
    let mut is_validated: Vec<bool> = vec![false; constant_pool.len()];

    for index in 0..constant_pool.len() {
        is_validated = check_each_cp(index, &constant_pool, is_validated.clone())?.1;
    }

    Ok(())
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

    let _constant_pool = check_constant_pool(&class_file.constant_pool)?;

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