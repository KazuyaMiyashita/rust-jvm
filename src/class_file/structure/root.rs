use super::constant_pool;
use super::attribute;

// 4.1. The ClassFile Structure
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
#[derive(Debug, PartialEq)]
pub struct ClassFile {
    pub magic: [u8; 4],
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<constant_pool::CpInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldsInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<attribute::Attribute>,
}

#[derive(Debug, PartialEq)]
pub struct FieldsInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<attribute::Attribute>,
}

#[derive(Debug, PartialEq)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<attribute::Attribute>,
}