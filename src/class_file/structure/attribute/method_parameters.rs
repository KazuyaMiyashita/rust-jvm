/// 4.7.24. The MethodParameters Attribute
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.24
#[derive(Debug, PartialEq)]
pub struct MethodParametersAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub parameters_count: u8,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name_index: u16,
    pub access_flags: u16,
}