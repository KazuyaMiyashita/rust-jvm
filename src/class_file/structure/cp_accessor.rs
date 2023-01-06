use super::*;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AccessError>;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("Invalid class file. {message:}")]
pub struct AccessError {
    pub message: String,
}

// utils
fn error<T>(message: String) -> Result<T> {
    Err(AccessError { message })
}

fn cp_info_name(cp_info: &CpInfo) -> &str {
    match cp_info {
        CpInfo::Utf8(_) => "CONSTANT_Utf8",
        CpInfo::Integer(_) => "CONSTANT_Integer",
        CpInfo::Float(_) => "CONSTANT_Float",
        CpInfo::Long(_) => "CONSTANT_Long",
        CpInfo::Double(_) => "CONSTANT_Double",
        CpInfo::Class(_) => "CONSTANT_Class",
        CpInfo::String(_) => "CONSTANT_String",
        CpInfo::Fieldref(_) => "CONSTANT_Fieldref",
        CpInfo::Methodref(_) => "CONSTANT_Methodref",
        CpInfo::InterfaceMethodref(_) => "CONSTANT_InterfaceMethodref",
        CpInfo::NameAndType(_) => "CONSTANT_NameAndType",
        CpInfo::MethodHandle(_) => "CONSTANT_MethodHandle",
        CpInfo::MethodType(_) => "CONSTANT_MethodType",
        CpInfo::Dynamic(_) => "CONSTANT_Dynamic",
        CpInfo::InvokeDynamic(_) => "CONSTANT_InvokeDynamic",
        CpInfo::Module(_) => "CONSTANT_Module",
        CpInfo::Package(_) => "CONSTANT_Package",
    }
}

// original constant_pool table is indexed from 1 to constant_pool_count - 1.
// Note that the Vec of this cp_infos structure is indexed from 0.
fn get_constant_pool_info(constant_pool: &Vec<CpInfo>, index: u16) -> Result<&CpInfo> {
    match constant_pool.get((index as usize) - 1) {
        Some(cp_info) => Ok(cp_info),
        None => error(format!("the index of constant_pool not found! index: {}", index)),
    }
}

pub trait CpAccessor {
    // indexed from 1 to constant_pool_count - 1.
    fn access_as_class<'a>(&'a self, index: u16) -> ClassCpAccessor<'a>;
}

impl CpAccessor for &Vec<CpInfo> {
    fn access_as_class<'a>(&'a self, index: u16) -> ClassCpAccessor<'a> {
        match get_constant_pool_info(self, index) {
            Ok(cp) => {
                match cp {
                    CpInfo::Class(info) => ClassCpAccessor { constant_pool: *self, class_info_or_err: Ok(info) },
                    other_info => ClassCpAccessor { constant_pool: *self, class_info_or_err: error(format!("Must specify index of CONSTANT_Class_info but {} found! index: {}", cp_info_name(other_info), index)) },
                }
            }
            Err(e) => ClassCpAccessor { constant_pool: *self, class_info_or_err: Err(e) }
        }
    }
}

pub struct ClassCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    class_info_or_err: Result<&'a ConstantClassInfo>,
}

impl ClassCpAccessor<'_> {
    fn name(&self) -> Utf8CpAccessor {
        match &self.class_info_or_err {
            Ok(class_info) => {
                match get_constant_pool_info(&self.constant_pool, class_info.name_index) {
                    Ok(CpInfo::Utf8(info)) => Utf8CpAccessor { constant_pool: self.constant_pool, utf8_info_or_err: Ok(&info) },
                    Ok(other_info) => Utf8CpAccessor {
                        constant_pool: self.constant_pool,
                        utf8_info_or_err: error(format!("The name_index must refer to CONSTANT_Utf8_info structure, but {} found! name_index: {}", cp_info_name(other_info), class_info.name_index)),
                    },
                    Err(e) => Utf8CpAccessor { constant_pool: self.constant_pool, utf8_info_or_err: Err(e) }
                }
            }
            Err(e) => Utf8CpAccessor { constant_pool: self.constant_pool, utf8_info_or_err: Err(e.to_owned()) }
        }
    }
}

pub struct Utf8CpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    utf8_info_or_err: Result<&'a ConstantUtf8Info>,
}

impl Utf8CpAccessor<'_> {
    fn bytes_as_string(&self) -> Result<String> {

        match &self.utf8_info_or_err {
            Ok(utf8_info) => String::from_utf8(utf8_info.bytes.clone()).or_else(|e| error(e.to_string())),
            Err(e) => Err(e.to_owned())
        }
    }
}

#[test]
fn test() {
    let constant_pool: &Vec<CpInfo> = &vec![
        CpInfo::Methodref(ConstantMethodrefInfo { tag: 0x0a, class_index: 0x02, name_and_type_index: 0x03 }),
        CpInfo::Class(ConstantClassInfo { tag: 0x07, name_index: 0x04 }),
        CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 0x0c, name_index: 0x05, descriptor_index: 0x06 }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x10, bytes: "java/lang/Object".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x06, bytes: "<init>".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x03, bytes: "()V".as_bytes().to_vec() }),
        CpInfo::Methodref(ConstantMethodrefInfo { tag: 0x0a, class_index: 0x08, name_and_type_index: 0x09 }),
        CpInfo::Class(ConstantClassInfo { tag: 0x07, name_index: 0x0a }),
        CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 0x0c, name_index: 0x0b, descriptor_index: 0x0c }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x07, bytes: "Sample1".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x03, bytes: "add".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x05, bytes: "(II)I".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x04, bytes: "Code".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x0f, bytes: "LineNumberTable".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x04, bytes: "prog".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x03, bytes: "()I".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x0a, bytes: "SourceFile".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x0c, bytes: "Sample1.java".as_bytes().to_vec() }),
    ];

    let str = constant_pool.access_as_class(8).name().bytes_as_string();

    assert_eq!(str, Ok("Sample1".to_string()))
}