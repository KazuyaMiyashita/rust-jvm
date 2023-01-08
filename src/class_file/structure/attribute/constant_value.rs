/// 4.7.2. The ConstantValue Attribute
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.2

#[derive(Debug, PartialEq)]
pub struct ConstantValueAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub constantvalue_index: u16,
}