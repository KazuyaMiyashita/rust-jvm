#[derive(Debug, PartialEq)]
pub struct ClassFile {
    magic: [u8; 4],
    minor_version: [u8; 2],
    major_version: [u8; 2],
    // The original fields constant_pool_count, constant_pool are in the constant_pool structure below,
    // and the original constant_pool has been renamed to cp_infos.
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    fields: Vec<FieldsInfo>,
    methods_count: u16,
    methods: Vec<MethodInfo>,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    fn read_from_bytes(bytes: &[u8]) -> ClassFile {
        let mut offset: usize = 0;

        let magic: [u8; 4] = read_n(&bytes, &mut offset);

        let minor_version: [u8; 2] = read_n(&bytes, &mut offset);

        let major_version: [u8; 2] = read_n(&bytes, &mut offset);

        let constant_pool: ConstantPool = ConstantPool::read(&bytes, &mut offset);

        let access_flags: u16 = read_u16(&bytes, &mut offset);

        let this_class: u16 = read_u16(&bytes, &mut offset);

        let super_class: u16 = read_u16(&bytes, &mut offset);

        let interfaces_count: u16 = read_u16(&bytes, &mut offset);

        let mut interfaces: Vec<u16> = Vec::new();
        for _ in 0..interfaces_count {
            let interface = read_u16(&bytes, &mut offset);
            interfaces.push(interface);
        }

        let fields_count: u16 = read_u16(&bytes, &mut offset);
        let mut fields: Vec<FieldsInfo> = Vec::new();
        for _ in 0..fields_count {
            fields.push(FieldsInfo::read(&bytes, &mut offset));
        }

        let methods_count: u16 = read_u16(&bytes, &mut offset);
        let mut methods: Vec<MethodInfo> = Vec::new();
        for _ in 0..methods_count {
            methods.push(MethodInfo::read(&bytes, &mut offset));
        }

        let attributes_count: u16 = read_u16(&bytes, &mut offset);
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::read(&bytes, &mut offset));
        }

        // 4.8. Format Checking
        // The class file must not be truncated or have extra bytes at the end.
        if bytes.len() != offset {
            panic!("class file too long!")
        }

        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        }
    }

    // 4.8. Format Checking
    // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.8
    fn check_format(&self) -> () {
        // The first four bytes must contain the right magic number.
        if self.magic != [0xca, 0xfe, 0xba, 0xbe] {
            panic!("The first four bytes must contain the right magic number.")
        }

        // 4.7. Attributes
        // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7
        //
        // Attributes are used in the ClassFile, field_info, method_info, Code_attribute, and record_component_info
        // structures of the class file format (§4.1, §4.5, §4.6, §4.7.3, §4.7.30).
        //
        // For all attributes, the attribute_name_index item must be a valid unsigned 16-bit index into the constant pool of the class.
        // The constant_pool entry at attribute_name_index must be a CONSTANT_Utf8_info structure (§4.4.7) representing the name of the attribute.
        let all_attribute_name_index_iter = self.attributes.iter().map(|x| x.attribute_name_index)
            .chain(self.fields.iter().flat_map(|a| a.attributes.iter().map(|y| y.attribute_name_index)))
            .chain(self.methods.iter().flat_map(|a| a.attributes.iter().map(|y| y.attribute_name_index)));
        // TODO: add Code_attribute https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.3
        // TODO: add record_component_info https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.30
        for attribute_name_index in all_attribute_name_index_iter {
            let cp_info = self.constant_pool.get_constant_pool_info(attribute_name_index as usize);
            match cp_info {
                CpInfo::ConstantUtf8Info { tag: CONSTANT_UTF8, .. } => (),
                _ => panic!("attribute's constant_pool entry must be a CONSTANT_Utf8_info structure!")
            }
        }

        // All predefined attributes (§4.7) must be of the proper length, except for StackMapTable,
        // RuntimeVisibleAnnotations, RuntimeInvisibleAnnotations, RuntimeVisibleParameterAnnotations,
        // RuntimeInvisibleParameterAnnotations, RuntimeVisibleTypeAnnotations, RuntimeInvisibleTypeAnnotations, and AnnotationDefault.
        // - TODO

        // The class file must not be truncated or have extra bytes at the end.
        // - This is done with ClassFile::read_from_bytes

        // The constant pool must satisfy the constraints documented throughout §4.4.
        // - タグの範囲は検証済みである。
        //
        // -- ConstantClassInfo の場合、 name_index は以下を満たす
        // The value of the name_index item must be a valid index into the constant_pool table.
        // The constant_pool entry at that index must be a CONSTANT_Utf8_info structure (§4.4.7) representing a valid binary class
        // or interface name encoded in internal form (§4.2.1).
        let constant_pool_constant_class_info_name_indexes= self.constant_pool.cp_infos.iter()
            .flat_map(|x| match x {  CpInfo::ConstantClassInfo { name_index, .. } => Some(*name_index), _ => None });
        for constant_pool_constant_class_info_name_index in constant_pool_constant_class_info_name_indexes {
            match self.constant_pool.get_constant_pool_info(constant_pool_constant_class_info_name_index as usize) {
                CpInfo::ConstantUtf8Info { tag: CONSTANT_UTF8, .. } => (),
                _ => panic!("The constant_pool entry at ConstantClassInfo's name_index must be a CONSTANT_Utf8_info structure!")
            }
        }

        // -- ConstantFieldrefInfo, ConstantMethodrefInfo, ConstantInterfaceMethodrefInfo に関して
        // class_index は constant_pool table の有効なインデックスである必要がある。


        // QUESTION:
        // さらに、 ConstantFieldrefInfo の場合、その class_index が指し示すものはクラス型またはインターフェイス型、
        // ConstantMethodrefInfo の場合、その class_index が指し示すものはクラス型でありインターフェース型ではない、
        // ConstantInterfaceMethodrefInfo の場合、 class_index が指し示すものはインターフェース型でありクラス型ではないとあるが、
        // これは検証する必要があるのだろうか？(検証が可能なのだろうか？)
        // ConstantClassInfo はクラスまたはインターフェースを表すために利用されるものだが、
        // クラスかインターフェースかを区別する識別子は ConstantClassInfo の中にはない。
        //
        // In a CONSTANT_Fieldref_info structure, the class_index item may be either a class type or an interface type.
        // In a CONSTANT_Methodref_info structure, the class_index item must be a class type, not an interface type.
        // In a CONSTANT_InterfaceMethodref_info structure, the class_index item must be an interface type, not a class type.
        // cf. https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.2

        // All field references and method references in the constant pool must have valid names, valid classes, and valid descriptors (§4.3).
        // -- TODO..

        todo!()
    }

    pub fn load(bytes: &[u8]) -> ClassFile {
        let cf = ClassFile::read_from_bytes(bytes);
        cf.check_format();
        cf
    }
}

