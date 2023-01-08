// 4.4. The Constant Pool
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4
#[derive(Debug, PartialEq)]
pub struct ConstantPool {
    pub constant_pool_count: u16,
    pub cp_infos: Vec<CpInfo>,
}

pub type CpInfoTag = u8;

pub const CONSTANT_UTF8: CpInfoTag = 1;
pub const CONSTANT_INTEGER: CpInfoTag = 3;
pub const CONSTANT_FLOAT: CpInfoTag = 4;
pub const CONSTANT_LONG: CpInfoTag = 5;
pub const CONSTANT_DOUBLE: CpInfoTag = 6;
pub const CONSTANT_CLASS: CpInfoTag = 7;
pub const CONSTANT_STRING: CpInfoTag = 8;
pub const CONSTANT_FIELDREF: CpInfoTag = 9;
pub const CONSTANT_METHODREF: CpInfoTag = 10;
pub const CONSTANT_INTERFACE_METHODREF: CpInfoTag = 11;
pub const CONSTANT_NAME_AND_TYPE: CpInfoTag = 12;
pub const CONSTANT_METHOD_HANDLE: CpInfoTag = 15;
pub const CONSTANT_METHOD_TYPE: CpInfoTag = 16;
pub const CONSTANT_DYNAMIC: CpInfoTag = 17;
pub const CONSTANT_INVOKE_DYNAMIC: CpInfoTag = 18;
pub const CONSTANT_MODULE: CpInfoTag = 19;
pub const CONSTANT_PACKAGE: CpInfoTag = 20;

#[derive(Debug, PartialEq)]
pub struct ConstantUtf8Info {
    pub tag: CpInfoTag,
    pub length: u16,
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct ConstantIntegerInfo {
    pub tag: CpInfoTag,
    pub bytes: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct ConstantFloatInfo {
    pub tag: CpInfoTag,
    pub bytes: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct ConstantLongInfo {
    pub tag: CpInfoTag,
    pub high_bytes: [u8; 4],
    pub low_bytes: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct ConstantDoubleInfo {
    pub tag: CpInfoTag,
    pub high_bytes: [u8; 4],
    pub low_bytes: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct ConstantClassInfo {
    pub tag: CpInfoTag,
    pub name_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantStringInfo {
    pub tag: CpInfoTag,
    pub string_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantFieldrefInfo {
    pub tag: CpInfoTag,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantMethodrefInfo {
    pub tag: CpInfoTag,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantInterfaceMethodrefInfo {
    pub tag: CpInfoTag,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantNameAndTypeInfo {
    pub tag: CpInfoTag,
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantMethodHandleInfo {
    pub tag: CpInfoTag,
    pub reference_kind: u8,
    pub reference_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantMethodTypeInfo {
    pub tag: CpInfoTag,
    pub descriptor_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantDynamicInfo {
    pub tag: CpInfoTag,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantInvokeDynamicInfo {
    pub tag: CpInfoTag,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantModuleInfo {
    pub tag: CpInfoTag,
    pub name_index: u16,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPackageInfo {
    pub tag: CpInfoTag,
    pub name_index: u16,
}

#[derive(Debug, PartialEq)]
pub enum CpInfo {
    Utf8(ConstantUtf8Info),
    Integer(ConstantIntegerInfo),
    Float(ConstantFloatInfo),
    Long(ConstantLongInfo),
    Double(ConstantDoubleInfo),
    Class(ConstantClassInfo),
    String(ConstantStringInfo),
    Fieldref(ConstantFieldrefInfo),
    Methodref(ConstantMethodrefInfo),
    InterfaceMethodref(ConstantInterfaceMethodrefInfo),
    NameAndType(ConstantNameAndTypeInfo),
    MethodHandle(ConstantMethodHandleInfo),
    MethodType(ConstantMethodTypeInfo),
    Dynamic(ConstantDynamicInfo),
    InvokeDynamic(ConstantInvokeDynamicInfo),
    Module(ConstantModuleInfo),
    Package(ConstantPackageInfo),
}