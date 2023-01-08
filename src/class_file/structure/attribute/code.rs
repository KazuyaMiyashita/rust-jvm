use super::Attribute;

/// 4.7.3. The Code Attribute
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.3
#[derive(Debug, PartialEq)]
pub struct CodeAttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub exception_table_length: u16,
    pub exception_table: Vec<ExceptionTable>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, PartialEq)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}