#[derive(Debug, PartialEq)]
struct ConstantPool {
    constant_pool_count: u16,
    cp_infos: Vec<CpInfo>,
}

impl ConstantPool {
    fn read(bytes: &[u8], offset: &mut usize) -> ConstantPool {
        let constant_pool_count: u16 = read_u16(&bytes, &mut *offset);

        let mut cp_infos: Vec<CpInfo> = Vec::new();
        // The constant_pool table is indexed from 1 to constant_pool_count - 1. .. https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
        for _ in 1..constant_pool_count {
            let cp = CpInfo::read(&bytes, &mut *offset);
            cp_infos.push(cp);
        };

        ConstantPool {
            constant_pool_count,
            cp_infos,
        }
    }

    // original constant_pool table is indexed from 1 to constant_pool_count - 1.
    // NOTICE: A call to get_constant_pool_info(0) yields an error.
    fn get_constant_pool_info(&self, index: usize) -> &CpInfo {
        self.cp_infos.get(index - 1).unwrap()
    }
}

type CpInfoTag = u8;

const CONSTANT_UTF8: CpInfoTag = 1;
const CONSTANT_INTEGER: CpInfoTag = 3;
const CONSTANT_FLOAT: CpInfoTag = 4;
const CONSTANT_LONG: CpInfoTag = 5;
const CONSTANT_DOUBLE: CpInfoTag = 6;
const CONSTANT_CLASS: CpInfoTag = 7;
const CONSTANT_STRING: CpInfoTag = 8;
const CONSTANT_FIELDREF: CpInfoTag = 9;
const CONSTANT_METHODREF: CpInfoTag = 10;
const CONSTANT_INTERFACE_METHODREF: CpInfoTag = 11;
const CONSTANT_NAME_AND_TYPE: CpInfoTag = 12;
const CONSTANT_METHOD_HANDLE: CpInfoTag = 15;
const CONSTANT_METHOD_TYPE: CpInfoTag = 16;
const CONSTANT_DYNAMIC: CpInfoTag = 17;
const CONSTANT_INVOKE_DYNAMIC: CpInfoTag = 18;
const CONSTANT_MODULE: CpInfoTag = 19;
const CONSTANT_PACKAGE: CpInfoTag = 20;

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

