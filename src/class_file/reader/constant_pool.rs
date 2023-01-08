use crate::class_file::structure::constant_pool::*;
use super::{Reader, VecReader, Result, error};

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
