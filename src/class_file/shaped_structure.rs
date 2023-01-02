// 4.1. The ClassFile Structure
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
#[derive(Debug, PartialEq)]
pub struct ClassFile {
    pub magic: Magic,
    // The original fields minor_version, major_version are in the version structure below.
    pub version: Version,
    // The original fields constant_pool_count, constant_pool are in the constant_pool structure below,
    // and the original constant_pool has been renamed to cp_infos.
    pub constant_pool: ConstantPool,
}

#[derive(Debug, PartialEq)]
pub struct Magic {
    pub value: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct Version {
    pub minor_version: u16,
    pub major_version: u16,
}

// 4.4. The Constant Pool
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4
#[derive(Debug, PartialEq)]
pub struct ConstantPool {
    pub constant_pool_count: u16,
    pub cp_infos: Vec<CpInfo>,
}

impl ConstantPool {
    // original constant_pool table is indexed from 1 to constant_pool_count - 1.
    // Note that the Vec of this cp_infos structure is indexed from 0.
    pub fn get_constant_pool_info(&self, index: usize) -> Option<&CpInfo> {
        self.cp_infos.get(index - 1)
    }
}

pub type CpInfoTag = u8;
pub type CpIndex = u16;

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
pub enum CpInfo {
    ConstantFieldrefInfo {
        tag: CpInfoTag,
        class_index: CpIndex,
        name_and_type_index: CpIndex,
        class_name: String,
        field_name: String,
        field_or_method_type: Or<FieldType, MethodType>,
    },
    ConstantMethodrefInfo {
        tag: CpInfoTag,
        class_index: CpIndex,
        name_and_type_index: CpIndex,
        class_name: String,
        method_name: String,
        method_type: MethodType,
    },
    ConstantInterfaceMethodrefInfo {
        tag: CpInfoTag,
        class_index: CpIndex,
        name_and_type_index: CpIndex,
    },
}

#[derive(Debug, PartialEq)]
pub enum FieldOrMethodType {
    FieldType { field: FieldType },
    MethodType { method: MethodType },
}

#[derive(Debug, PartialEq)]
pub enum Or<A, B> {
    A(A),
    B(B),
}

#[derive(Debug, PartialEq)]
pub struct MethodType {
    pub parameter_types: Vec<FieldType>,
    pub return_type: ReturnType,
}

#[derive(Debug, PartialEq)]
pub enum FieldType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Class { name: String }, // leading `L` and trailing `;` are removed.
    Short,
    Boolean,
    Array { value: Box<FieldType> },
}

#[derive(Debug, PartialEq)]
pub enum ReturnType {
    Field { value: FieldType },
    Void,
}

peg::parser! {
    grammar descriptor_parser() for str {
        // not strict.
        // see below for class names
        // https://docs.oracle.com/javase/specs/jls/se17/html/jls-8.html#jls-8.1
        // and see below for internal form of class name
        // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.2.1
        pub rule class_name() -> String
            = str:$(['a'..='z' | 'A'..='Z' | '/' | '$' | '_'] ['a'..='z' | 'A'..='Z' | '/' | '$' | '_' | '0'..='9']*) { str.to_string() }

        pub rule field_type() -> FieldType
            = "B" { FieldType::Byte } /
              "C" { FieldType::Char } /
              "D" { FieldType::Double } /
              "F" { FieldType::Float } /
              "I" { FieldType::Int } /
              "J" { FieldType::Long } /
              "L" str:class_name() ";" { FieldType::Class { name: str } } /
              "S" { FieldType::Char } /
              "Z" { FieldType::Boolean } /
              "[" ft:field_type() { FieldType::Array { value: Box::new(ft)} }

        pub rule return_type() -> ReturnType
            = ft:field_type() { ReturnType::Field { value: ft } } /
              "V" { ReturnType::Void }

        pub rule method_type() -> MethodType
            = "(" fs:field_type()* ")" rt:return_type() { MethodType { parameter_types: fs, return_type: rt } }
    }
}

fn parse_field_type(parameter_descriptor: &str) -> Result<FieldType, String> {
    descriptor_parser::field_type(parameter_descriptor).map_err(|_| format!("invalid parameter descriptor: {}", parameter_descriptor))
}

fn parse_method_descriptor(method_descriptor: &str) -> Result<MethodType, String> {
    descriptor_parser::method_type(method_descriptor).map_err(|_| format!("invalid method descriptor: {}", method_descriptor))
}

#[test]
fn test_parse_field_type() {
    assert_eq!(
        parse_field_type("Z"),
        Ok(FieldType::Boolean)
    );
    assert_eq!(
        parse_field_type("Ljava/lang/String;"),
        Ok(FieldType::Class { name: "java/lang/String".to_string() } )
    );
    assert_eq!(
        parse_field_type("[[[D"),
        Ok(FieldType::Array { value: Box::new(FieldType::Array { value: Box::new(FieldType::Array { value: Box::new(FieldType::Double) }) }) })
    );
    assert_eq!(
        parse_field_type("BBB"),
        Err("invalid parameter descriptor: BBB".to_string())
    );
}

#[test]
fn test_parse_method_descriptor() {
    assert_eq!(
        parse_method_descriptor("(IDLjava/lang/Thread;)Ljava/lang/Object;"),
        Ok(MethodType {
            parameter_types: vec![
                FieldType::Int,
                FieldType::Double,
                FieldType::Class { name: "java/lang/Thread".to_string() }
            ],
            return_type: ReturnType::Field { value: FieldType::Class { name: "java/lang/Object".to_string() } } ,
        })
    );
    assert_eq!(
        parse_method_descriptor("(Ljava/lang/String;I)V"),
        Ok(MethodType {
            parameter_types: vec![
                FieldType::Class { name: "java/lang/String".to_string() } ,
                FieldType::Int
            ],
            return_type: ReturnType::Void,
        })
    );
}