impl CpInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> CpInfo {
        let tag: CpInfoTag = read_u8(&bytes, &mut *offset);
        match tag {
            CONSTANT_UTF8 => {
                let length = read_u16(&bytes, &mut *offset);
                CpInfo::ConstantUtf8Info {
                    tag,
                    length,
                    bytes: read_u8_vec(&bytes, &mut *offset, length as usize),
                }
            }
            CONSTANT_INTEGER => {
                CpInfo::ConstantIntegerInfo {
                    tag,
                    bytes: read_u32(&bytes, &mut *offset),
                }
            }
            CONSTANT_FLOAT => {
                CpInfo::ConstantFloatInfo {
                    tag,
                    bytes: read_u32(&bytes, &mut *offset),
                }
            }
            CONSTANT_LONG => {
                CpInfo::ConstantLongInfo {
                    tag,
                    high_bytes: read_u32(&bytes, &mut *offset),
                    low_bytes: read_u32(&bytes, &mut *offset),
                }
            }
            CONSTANT_DOUBLE => {
                CpInfo::ConstantDoubleInfo {
                    tag,
                    high_bytes: read_u32(&bytes, &mut *offset),
                    low_bytes: read_u32(&bytes, &mut *offset),
                }
            }
            CONSTANT_CLASS => {
                CpInfo::ConstantClassInfo {
                    tag,
                    name_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_STRING => {
                CpInfo::ConstantStringInfo {
                    tag,
                    string_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_FIELDREF => {
                CpInfo::ConstantFieldrefInfo {
                    tag,
                    class_index: read_u16(&bytes, &mut *offset),
                    name_and_type_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_METHODREF => {
                CpInfo::ConstantMethodrefInfo {
                    tag,
                    class_index: read_u16(&bytes, &mut *offset),
                    name_and_type_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_INTERFACE_METHODREF => {
                CpInfo::ConstantInterfaceMethodrefInfo {
                    tag,
                    class_index: read_u16(&bytes, &mut *offset),
                    name_and_type_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_NAME_AND_TYPE => {
                CpInfo::ConstantNameAndTypeInfo {
                    tag,
                    name_index: read_u16(&bytes, &mut *offset),
                    descriptor_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_METHOD_HANDLE => {
                CpInfo::ConstantMethodHandleInfo {
                    tag,
                    reference_kind: read_u8(&bytes, &mut *offset),
                    reference_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_METHOD_TYPE => {
                CpInfo::ConstantMethodTypeInfo {
                    tag,
                    descriptor_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_DYNAMIC => {
                CpInfo::ConstantDynamicInfo {
                    tag,
                    bootstrap_method_attr_index: read_u16(&bytes, &mut *offset),
                    name_and_type_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_INVOKE_DYNAMIC => {
                CpInfo::ConstantInvokeDynamicInfo {
                    tag,
                    bootstrap_method_attr_index: read_u16(&bytes, &mut *offset),
                    name_and_type_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_MODULE => {
                CpInfo::ConstantModuleInfo {
                    tag,
                    name_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_PACKAGE => {
                CpInfo::ConstantPackageInfo {
                    tag,
                    name_index: read_u16(&bytes, &mut *offset),
                }
            }
            _ => panic!("unsupported tag {}", tag)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldsInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl FieldsInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> FieldsInfo {
        let access_flags: u16 = read_u16(&bytes, &mut *offset);
        let name_index: u16 = read_u16(&bytes, &mut *offset);
        let descriptor_index: u16 = read_u16(&bytes, &mut *offset);
        let attributes_count: u16 = read_u16(&bytes, &mut *offset);
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::read(&bytes, &mut *offset));
        }
        FieldsInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<CodeAttributeInfo>,
}

impl MethodInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> MethodInfo {
        let access_flags: u16 = read_u16(&bytes, &mut *offset);
        let name_index: u16 = read_u16(&bytes, &mut *offset);
        let descriptor_index: u16 = read_u16(&bytes, &mut *offset);
        let attributes_count: u16 = read_u16(&bytes, &mut *offset);
        let mut attributes: Vec<CodeAttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(CodeAttributeInfo::read(&bytes, &mut *offset));
        }
        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

impl AttributeInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> AttributeInfo {
        let attribute_name_index: u16 = read_u16(&bytes, &mut *offset);
        let attribute_length: u32 = read_u32(&bytes, &mut *offset);
        let mut info: Vec<u8> = Vec::new();
        for _ in 0..attribute_length {
            let i = read_u8(&bytes, &mut *offset);
            info.push(i)
        }
        AttributeInfo {
            attribute_name_index,
            attribute_length,
            info,
        }
    }
}

// 4.7.3. The Code Attribute
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.3
// The Code attribute is a variable-length attribute in the attributes table of a method_info structure
#[derive(Debug, PartialEq)]
pub struct CodeAttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    max_stack: u16,
    max_locals: u16,
    code_length: u32,
    code: Vec<u8>,
    exception_table_length: u16,
    exception_table: Vec<ExceptionTable>,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl CodeAttributeInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> CodeAttributeInfo {
        let attribute_name_index: u16 = read_u16(&bytes, &mut *offset);
        let attribute_length: u32 = read_u32(&bytes, &mut *offset);
        let max_stack: u16 = read_u16(&bytes, &mut *offset);
        let max_locals: u16 = read_u16(&bytes, &mut *offset);
        let code_length: u32 = read_u32(&bytes, &mut *offset);
        let mut code: Vec<u8> = Vec::new();
        for _ in 0..code_length {
            code.push(read_u8(&bytes, &mut *offset))
        }
        let exception_table_length: u16 = read_u16(&bytes, &mut *offset);
        let mut exception_table: Vec<ExceptionTable> = Vec::new();
        for _ in 0..exception_table_length {
            exception_table.push(ExceptionTable::read(&bytes, &mut *offset))
        }
        let attributes_count: u16 = read_u16(&bytes, &mut *offset);
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::read(&bytes, &mut *offset))
        }
        CodeAttributeInfo {
            attribute_name_index,
            attribute_length,
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table_length,
            exception_table,
            attributes_count,
            attributes,
        }
    }
}

// 4.7.4. The StackMapTable Attribute
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.4
// The StackMapTable attribute is a variable-length attribute in the attributes table of a Code attribute (§4.7.3).
// StackMapTable attribute is used during the process of verification by type checking (§4.10.1).
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct StackMapTableAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    number_of_entries: u16,
    entries: Vec<StackMapFrame>,
}

#[allow(dead_code)]
impl StackMapTableAttribute {
    fn read(bytes: &[u8], offset: &mut usize) -> StackMapTableAttribute {
        let attribute_name_index: u16 = read_u16(&bytes, &mut *offset);
        let attribute_length: u32 = read_u32(&bytes, &mut *offset);
        let number_of_entries: u16 = read_u16(&bytes, &mut *offset);
        let mut entries: Vec<StackMapFrame> = Vec::new();
        for _ in 0..number_of_entries {
            entries.push(StackMapFrame::read(&bytes, &mut *offset));
        }
        StackMapTableAttribute {
            attribute_name_index,
            attribute_length,
            number_of_entries,
            entries,
        }
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum StackMapFrame {
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

#[allow(dead_code)]
impl StackMapFrame {
    fn read(bytes: &[u8], offset: &mut usize) -> StackMapFrame {
        let frame_type: u8 = read_u8(&bytes, &mut *offset);
        match frame_type {
            0..=63 => StackMapFrame::SameFrame { frame_type },
            64..=127 => StackMapFrame::SameLocals1StackItemFrame {
                frame_type,
                stack: VerificationTypeInfo::read(&bytes, &mut *offset, 1),
            },
            247 => StackMapFrame::SameLocals1StackItemFrameExtended {
                frame_type,
                offset_delta: read_u16(&bytes, &mut *offset),
                stack: VerificationTypeInfo::read(&bytes, &mut *offset, 1),
            },
            248..=250 => StackMapFrame::ChopFrame {
                frame_type,
                offset_delta: read_u16(&bytes, &mut *offset),
            },
            251 => StackMapFrame::SameFrameExtended {
                frame_type,
                offset_delta: read_u16(&bytes, &mut *offset),
            },
            252..=254 => StackMapFrame::AppendFrame {
                frame_type,
                offset_delta: read_u16(&bytes, &mut *offset),
                locals: VerificationTypeInfo::read(&bytes, &mut *offset, (frame_type - 251) as u16),
            },
            255 => {
                let offset_delta = read_u16(&bytes, &mut *offset);
                let number_of_locals = read_u16(&bytes, &mut *offset);
                let locals = VerificationTypeInfo::read(&bytes, &mut *offset, number_of_locals as u16);
                let number_of_stack_items = read_u16(&bytes, &mut *offset);
                let stack = VerificationTypeInfo::read(&bytes, &mut *offset, number_of_stack_items as u16);
                StackMapFrame::FullFrame {
                    frame_type,
                    offset_delta,
                    number_of_locals,
                    locals,
                    number_of_stack_items,
                    stack,
                }
            },
            _ => panic!()
        }
    }
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

#[allow(dead_code)]
impl VerificationTypeInfo {
    fn read(bytes: &[u8], offset: &mut usize, num_of_items: u16) -> Vec<VerificationTypeInfo> {
        let mut items: Vec<VerificationTypeInfo> = Vec::new();
        for _ in 0..num_of_items {
            let tag: u8 = read_u8(&bytes, &mut *offset);
            let item = match tag {
                0 => VerificationTypeInfo::TopVariableInfo { tag },
                1 => VerificationTypeInfo::IntegerVariableInfo { tag },
                2 => VerificationTypeInfo::FloatVariableInfo { tag },
                3 => VerificationTypeInfo::DoubleVariableInfo { tag },
                4 => VerificationTypeInfo::LongVariableInfo { tag },
                5 => VerificationTypeInfo::NullVariableInfo { tag },
                6 => VerificationTypeInfo::UninitializedThisVariableInfo { tag },
                7 => VerificationTypeInfo::ObjectVariableInfo {
                    tag,
                    cpool_index: read_u16(&bytes, &mut *offset),
                },
                8 => VerificationTypeInfo::UninitializedVariableInfo {
                    tag,
                    offset: read_u16(&bytes, &mut *offset),
                },
                _ => panic!()
            };
            items.push(item)
        }
        items
    }
}


#[derive(Debug, PartialEq)]
pub struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionTable {
    fn read(bytes: &[u8], offset: &mut usize) -> ExceptionTable {
        let start_pc: u16 = read_u16(&bytes, &mut *offset);
        let end_pc: u16 = read_u16(&bytes, &mut *offset);
        let handler_pc: u16 = read_u16(&bytes, &mut *offset);
        let catch_type: u16 = read_u16(&bytes, &mut *offset);
        ExceptionTable {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}

fn read_n<const N: usize>(bytes: &[u8], offset: &mut usize) -> [u8; N] {
    let next = *offset + N;
    let a: [u8; N] = bytes[*offset..next].try_into().unwrap();
    *offset = next;
    a
}

fn read_u8(bytes: &[u8], offset: &mut usize) -> u8 {
    u8::from_be_bytes(read_n::<1>(&bytes, &mut *offset))
}

fn read_u16(bytes: &[u8], offset: &mut usize) -> u16 {
    u16::from_be_bytes(read_n::<2>(&bytes, &mut *offset))
}

fn read_u32(bytes: &[u8], offset: &mut usize) -> u32 {
    u32::from_be_bytes(read_n::<4>(&bytes, &mut *offset))
}

fn read_u8_vec(bytes: &[u8], offset: &mut usize, length: usize) -> Vec<u8> {
    let next = *offset + length;
    let a: Vec<u8> = bytes[*offset..next].to_vec();
    *offset = next;
    a
}

#[test]
fn test() {
    let bytes: &[u8] = &[
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x13, 0x0a, 0x00, 0x02, 0x00, 0x03, 0x07,
        0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e,
        0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x0a, 0x00, 0x08, 0x00, 0x09, 0x07, 0x00,
        0x0a, 0x0c, 0x00, 0x0b, 0x00, 0x0c, 0x01, 0x00, 0x07, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x32,
        0x01, 0x00, 0x03, 0x61, 0x64, 0x64, 0x01, 0x00, 0x05, 0x28, 0x49, 0x49, 0x29, 0x49, 0x01, 0x00,
        0x04, 0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62,
        0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x01, 0x00, 0x04, 0x70, 0x72, 0x6f, 0x67, 0x01, 0x00,
        0x03, 0x28, 0x29, 0x49, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c,
        0x65, 0x01, 0x00, 0x0c, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x32, 0x2e, 0x6a, 0x61, 0x76, 0x61,
        0x00, 0x20, 0x00, 0x08, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x05,
        0x00, 0x06, 0x00, 0x01, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x1d, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x05, 0x2a, 0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x09, 0x00, 0x0f, 0x00, 0x10, 0x00, 0x01, 0x00,
        0x0d, 0x00, 0x00, 0x00, 0x31, 0x00, 0x02, 0x00, 0x03, 0x00, 0x00, 0x00, 0x0d, 0x04, 0x3b, 0x10,
        0x2a, 0x3c, 0x1a, 0x1b, 0xb8, 0x00, 0x07, 0x3d, 0x1c, 0xac, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0e,
        0x00, 0x00, 0x00, 0x12, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x02, 0x00, 0x05, 0x00, 0x05,
        0x00, 0x06, 0x00, 0x0b, 0x00, 0x07, 0x00, 0x09, 0x00, 0x0b, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x0d,
        0x00, 0x00, 0x00, 0x1c, 0x00, 0x02, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x1a, 0x1b, 0x60, 0xac,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0b,
        0x00, 0x01, 0x00, 0x11, 0x00, 0x00, 0x00, 0x02, 0x00, 0x12];

    let class_file = ClassFile::load(bytes);

    println!("{:#04x?}", class_file);

    assert_eq!(class_file, ClassFile {
        magic: [0xca, 0xfe, 0xba, 0xbe],
        minor_version: [0x00, 0x00],
        major_version: [0x00, 0x3e],
        constant_pool: ConstantPool {
            constant_pool_count: 19,
            cp_infos: vec![
                CpInfo::ConstantMethodrefInfo { tag: 10, class_index: 2, name_and_type_index: 3 },
                CpInfo::ConstantClassInfo { tag: 7, name_index: 4 },
                CpInfo::ConstantNameAndTypeInfo { tag: 12, name_index: 5, descriptor_index: 6 },
                CpInfo::ConstantUtf8Info { tag: 1, length: 16, bytes: vec![0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74] },
                CpInfo::ConstantUtf8Info { tag: 1, length: 6, bytes: vec![0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e] },
                CpInfo::ConstantUtf8Info { tag: 1, length: 3, bytes: vec![0x28, 0x29, 0x56] },
                CpInfo::ConstantMethodrefInfo { tag: 10, class_index: 8, name_and_type_index: 9 },
                CpInfo::ConstantClassInfo { tag: 7, name_index: 10 },
                CpInfo::ConstantNameAndTypeInfo { tag: 12, name_index: 11, descriptor_index: 12 },
                CpInfo::ConstantUtf8Info { tag: 1, length: 7, bytes: vec![0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x32] },
                CpInfo::ConstantUtf8Info { tag: 1, length: 3, bytes: vec![0x61, 0x64, 0x64] },
                CpInfo::ConstantUtf8Info { tag: 1, length: 5, bytes: vec![0x28, 0x49, 0x49, 0x29, 0x49] },
                CpInfo::ConstantUtf8Info { tag: 1, length: 4, bytes: vec![0x43, 0x6f, 0x64, 0x65] },
                CpInfo::ConstantUtf8Info { tag: 1, length: 15, bytes: vec![0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65] },
                CpInfo::ConstantUtf8Info { tag: 1, length: 4, bytes: vec![0x70, 0x72, 0x6f, 0x67] },
                CpInfo::ConstantUtf8Info { tag: 1, length: 3, bytes: vec![0x28, 0x29, 0x49] },
                CpInfo::ConstantUtf8Info { tag: 1, length: 10, bytes: vec![0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65] },
                CpInfo::ConstantUtf8Info { tag: 1, length: 12, bytes: vec![0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x32, 0x2e, 0x6a, 0x61, 0x76, 0x61] },
            ],
        },
        access_flags: 32,
        this_class: 8,
        super_class: 2,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 3,
        methods: vec![
            MethodInfo {
                access_flags: 0x00,
                name_index: 0x05,
                descriptor_index: 0x06,
                attributes_count: 0x01,
                attributes: vec![
                    CodeAttributeInfo {
                        attribute_name_index: 0x0d,
                        attribute_length: 0x1d,
                        max_stack: 0x01,
                        max_locals: 0x01,
                        code_length: 0x05,
                        code: vec![
                            0x2a,
                            0xb7,
                            0x00,
                            0x01,
                            0xb1,
                        ],
                        exception_table_length: 0x00,
                        exception_table: vec![],
                        attributes_count: 0x01,
                        attributes: vec![
                            AttributeInfo {
                                attribute_name_index: 0x0e,
                                attribute_length: 0x06,
                                info: vec![
                                    0x00,
                                    0x01,
                                    0x00,
                                    0x00,
                                    0x00,
                                    0x01,
                                ],
                            },
                        ],
                    }
                ],
            },
            MethodInfo {
                access_flags: 0x09,
                name_index: 0x0f,
                descriptor_index: 0x10,
                attributes_count: 0x01,
                attributes: vec![
                    CodeAttributeInfo {
                        attribute_name_index: 0x0d,
                        attribute_length: 0x31,
                        max_stack: 0x02,
                        max_locals: 0x03,
                        code_length: 0x0d,
                        code: vec![
                            0x04,
                            0x3b,
                            0x10,
                            0x2a,
                            0x3c,
                            0x1a,
                            0x1b,
                            0xb8,
                            0x00,
                            0x07,
                            0x3d,
                            0x1c,
                            0xac,
                        ],
                        exception_table_length: 0x00,
                        exception_table: vec![],
                        attributes_count: 0x01,
                        attributes: vec![
                            AttributeInfo {
                                attribute_name_index: 0x0e,
                                attribute_length: 0x12,
                                info: vec![
                                    0x00,
                                    0x04,
                                    0x00,
                                    0x00,
                                    0x00,
                                    0x04,
                                    0x00,
                                    0x02,
                                    0x00,
                                    0x05,
                                    0x00,
                                    0x05,
                                    0x00,
                                    0x06,
                                    0x00,
                                    0x0b,
                                    0x00,
                                    0x07,
                                ],
                            },
                        ],
                    }
                ],
            },
            MethodInfo {
                access_flags: 0x09,
                name_index: 0x0b,
                descriptor_index: 0x0c,
                attributes_count: 0x01,
                attributes: vec![
                    CodeAttributeInfo {
                        attribute_name_index: 0x0d,
                        attribute_length: 0x1c,
                        max_stack: 0x02,
                        max_locals: 0x02,
                        code_length: 0x04,
                        code: vec![
                            0x1a,
                            0x1b,
                            0x60,
                            0xac,
                        ],
                        exception_table_length: 0x00,
                        exception_table: vec![],
                        attributes_count: 0x01,
                        attributes: vec![
                            AttributeInfo {
                                attribute_name_index: 0x0e,
                                attribute_length: 0x06,
                                info: vec![
                                    0x00,
                                    0x01,
                                    0x00,
                                    0x00,
                                    0x00,
                                    0x0b,
                                ],
                            },
                        ],
                    }
                ],
            },
        ],
        attributes_count: 1,
        attributes: vec![
            AttributeInfo {
                attribute_name_index: 17,
                attribute_length: 2,
                info: vec![0x00, 0x12],
            }],
    })
}