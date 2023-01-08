/// 4.7.23. The BootstrapMethods Attribute
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.23
#[derive(Debug, PartialEq)]
pub struct BootstrapMethodsAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub num_bootstrap_methods: u16,
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug, PartialEq)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}