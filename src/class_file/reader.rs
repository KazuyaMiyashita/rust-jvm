use super::structure::*;
use super::structure::cp_accessor::*;
use super::error::{Error, Result};
use super::checker;

pub fn read_class_file(bytes: Vec<u8>) -> Result<ClassFile> {
    let class_file: ClassFile = Reader::read(&bytes, &mut (0 as usize))?;
    Ok(class_file)
}

trait Reader {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<Self> where Self: Sized;
}

trait VecReader {
    fn read(bytes: &[u8], offset: &mut usize, num_of_items: usize) -> Result<Vec<Self>> where Self: Sized;
}

trait ReaderWithCp {
    fn read(bytes: &[u8], offset: &mut usize, constant_pool: &Vec<CpInfo>) -> Result<Self> where Self: Sized;
}

trait VecReaderWithCp {
    fn read(bytes: &[u8], offset: &mut usize, constant_pool: &Vec<CpInfo>, num_of_items: usize) -> Result<Vec<Self>> where Self: Sized;
}

fn error<T>(message: String, offset: &mut usize) -> Result<T> {
    Err(Error { message: format!("{}, offset: {}", message, offset.clone()) })
}


impl<const N: usize> Reader for [u8; N] {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<[u8; N]> {
        let next = *offset + N;
        if bytes.len() >= next {
            let a: [u8; N] = bytes[*offset..next].try_into().unwrap();
            *offset = next;
            Ok(a)
        } else {
            error("Input is shorter than required and cannot be read.".to_string(), offset)
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

impl<T> VecReaderWithCp for T where T: ReaderWithCp {
    fn read(bytes: &[u8], offset: &mut usize, constant_pool: &Vec<CpInfo>, num_of_items: usize) -> Result<Vec<T>> where Self: Sized {
        let mut items: Vec<T> = Vec::new();
        for _ in 0..num_of_items {
            items.push(T::read(&bytes, &mut *offset, constant_pool)?);
        };
        Ok(items)
    }
}

impl Reader for ClassFile {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ClassFile> {
        let magic: [u8; 4] = Reader::read(&bytes, &mut *offset)?;
        // check the magic item `cafebabe` at the first early.
        checker::check_magic(&magic).or_else(|e| error(e.message, offset))?;
        let minor_version: u16 = Reader::read(&bytes, &mut *offset)?;
        let major_version: u16 = Reader::read(&bytes, &mut *offset)?;
        // check the class file version early.
        checker::check_version(minor_version, major_version).or_else(|e| error(e.message, offset))?;
        // The rest of the checking done by the class file reader is only checking
        // whether all the bytes at the end have been consumed, and the rest is left to ClassFileChecker
        let constant_pool_count: u16 = Reader::read(&bytes, &mut *offset)?;
        // The constant_pool table is indexed from 1 to constant_pool_count - 1.
        // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
        let constant_pool: Vec<CpInfo> = VecReader::read(&bytes, &mut *offset, (constant_pool_count - 1) as usize)?;
        checker::check_constant_pool(&constant_pool, major_version)?;
        let access_flags: u16 = Reader::read(&bytes, &mut *offset)?;
        let this_class: u16 = Reader::read(&bytes, &mut *offset)?;
        let super_class: u16 = Reader::read(&bytes, &mut *offset)?;
        let interfaces_count: u16 = Reader::read(&bytes, &mut *offset)?;
        let interfaces: Vec<u16> = VecReader::read(&bytes, &mut *offset, interfaces_count as usize)?;
        let fields_count: u16 = Reader::read(&bytes, &mut *offset)?;
        let fields: Vec<FieldsInfo> = VecReaderWithCp::read(&bytes, &mut *offset, &constant_pool,fields_count as usize)?;
        let methods_count: u16 = Reader::read(&bytes, &mut *offset)?;
        let methods: Vec<MethodInfo> = VecReaderWithCp::read(&bytes, &mut *offset, &constant_pool,methods_count as usize)?;
        let attributes_count: u16 = Reader::read(&bytes, &mut *offset)?;
        let attributes: Vec<Attribute> = VecReaderWithCp::read(&bytes, &mut *offset, &constant_pool, attributes_count as usize)?;

        // 4.8. Format Checking
        // The class file must not be truncated or have extra bytes at the end.
        if bytes.len() != *offset {
            return error(format!("Too many bytes after reading class file. {}  bytes remaining.", bytes.len() - *offset), offset);
        }

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
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

impl Reader for ConstantUtf8Info {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantUtf8Info> {
        let length = Reader::read(&bytes, &mut *offset)?;
        Ok(ConstantUtf8Info {
            tag: CONSTANT_UTF8,
            length,
            bytes: VecReader::read(&bytes, &mut *offset, length as usize)?,
        })
    }
}

impl Reader for ConstantIntegerInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantIntegerInfo> {
        Ok(ConstantIntegerInfo {
            tag: CONSTANT_INTEGER,
            bytes: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantFloatInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantFloatInfo> {
        Ok(ConstantFloatInfo {
            tag: CONSTANT_FLOAT,
            bytes: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantLongInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantLongInfo> {
        Ok(ConstantLongInfo {
            tag: CONSTANT_LONG,
            high_bytes: Reader::read(&bytes, &mut *offset)?,
            low_bytes: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantDoubleInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantDoubleInfo> {
        Ok(ConstantDoubleInfo {
            tag: CONSTANT_DOUBLE,
            high_bytes: Reader::read(&bytes, &mut *offset)?,
            low_bytes: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantClassInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantClassInfo> {
        Ok(ConstantClassInfo {
            tag: CONSTANT_CLASS,
            name_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantStringInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantStringInfo> {
        Ok(ConstantStringInfo {
            tag: CONSTANT_STRING,
            string_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantFieldrefInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantFieldrefInfo> {
        Ok(ConstantFieldrefInfo {
            tag: CONSTANT_FIELDREF,
            class_index: Reader::read(&bytes, &mut *offset)?,
            name_and_type_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantMethodrefInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantMethodrefInfo> {
        Ok(ConstantMethodrefInfo {
            tag: CONSTANT_METHODREF,
            class_index: Reader::read(&bytes, &mut *offset)?,
            name_and_type_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantInterfaceMethodrefInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantInterfaceMethodrefInfo> {
        Ok(ConstantInterfaceMethodrefInfo {
            tag: CONSTANT_INTERFACE_METHODREF,
            class_index: Reader::read(&bytes, &mut *offset)?,
            name_and_type_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantNameAndTypeInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantNameAndTypeInfo> {
        Ok(ConstantNameAndTypeInfo {
            tag: CONSTANT_NAME_AND_TYPE,
            name_index: Reader::read(&bytes, &mut *offset)?,
            descriptor_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantMethodHandleInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantMethodHandleInfo> {
        Ok(ConstantMethodHandleInfo {
            tag: CONSTANT_METHOD_HANDLE,
            reference_kind: Reader::read(&bytes, &mut *offset)?,
            reference_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantMethodTypeInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantMethodTypeInfo> {
        Ok(ConstantMethodTypeInfo {
            tag: CONSTANT_METHOD_TYPE,
            descriptor_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantDynamicInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantDynamicInfo> {
        Ok(ConstantDynamicInfo {
            tag: CONSTANT_DYNAMIC,
            bootstrap_method_attr_index: Reader::read(&bytes, &mut *offset)?,
            name_and_type_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantInvokeDynamicInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantInvokeDynamicInfo> {
        Ok(ConstantInvokeDynamicInfo {
            tag: CONSTANT_INVOKE_DYNAMIC,
            bootstrap_method_attr_index: Reader::read(&bytes, &mut *offset)?,
            name_and_type_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for ConstantModuleInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantModuleInfo> {
        Ok(ConstantModuleInfo {
            tag: CONSTANT_MODULE,
            name_index: Reader::read(&bytes, &mut *offset)?,

        })
    }
}

impl Reader for ConstantPackageInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<ConstantPackageInfo> {
        Ok(ConstantPackageInfo {
            tag: CONSTANT_PACKAGE,
            name_index: Reader::read(&bytes, &mut *offset)?,
        })
    }
}

impl Reader for CpInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<CpInfo> {
        let tag: CpInfoTag = Reader::read(&bytes, &mut *offset)?;
        let cp_info = match tag {
            CONSTANT_UTF8 => CpInfo::Utf8(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_INTEGER => CpInfo::Integer(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_FLOAT => CpInfo::Float(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_LONG => CpInfo::Long(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_DOUBLE => CpInfo::Double(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_CLASS => CpInfo::Class(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_STRING => CpInfo::String(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_FIELDREF => CpInfo::Fieldref(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_METHODREF => CpInfo::Methodref(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_INTERFACE_METHODREF => CpInfo::InterfaceMethodref(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_NAME_AND_TYPE => CpInfo::NameAndType(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_METHOD_HANDLE => CpInfo::MethodHandle(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_METHOD_TYPE => CpInfo::MethodType(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_DYNAMIC => CpInfo::Dynamic(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_INVOKE_DYNAMIC => CpInfo::InvokeDynamic(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_MODULE => CpInfo::Module(Reader::read(&bytes, &mut *offset)?),
            CONSTANT_PACKAGE => CpInfo::Package(Reader::read(&bytes, &mut *offset)?),
            _ => return error(format!("unsupported tag {}", tag), offset)
        };
        Ok(cp_info)
    }
}

impl ReaderWithCp for FieldsInfo {
    fn read(bytes: &[u8], offset: &mut usize, constant_pool: &Vec<CpInfo>) -> Result<FieldsInfo> {
        let access_flags: u16 = Reader::read(&bytes, &mut *offset)?;
        let name_index: u16 = Reader::read(&bytes, &mut *offset)?;
        let descriptor_index: u16 = Reader::read(&bytes, &mut *offset)?;
        let attributes_count: u16 = Reader::read(&bytes, &mut *offset)?;
        let attributes: Vec<Attribute> = VecReaderWithCp::read(&bytes, &mut *offset, constant_pool, attributes_count as usize)?;
        Ok(FieldsInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

impl ReaderWithCp for MethodInfo {
    fn read(bytes: &[u8], offset: &mut usize, constant_pool: &Vec<CpInfo>) -> Result<MethodInfo> {
        let access_flags: u16 = Reader::read(&bytes, &mut *offset)?;
        let name_index: u16 = Reader::read(&bytes, &mut *offset)?;
        let descriptor_index: u16 = Reader::read(&bytes, &mut *offset)?;
        let attributes_count: u16 = Reader::read(&bytes, &mut *offset)?;
        let attributes: Vec<Attribute> = VecReaderWithCp::read(&bytes, &mut *offset,constant_pool, attributes_count as usize)?;
        Ok(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

impl ReaderWithCp for Attribute {
    fn read(bytes: &[u8], offset: &mut usize, constant_pool: &Vec<CpInfo>) -> Result<Attribute> {
        let attribute_name_index: u16 = Reader::read(&bytes, &mut *offset)?;
        let attribute_length: u32 = Reader::read(&bytes, &mut *offset)?;
        let attribute_name = constant_pool.access_as_utf8(attribute_name_index).bytes_as_string()?;
        let attribute = match attribute_name.as_str() {
            "ConstantValue" => {
                let constantvalue_index: u16 = Reader::read(&bytes, &mut *offset)?;
                Attribute::ConstantValue(ConstantValueAttribute{
                    attribute_name_index,
                    attribute_length,
                    constantvalue_index
                })
            },
            "Code" => {
                let max_stack: u16 = Reader::read(&bytes, &mut *offset)?;
                let max_locals: u16 = Reader::read(&bytes, &mut *offset)?;
                let code_length: u32 = Reader::read(&bytes, &mut *offset)?;
                let code: Vec<u8> = VecReader::read(&bytes, &mut *offset, code_length as usize)?;
                let exception_table_length: u16 = Reader::read(&bytes, &mut *offset)?;
                let exception_table: Vec<ExceptionTable> = VecReader::read(&bytes, &mut *offset, exception_table_length as usize)?;
                let attributes_count: u16 = Reader::read(&bytes, &mut *offset)?;
                let attributes: Vec<Attribute> = VecReaderWithCp::read(&bytes, &mut *offset, &constant_pool, attributes_count as usize)?;
                Attribute::Code(CodeAttributeInfo {
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
            },
            "StackMapTable" => {
                let number_of_entries: u16 = Reader::read(&bytes, &mut *offset)?;
                let entries: Vec<StackMapFrame> = VecReader::read(&bytes, &mut *offset, number_of_entries as usize)?;
                Attribute::StackMapTable(StackMapTableAttribute {
                    attribute_name_index,
                    attribute_length,
                    number_of_entries,
                    entries,
                })
            },
            "BootstrapMethods" => {
                let num_bootstrap_methods: u16 = Reader::read(&bytes, &mut *offset)?;
                let bootstrap_methods: Vec<BootstrapMethod> = VecReader::read(&bytes, &mut *offset, num_bootstrap_methods as usize)?;
                Attribute::BootstrapMethods(BootstrapMethodsAttribute {
                    attribute_name_index,
                    attribute_length,
                    num_bootstrap_methods,
                    bootstrap_methods,
                })
            },
            _ => {
                let info: Vec<u8> = VecReader::read(&bytes, &mut *offset, attribute_length as usize)?;
                Attribute::General(AttributeInfo {
                    attribute_name_index,
                    attribute_length,
                    info,
                })
            }
        };
        Ok(attribute)
    }
}


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

impl Reader for BootstrapMethod {
    fn read(bytes: &[u8], offset: &mut usize) -> Result<BootstrapMethod> {
        let bootstrap_method_ref: u16 = Reader::read(&bytes, &mut *offset)?;
        let num_bootstrap_arguments: u16 = Reader::read(&bytes, &mut *offset)?;
        let bootstrap_arguments: Vec<u16> = VecReader::read(&bytes, &mut *offset, num_bootstrap_arguments as u16 as usize)?;
        Ok(BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        })
    }
}