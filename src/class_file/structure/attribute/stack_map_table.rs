/// 4.7.4. The StackMapTable Attribute
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.4
/// The StackMapTable attribute is a variable-length attribute in the attributes table of a Code attribute (§4.7.3).
/// StackMapTable attribute is used during the process of verification by type checking (§4.10.1).
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