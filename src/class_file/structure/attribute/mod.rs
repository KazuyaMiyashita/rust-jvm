pub mod constant_value;
pub mod code;
pub mod stack_map_table;
pub mod bootstrap_methods;
pub mod method_parameters;
pub mod module;

/// 4.7 Attributes
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7
/// - [x] ConstantValue
/// - [x] Code
/// - [x] StackMapTable
/// - [ ] Exceptions
/// - [x] BootstrapMethods
/// - [ ] NestHost
/// - [ ] NestMembers
/// - [ ] PermittedSubclasses
/// - [ ] InnerClasses
/// - [ ] EnclosingMethod
/// - [ ] Synthetic
/// - [ ] Signature
/// - [ ] Record
/// - [ ] SourceFile
/// - [ ] LineNumberTable
/// - [ ] LocalVariableTable
/// - [ ] LocalVariableTypeTable
/// - [ ] SourceDebugExtension
/// - [ ] Deprecated
/// - [ ] RuntimeVisibleAnnotations
/// - [ ] RuntimeInvisibleAnnotations
/// - [ ] RuntimeVisibleParameterAnnotations
/// - [ ] RuntimeInvisibleParameterAnnotations
/// - [ ] RuntimeVisibleTypeAnnotations
/// - [ ] RuntimeInvisibleTypeAnnotations
/// - [ ] AnnotationDefault
/// - [x] MethodParameters
/// - [x] Module
/// - [ ] ModulePackages
/// - [ ] ModuleMainClass
#[derive(Debug, PartialEq)]
pub enum Attribute {
    General(AttributeInfo),
    ConstantValue(constant_value::ConstantValueAttribute),
    Code(code::CodeAttributeInfo),
    StackMapTable(stack_map_table::StackMapTableAttribute),
    BootstrapMethods(bootstrap_methods::BootstrapMethodsAttribute),
    MethodParameters(method_parameters::MethodParametersAttribute),
    Module(module::ModuleAttribute),
}

/// general format
#[derive(Debug, PartialEq)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub info: Vec<u8>,
}
