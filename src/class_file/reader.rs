use super::structure::*;
use super::checker::Checker;

use thiserror::Error;


pub fn read_class_file(bytes: &[u8]) -> Result<ClassFile> {
    let mut offset: usize = 0;

    let class_file: ClassFile = Reader::read(&bytes, &mut offset)?;
    Ok(class_file)
}

pub type Result<T> = std::result::Result<T, ReadError>;

#[derive(Error, Debug, PartialEq)]
#[error("Failed to read class file. {message:} (at offset {offset} [byte])")]
pub struct ReadError {
    message: String,
    offset: usize,
}


trait Reader {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<Self> where Self: Sized;
}

trait VecReader {
    fn read(bytes: &[u8], offset: &mut usize, num_of_items: usize) -> Result<Vec<Self>> where Self: Sized;
}

impl<const N: usize> Reader for [u8; N] {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<[u8; N]> {
        let next = *offset + N;
        if bytes.len() >= next {
            let a: [u8; N] = bytes[*offset..next].try_into().unwrap();
            *offset = next;
            Ok(a)
        } else {
            Err(ReadError {
                message: "Input is shorter than required and cannot be read.".to_string(),
                offset: offset.clone(),
            })
        }
    }
}

impl Reader for u8 {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<Self> {
        let a: [u8; 1] = Reader::read(&bytes, &mut *offset)?;
        Ok(u8::from_be_bytes(a))
    }
}

impl Reader for u16 {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<Self> {
        let a: [u8; 2] = Reader::read(&bytes, &mut *offset)?;
        Ok(u16::from_be_bytes(a))
    }
}

impl Reader for u32 {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<Self> {
        let a: [u8; 4] = Reader::read(&bytes, &mut *offset)?;
        Ok(u32::from_be_bytes(a))
    }
}

impl<T> VecReader for T where T: Reader {
    fn read(bytes: &[u8], offset: &mut usize, num_of_items: usize) -> Result<Vec<T>> where Self: Sized {
        let mut items: Vec<T> = Vec::new();
        for _ in 0..num_of_items {
            items.push(T::read(&bytes, &mut *offset)?);
        };
        Ok(items)
    }
}

// utils
fn error<T>(message: String, offset: &mut usize) -> Result<T> {
    Err(ReadError { message, offset: offset.clone() })
}

impl Reader for Magic {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<Self> where Self: Sized {
        let value: [u8; 4] = Reader::read(&bytes, &mut *offset)?;
        Ok(Magic { value })
    }
}

impl Reader for Version {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<Self> where Self: Sized {
        let minor_version: u16 = Reader::read(&bytes, &mut *offset)?;
        let major_version: u16 = Reader::read(&bytes, &mut *offset)?;
        Ok(Version { minor_version, major_version })
    }
}

impl Reader for ClassFile {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ClassFile> {
        let magic: Magic = Reader::read(&bytes, &mut *offset)?;

        // check the magic item `cafebabe` at the first early.
        magic.check().map_err(|e| ReadError { message: e.message, offset: offset.clone() })?;

        let version: Version = Reader::read(&bytes, &mut *offset)?;
        // check the class file version early.
        version.check().map_err(|e| ReadError { message: e.message, offset: offset.clone() })?;

        // The rest of the checking done by the class file reader is only checking
        // whether all the bytes at the end have been consumed, and the rest is left to ClassFileChecker

        let constant_pool: ConstantPool = Reader::read(&bytes, &mut *offset)?;

        let access_flags: u16 = Reader::read(&bytes, &mut *offset)?;

        let this_class: u16 = Reader::read(&bytes, &mut *offset)?;

        let super_class: u16 = Reader::read(&bytes, &mut *offset)?;

        let interfaces_count: u16 = Reader::read(&bytes, &mut *offset)?;

        let interfaces: Vec<u16> = VecReader::read(&bytes, &mut *offset, interfaces_count as usize)?;

        let fields_count: u16 = Reader::read(&bytes, &mut *offset)?;

        let fields: Vec<FieldsInfo> = VecReader::read(&bytes, &mut *offset, fields_count as usize)?;

        let methods_count: u16 = Reader::read(&bytes, &mut *offset)?;

