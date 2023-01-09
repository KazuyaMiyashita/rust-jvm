/// 4.7. Attributes
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7
#[derive(Debug, PartialEq)]
pub enum Attribute {
    General(AttributeInfo),
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttributeInfo),
    StackMapTable(StackMapTableAttribute),
    BootstrapMethods(BootstrapMethodsAttribute),
    MethodParameters(MethodParametersAttribute),
    Module(ModuleAttribute),
    NestHost(NestHostAttribute),
    NestMembers(NestMembersAttribute),
    PermittedSubclasses(PermittedSubclassesAttribute)
}

#[derive(Debug, PartialEq)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub info: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct ConstantValueAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub constantvalue_index: u16,
}

// 4.7.3. The Code Attribute
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.3
// The Code attribute is a variable-length attribute in the attributes table of a method_info structure
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

// 4.7.4. The StackMapTable Attribute
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.4
// The StackMapTable attribute is a variable-length attribute in the attributes table of a Code attribute (§4.7.3).
// StackMapTable attribute is used during the process of verification by type checking (§4.10.1).
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct StackMapTableAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_entries: u16,
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum StackMapFrame {
    SameFrame {
        frame_type: u8 // = SAME; /* 0-63 */
    },
    SameLocals1StackItemFrame {
        frame_type: u8,
        // = SAME_LOCALS_1_STACK_ITEM; /* 64-127 */
        stack: Vec<VerificationTypeInfo>, // length is 1 fixed.
    },
    SameLocals1StackItemFrameExtended {
        frame_type: u8,
        // = SAME_LOCALS_1_STACK_ITEM_EXTENDED; /* 247 */
        offset_delta: u16,
        stack: Vec<VerificationTypeInfo>, // length is 1 fixed.
    },
    ChopFrame {
        frame_type: u8,
        // = CHOP; /* 248-250 */
        offset_delta: u16,
    },
    SameFrameExtended {
        frame_type: u8,
        // = SAME_FRAME_EXTENDED; /* 251 */
        offset_delta: u16,
    },
    AppendFrame {
        frame_type: u8,
        // = APPEND; /* 252-254 */
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>, // length is frame_type - 251
    },
    FullFrame {
        frame_type: u8,
        // = FULL_FRAME; /* 255 */
        offset_delta: u16,
        number_of_locals: u16,
        locals: Vec<VerificationTypeInfo>,
        // length is number_of_locals
        number_of_stack_items: u16,
        stack: Vec<VerificationTypeInfo>, // length is number_of_stack_items
    },
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum VerificationTypeInfo {
    TopVariableInfo {
        tag: u8 // = ITEM_Top; /* 0 */
    },
    IntegerVariableInfo {
        tag: u8 // = ITEM_Integer; /* 1 */
    },
    FloatVariableInfo {
        tag: u8 // = ITEM_Float; /* 2 */
    },
    DoubleVariableInfo {
        tag: u8 // = ITEM_Double; /* 3 */
    },
    LongVariableInfo {
        tag: u8 // = ITEM_Long; /* 4 */
    },
    NullVariableInfo {
        tag: u8 // = ITEM_Null; /* 5 */
    },
    UninitializedThisVariableInfo {
        tag: u8 // = ITEM_UninitializedThis; /* 6 */
    },
    ObjectVariableInfo {
        tag: u8,
        // = ITEM_Object; /* 7 */
        cpool_index: u16,
    },
    UninitializedVariableInfo {
        tag: u8,
        // = ITEM_Uninitialized; /* 8 */
        offset: u16,
    },
}


#[derive(Debug, PartialEq)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

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

/// 4.7.28. The NestHost Attribute
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.28
#[derive(Debug, PartialEq)]
pub struct NestHostAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub host_class_index: u16,
}

/// 4.7.29. The NestMembers Attribute
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.29
#[derive(Debug, PartialEq)]
pub struct NestMembersAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_classes: u16,
    pub classes: Vec<u16>
}

/// 4.7.31. The PermittedSubclasses Attribute
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.31
#[derive(Debug, PartialEq)]
pub struct PermittedSubclassesAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_classes: u16,
    pub classes: Vec<u16>
}