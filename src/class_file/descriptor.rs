use crate::class_file::error::{Result, error};

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
    Class { name: String /* leading `L` and trailing `;` are removed. */ },
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

pub fn parse_field_type(parameter_descriptor: &str) -> Result<FieldType> {
    descriptor_parser::field_type(parameter_descriptor).or(error(format!("invalid parameter descriptor: {}", parameter_descriptor)))
}

pub fn parse_method_descriptor(method_descriptor: &str) -> Result<MethodType> {
    descriptor_parser::method_type(method_descriptor).or(error(format!("invalid method descriptor: {}", method_descriptor)))
}

#[test]
fn test_parse_field_type() {
    assert_eq!(
        parse_field_type("Z"),
        Ok(FieldType::Boolean)
    );
    assert_eq!(
        parse_field_type("Ljava/lang/String;"),
        Ok(FieldType::Class { name: "java/lang/String".to_string() })
    );
    assert_eq!(
        parse_field_type("[[[D"),
        Ok(FieldType::Array { value: Box::new(FieldType::Array { value: Box::new(FieldType::Array { value: Box::new(FieldType::Double) }) }) })
    );
    assert_eq!(
        parse_field_type("BBB"),
        error("invalid parameter descriptor: BBB".to_string())
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
                FieldType::Class { name: "java/lang/Thread".to_string() },
            ],
            return_type: ReturnType::Field { value: FieldType::Class { name: "java/lang/Object".to_string() } },
        })
    );
    assert_eq!(
        parse_method_descriptor("(Ljava/lang/String;I)V"),
        Ok(MethodType {
            parameter_types: vec![
                FieldType::Class { name: "java/lang/String".to_string() },
                FieldType::Int,
            ],
            return_type: ReturnType::Void,
        })
    );
}