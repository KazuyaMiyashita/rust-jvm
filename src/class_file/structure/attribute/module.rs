/// 4.7.25. The Module Attribute
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.25
#[derive(Debug, PartialEq)]
pub struct ModuleAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub module_name_index: u16,
    pub module_flags: u16,
    pub module_version_index: u16,
    pub requires_count: u16,
    pub requires: Vec<Require>,
    pub exports_count: u16,
    pub exports: Vec<Export>,
    pub opens_count: u16,
    pub opens: Vec<Open>,
    pub uses_count: u16,
    pub uses_index: Vec<u16>,
    pub provides_count: u16,
    pub provides: Vec<Provide>,
}

#[derive(Debug, PartialEq)]
pub struct Require {
    pub requires_index: u16,
    pub requires_flags: u16,
    pub requires_version_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct Export {
    pub exports_index: u16,
    pub exports_flags: u16,
    pub exports_to_count: u16,
    pub exports_to_index: Vec<u16>,
}

#[derive(Debug, PartialEq)]
pub struct Open {
    pub opens_index: u16,
    pub opens_flags: u16,
    pub opens_to_count: u16,
    pub opens_to_index: Vec<u16>,
}

#[derive(Debug, PartialEq)]
pub struct Provide {
    pub provides_index: u16,
    pub provides_with_count: u16,
    pub provides_with_index: Vec<u16>,
}