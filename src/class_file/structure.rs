// 4.1. The ClassFile Structure
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
#[derive(Debug, PartialEq)]
pub struct ClassFile {
    pub magic: [u8; 4],
    pub minor_version: u16,
    pub major_version: u16,
    // The original fields constant_pool_count, constant_pool are in the constant_pool structure below,
    // and the original constant_pool has been renamed to cp_infos.
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldsInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
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
    pub fn get_constant_pool_info(&self, index: usize) -> &CpInfo {
        self.cp_infos.get(index - 1).unwrap()
    }
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
pub enum CpInfo {
    ConstantUtf8Info {
        tag: CpInfoTag,
        length: u16,
        bytes: Vec<u8>,
    },
    ConstantIntegerInfo {
        tag: CpInfoTag,
        bytes: u32,
    },
    ConstantFloatInfo {
        tag: CpInfoTag,
        bytes: u32,
    },
    ConstantLongInfo {
        tag: CpInfoTag,
        high_bytes: u32,
        low_bytes: u32,
    },
    ConstantDoubleInfo {
        tag: CpInfoTag,
        high_bytes: u32,
        low_bytes: u32,
    },
    ConstantClassInfo {
        tag: CpInfoTag,
        name_index: u16,
    },
    ConstantStringInfo {
        tag: CpInfoTag,
        string_index: u16,
    },
    ConstantFieldrefInfo {
        tag: CpInfoTag,
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantMethodrefInfo {
        tag: CpInfoTag,
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantInterfaceMethodrefInfo {
        tag: CpInfoTag,
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantNameAndTypeInfo {
        tag: CpInfoTag,
        name_index: u16,
        descriptor_index: u16,
    },
    ConstantMethodHandleInfo {
        tag: CpInfoTag,
        reference_kind: u8,
        reference_index: u16,
    },
    ConstantMethodTypeInfo {
        tag: CpInfoTag,
        descriptor_index: u16,
    },
    ConstantDynamicInfo {
        tag: CpInfoTag,
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    ConstantInvokeDynamicInfo {
        tag: CpInfoTag,
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    ConstantModuleInfo {
        tag: CpInfoTag,
        name_index: u16,
    },
    ConstantPackageInfo {
        tag: CpInfoTag,
        name_index: u16,
    },
}

#[derive(Debug, PartialEq)]
pub struct FieldsInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, PartialEq)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<CodeAttributeInfo>,
}


#[derive(Debug, PartialEq)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub info: Vec<u8>,
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
    pub attributes: Vec<AttributeInfo>,
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