        let methods: Vec<MethodInfo> = VecReader::read(&bytes, &mut *offset, methods_count as usize)?;

        let attributes_count: u16 = Reader::read(&bytes, &mut *offset)?;

        let attributes: Vec<AttributeInfo> = VecReader::read(&bytes, &mut *offset, attributes_count as usize)?;

        // 4.8. Format Checking
        // The class file must not be truncated or have extra bytes at the end.
        if bytes.len() != *offset {
            return error("Too many bytes after reading class file.".to_string(), offset);
        }

        Ok(ClassFile {
            magic,
            version,
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
        })
    }
}

impl Reader for ConstantPool {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantPool> {
        let constant_pool_count: u16 = Reader::read(&bytes, &mut *offset)?;
        // The constant_pool table is indexed from 1 to constant_pool_count - 1.
        // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
        let cp_infos: Vec<CpInfo> = VecReader::read(&bytes, &mut *offset, (constant_pool_count - 1) as usize)?;
        Ok(ConstantPool {
            constant_pool_count,
            cp_infos,
        })
    }
}

impl Reader for CpInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<CpInfo> {
        let tag: CpInfoTag = Reader::read(&bytes, &mut *offset)?;
        let cp_info = match tag {
            CONSTANT_UTF8 => {
                let length: u16 = Reader::read(&bytes, &mut *offset)?;
                CpInfo::ConstantUtf8Info {
                    tag,
                    length,
                    bytes: VecReader::read(&bytes, &mut *offset, length as usize)?,
                }
            }
            CONSTANT_INTEGER => {
                CpInfo::ConstantIntegerInfo {
                    tag,
                    bytes: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_FLOAT => {
                CpInfo::ConstantFloatInfo {
                    tag,
                    bytes: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_LONG => {
                CpInfo::ConstantLongInfo {
                    tag,
                    high_bytes: Reader::read(&bytes, &mut *offset)?,
                    low_bytes: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_DOUBLE => {
                CpInfo::ConstantDoubleInfo {
                    tag,
                    high_bytes: Reader::read(&bytes, &mut *offset)?,
                    low_bytes: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_CLASS => {
                CpInfo::ConstantClassInfo {
                    tag,
                    name_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_STRING => {
                CpInfo::ConstantStringInfo {
                    tag,
                    string_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_FIELDREF => {
                CpInfo::ConstantFieldrefInfo {
                    tag,
                    class_index: Reader::read(&bytes, &mut *offset)?,
                    name_and_type_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_METHODREF => {
                CpInfo::ConstantMethodrefInfo {
                    tag,
                    class_index: Reader::read(&bytes, &mut *offset)?,
                    name_and_type_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_INTERFACE_METHODREF => {
                CpInfo::ConstantInterfaceMethodrefInfo {
                    tag,
                    class_index: Reader::read(&bytes, &mut *offset)?,
                    name_and_type_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_NAME_AND_TYPE => {
                CpInfo::ConstantNameAndTypeInfo {
                    tag,
                    name_index: Reader::read(&bytes, &mut *offset)?,
                    descriptor_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_METHOD_HANDLE => {
                CpInfo::ConstantMethodHandleInfo {
                    tag,
                    reference_kind: Reader::read(&bytes, &mut *offset)?,
                    reference_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_METHOD_TYPE => {
                CpInfo::ConstantMethodTypeInfo {
                    tag,
                    descriptor_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_DYNAMIC => {
                CpInfo::ConstantDynamicInfo {
                    tag,
                    bootstrap_method_attr_index: Reader::read(&bytes, &mut *offset)?,
                    name_and_type_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_INVOKE_DYNAMIC => {
                CpInfo::ConstantInvokeDynamicInfo {
                    tag,
                    bootstrap_method_attr_index: Reader::read(&bytes, &mut *offset)?,
                    name_and_type_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_MODULE => {
                CpInfo::ConstantModuleInfo {
                    tag,
                    name_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            CONSTANT_PACKAGE => {
                CpInfo::ConstantPackageInfo {
                    tag,
                    name_index: Reader::read(&bytes, &mut *offset)?,
                }
            }
            _ => return error(format!("unsupported tag {}", tag), offset)
        };
        Ok(cp_info)
    }
}

impl Reader for CpRef {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<Self> where Self: Sized {
        let value: u16 = Reader::read(&bytes, &mut *offset)?;
        Ok(CpRef { value })
    }
}

impl Reader for CpUtf8Ref {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<Self> where Self: Sized {
        let value: u16 = Reader::read(&bytes, &mut *offset)?;
        Ok(CpUtf8Ref { value })
    }
}

impl Reader for FieldsInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<FieldsInfo> {
        let access_flags: u16 = Reader::read(&bytes, &mut *offset)?;
        let name_index: CpUtf8Ref = Reader::read(&bytes, &mut *offset)?;
        let descriptor_index: u16 = Reader::read(&bytes, &mut *offset)?;
        let attributes_count: u16 = Reader::read(&bytes, &mut *offset)?;
        let attributes: Vec<AttributeInfo> = VecReader::read(&bytes, &mut *offset, attributes_count as usize)?;
        Ok(FieldsInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

impl Reader for MethodInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<MethodInfo> {
        let access_flags: u16 = Reader::read(&bytes, &mut *offset)?;
        let name_index: CpUtf8Ref = Reader::read(&bytes, &mut *offset)?;
        let descriptor_index: u16 = Reader::read(&bytes, &mut *offset)?;
        let attributes_count: u16 = Reader::read(&bytes, &mut *offset)?;
        let attributes: Vec<CodeAttributeInfo> = VecReader::read(&bytes, &mut *offset, attributes_count as usize)?;
        Ok(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

impl Reader for AttributeInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<AttributeInfo> {
        let attribute_name_index: CpUtf8Ref = Reader::read(&bytes, &mut *offset)?;
        let attribute_length: u32 = Reader::read(&bytes, &mut *offset)?;
        let info: Vec<u8> = VecReader::read(&bytes, &mut *offset, attribute_length as usize)?;
        Ok(AttributeInfo {
            attribute_name_index,
            attribute_length,
            info,
        })
    }
}

impl Reader for CodeAttributeInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<CodeAttributeInfo> {
        let attribute_name_index: CpUtf8Ref = Reader::read(&bytes, &mut *offset)?;
        let attribute_length: u32 = Reader::read(&bytes, &mut *offset)?;
        let max_stack: u16 = Reader::read(&bytes, &mut *offset)?;
        let max_locals: u16 = Reader::read(&bytes, &mut *offset)?;
        let code_length: u32 = Reader::read(&bytes, &mut *offset)?;
        let code: Vec<u8> = VecReader::read(&bytes, &mut *offset, code_length as usize)?;
        let exception_table_length: u16 = Reader::read(&bytes, &mut *offset)?;
        let exception_table: Vec<ExceptionTable> = VecReader::read(&bytes, &mut *offset, exception_table_length as usize)?;
        let attributes_count: u16 = Reader::read(&bytes, &mut *offset)?;
        let attributes: Vec<AttributeInfo> = VecReader::read(&bytes, &mut *offset, attributes_count as usize)?;
        Ok(CodeAttributeInfo {
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
        })
    }
}


#[allow(dead_code)]
impl Reader for StackMapTableAttribute {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<StackMapTableAttribute> {
        let attribute_name_index: CpUtf8Ref = Reader::read(&bytes, &mut *offset)?;
        let attribute_length: u32 = Reader::read(&bytes, &mut *offset)?;
        let number_of_entries: u16 = Reader::read(&bytes, &mut *offset)?;
        let entries: Vec<StackMapFrame> = VecReader::read(&bytes, &mut *offset, number_of_entries as usize)?;
        Ok(StackMapTableAttribute {
            attribute_name_index,
            attribute_length,
            number_of_entries,
            entries,
        })
    }
}

#[allow(dead_code)]
impl Reader for StackMapFrame {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<StackMapFrame> {
        let frame_type: u8 = Reader::read(&bytes, &mut *offset)?;
        let stack_map_frame = match frame_type {
            0..=63 => StackMapFrame::SameFrame { frame_type },
            64..=127 => StackMapFrame::SameLocals1StackItemFrame {
                frame_type,
                stack: VecReader::read(&bytes, &mut *offset, 1)?,
            },
            247 => StackMapFrame::SameLocals1StackItemFrameExtended {
                frame_type,
                offset_delta: Reader::read(&bytes, &mut *offset)?,
                stack: VecReader::read(&bytes, &mut *offset, 1)?,
            },
            248..=250 => StackMapFrame::ChopFrame {
                frame_type,
                offset_delta: Reader::read(&bytes, &mut *offset)?,
            },
            251 => StackMapFrame::SameFrameExtended {
                frame_type,
                offset_delta: Reader::read(&bytes, &mut *offset)?,
            },
            252..=254 => StackMapFrame::AppendFrame {
                frame_type,
                offset_delta: Reader::read(&bytes, &mut *offset)?,
                locals: VecReader::read(&bytes, &mut *offset, (frame_type - 251) as usize)?,
            },
            255 => {
                let offset_delta = Reader::read(&bytes, &mut *offset)?;
                let number_of_locals = Reader::read(&bytes, &mut *offset)?;
                let locals = VecReader::read(&bytes, &mut *offset, number_of_locals as u16 as usize)?;
                let number_of_stack_items = Reader::read(&bytes, &mut *offset)?;
                let stack = VecReader::read(&bytes, &mut *offset, number_of_stack_items as u16 as usize)?;
                StackMapFrame::FullFrame {
                    frame_type,
                    offset_delta,
                    number_of_locals,
                    locals,
                    number_of_stack_items,
                    stack,
                }
            }
            _ => return error(format!("invalid stack frame type! type: {}", frame_type), offset)
        };
        Ok(stack_map_frame)
    }
}

#[allow(dead_code)]
impl Reader for VerificationTypeInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<Self> where Self: Sized {
        let tag: u8 = Reader::read(&bytes, &mut *offset)?;
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
                cpool_index: Reader::read(&bytes, &mut *offset)?,
            },
            8 => VerificationTypeInfo::UninitializedVariableInfo {
                tag,
                offset: Reader::read(&bytes, &mut *offset)?,
            },
            _ => return error(format!("Verification type's tag must be 0..8 !. tag: {}", tag), offset)
        };
        Ok(item)
    }
}

impl Reader for ExceptionTable {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ExceptionTable> {
        let start_pc: u16 = Reader::read(&bytes, &mut *offset)?;
        let end_pc: u16 = Reader::read(&bytes, &mut *offset)?;
        let handler_pc: u16 = Reader::read(&bytes, &mut *offset)?;
        let catch_type: u16 = Reader::read(&bytes, &mut *offset)?;
        Ok(ExceptionTable {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }
}


#[test]
fn test() {
    let bytes: &[u8] = &[
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x13, 0x0a, 0x00, 0x02, 0x00, 0x03, 0x07,
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

    let class_file = read_class_file(bytes);

    // println!("{:#04x?}", class_file);

    assert_eq!(class_file, Ok(ClassFile {
        magic: Magic { value: [0xca, 0xfe, 0xba, 0xbe] },
        version: Version {
            minor_version: 0,
            major_version: 61,
        },
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
                name_index: CpUtf8Ref { value: 0x05 },
                descriptor_index: 0x06,
                attributes_count: 0x01,
                attributes: vec![
                    CodeAttributeInfo {
                        attribute_name_index: CpUtf8Ref { value: 0x0d },
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
                                attribute_name_index:CpUtf8Ref { value: 0x0e },
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
                name_index: CpUtf8Ref { value: 0x0f },
                descriptor_index: 0x10,
                attributes_count: 0x01,
                attributes: vec![
                    CodeAttributeInfo {
                        attribute_name_index: CpUtf8Ref { value: 0x0d },
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
                                attribute_name_index: CpUtf8Ref { value: 0x0e },
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
                name_index: CpUtf8Ref { value: 0x0b },
                descriptor_index: 0x0c,
                attributes_count: 0x01,
                attributes: vec![
                    CodeAttributeInfo {
                        attribute_name_index: CpUtf8Ref { value: 0x0d },
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
                                attribute_name_index: CpUtf8Ref { value: 0x0e },
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
                attribute_name_index: CpUtf8Ref { value: 17 },
                attribute_length: 2,
                info: vec![0x00, 0x12],
            }],
    }))
}