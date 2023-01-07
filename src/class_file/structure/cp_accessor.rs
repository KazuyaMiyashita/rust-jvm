use super::*;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AccessError>;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("Invalid class file. {message:}")]
pub struct AccessError {
    pub message: String,
}

// utils
fn error<T>(message: String) -> Result<T> {
    Err(AccessError { message })
}

fn cp_info_name(cp_info: &CpInfo) -> &str {
    match cp_info {
        CpInfo::Utf8(_) => "CONSTANT_Utf8",
        CpInfo::Integer(_) => "CONSTANT_Integer",
        CpInfo::Float(_) => "CONSTANT_Float",
        CpInfo::Long(_) => "CONSTANT_Long",
        CpInfo::Double(_) => "CONSTANT_Double",
        CpInfo::Class(_) => "CONSTANT_Class",
        CpInfo::String(_) => "CONSTANT_String",
        CpInfo::Fieldref(_) => "CONSTANT_Fieldref",
        CpInfo::Methodref(_) => "CONSTANT_Methodref",
        CpInfo::InterfaceMethodref(_) => "CONSTANT_InterfaceMethodref",
        CpInfo::NameAndType(_) => "CONSTANT_NameAndType",
        CpInfo::MethodHandle(_) => "CONSTANT_MethodHandle",
        CpInfo::MethodType(_) => "CONSTANT_MethodType",
        CpInfo::Dynamic(_) => "CONSTANT_Dynamic",
        CpInfo::InvokeDynamic(_) => "CONSTANT_InvokeDynamic",
        CpInfo::Module(_) => "CONSTANT_Module",
        CpInfo::Package(_) => "CONSTANT_Package",
    }
}

// original constant_pool table is indexed from 1 to constant_pool_count - 1.
// Note that the Vec of this cp_infos structure is indexed from 0.
fn get_constant_pool_info(constant_pool: &Vec<CpInfo>, index: u16) -> Result<&CpInfo> {
    match constant_pool.get((index as usize) - 1) {
        Some(cp_info) => Ok(cp_info),
        None => error(format!("the index of constant_pool not found! index: {}", index)),
    }
}

pub trait CpAccessor {
    // indexed from 1 to constant_pool_count - 1.
    fn access_as_utf8(&self, index: u16) -> Utf8CpAccessor;
    fn access_as_integer(&self, index: u16) -> IntegerCpAccessor;
    fn access_as_float(&self, index: u16) -> FloatCpAccessor;
    fn access_as_double(&self, index: u16) -> DoubleCpAccessor;
    fn access_as_class(&self, index: u16) -> ClassCpAccessor;
    fn access_as_fieldref(&self, index: u16) -> FieldrefCpAccessor;
    fn access_as_methodref(&self, index: u16) -> MethodrefCpAccessor;
    fn access_as_interface_methodref(&self, index: u16) -> InterfaceMethodrefCpAccessor;
    fn access_as_name_and_type(&self, index: u16) -> NameAndTypeCpAccessor;
    fn access_as_method_handle(&self, index: u16) -> MethodHandleCpAccessor;
    fn access_as_method_type(&self, index: u16) -> MethodTypeCpAccessor;
    fn access_as_dynamic(&self, index: u16) -> DynamicCpAccessor;
    fn access_as_invoke_dynamic(&self, index: u16) -> InvokeDynamicCpAccessor;
    fn access_as_module(&self, index: u16) -> ModuleCpAccessor;
    fn access_as_package(&self, index: u16) -> PackageCpAccessor;
}

impl CpAccessor for &Vec<CpInfo> {
    fn access_as_utf8(&self, index: u16) -> Utf8CpAccessor {
        Utf8CpAccessor::from(self, index)
    }

    fn access_as_integer(&self, index: u16) -> IntegerCpAccessor {
        IntegerCpAccessor::from(self, index)
    }

    fn access_as_float(&self, index: u16) -> FloatCpAccessor {
        FloatCpAccessor::from(self, index)
    }

    fn access_as_double(&self, index: u16) -> DoubleCpAccessor {
        DoubleCpAccessor::from(self, index)
    }

    fn access_as_class(&self, index: u16) -> ClassCpAccessor {
        ClassCpAccessor::from(self, index)
    }

    fn access_as_fieldref(&self, index: u16) -> FieldrefCpAccessor {
        FieldrefCpAccessor::from(self, index)
    }

    fn access_as_methodref(&self, index: u16) -> MethodrefCpAccessor {
        MethodrefCpAccessor::from(self, index)
    }

    fn access_as_interface_methodref(&self, index: u16) -> InterfaceMethodrefCpAccessor {
        InterfaceMethodrefCpAccessor::from(self, index)
    }

    fn access_as_name_and_type(&self, index: u16) -> NameAndTypeCpAccessor {
        NameAndTypeCpAccessor::from(self, index)
    }

    fn access_as_method_handle(&self, index: u16) -> MethodHandleCpAccessor {
        MethodHandleCpAccessor::from(self, index)
    }

    fn access_as_method_type(&self, index: u16) -> MethodTypeCpAccessor {
        MethodTypeCpAccessor::from(self, index)
    }

    fn access_as_dynamic(&self, index: u16) -> DynamicCpAccessor {
        DynamicCpAccessor::from(self, index)
    }

    fn access_as_invoke_dynamic(&self, index: u16) -> InvokeDynamicCpAccessor {
        InvokeDynamicCpAccessor::from(self, index)
    }

    fn access_as_module(&self, index: u16) -> ModuleCpAccessor {
        ModuleCpAccessor::from(self, index)
    }

    fn access_as_package(&self, index: u16) -> PackageCpAccessor {
        PackageCpAccessor::from(self, index)
    }
}

pub struct Utf8CpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantUtf8Info>,
}

impl Utf8CpAccessor<'_> {
    fn bytes_as_string(&self) -> Result<String> {
        match &self.info_or_err {
            Ok(info) => String::from_utf8(info.bytes.clone()).or_else(|e| error(e.to_string())),
            Err(e) => Err(e.to_owned())
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> Utf8CpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Utf8(info)) => Utf8CpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => Utf8CpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_Utf8_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => Utf8CpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> Utf8CpAccessor<'a> {
        Utf8CpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct IntegerCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantIntegerInfo>,
}

impl IntegerCpAccessor<'_> {
    fn bytes_as_integer(&self) -> Result<i32> {
        match &self.info_or_err {
            Ok(info) => Ok(i32::from_be_bytes(info.bytes)),
            Err(e) => Err(e.to_owned())
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> IntegerCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Integer(info)) => IntegerCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => IntegerCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_Integer_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => IntegerCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> IntegerCpAccessor<'a> {
        IntegerCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct FloatCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantFloatInfo>,
}

impl FloatCpAccessor<'_> {
    fn bytes_as_float(&self) -> Result<f32> {
        match &self.info_or_err {
            Ok(info) => Ok(f32::from_be_bytes(info.bytes)),
            Err(e) => Err(e.to_owned())
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> FloatCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Float(info)) => FloatCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => FloatCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_Float_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => FloatCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> FloatCpAccessor<'a> {
        FloatCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct LongCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantLongInfo>,
}

impl LongCpAccessor<'_> {
    fn bytes_as_long(&self) -> Result<i64> {
        match &self.info_or_err {
            Ok(info) => Ok(i64::from_be_bytes([info.high_bytes, info.low_bytes].concat().try_into().unwrap())),
            Err(e) => Err(e.to_owned())
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> LongCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Long(info)) => LongCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => LongCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_Long_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => LongCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> LongCpAccessor<'a> {
        LongCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct DoubleCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantDoubleInfo>,
}

impl DoubleCpAccessor<'_> {
    fn bytes_as_double(&self) -> Result<f64> {
        match &self.info_or_err {
            Ok(info) => Ok(f64::from_be_bytes([info.high_bytes, info.low_bytes].concat().try_into().unwrap())),
            Err(e) => Err(e.to_owned())
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> DoubleCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Double(info)) => DoubleCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => DoubleCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_Double_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => DoubleCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> DoubleCpAccessor<'a> {
        DoubleCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct ClassCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantClassInfo>,
}

impl ClassCpAccessor<'_> {
    fn name(&self) -> Utf8CpAccessor {
        match &self.info_or_err {
            Ok(info) => Utf8CpAccessor::from(self.constant_pool, info.name_index),
            Err(e) => Utf8CpAccessor::error(self.constant_pool, e)
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> ClassCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Class(info)) => ClassCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => ClassCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_Class_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => ClassCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> ClassCpAccessor<'a> {
        ClassCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct StringCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantClassInfo>,
}

impl StringCpAccessor<'_> {
    fn name(&self) -> Utf8CpAccessor {
        match &self.info_or_err {
            Ok(info) => Utf8CpAccessor::from(self.constant_pool, info.name_index),
            Err(e) => Utf8CpAccessor::error(self.constant_pool, e)
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> StringCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Class(info)) => StringCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => StringCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_String_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => StringCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> StringCpAccessor<'a> {
        StringCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct FieldrefCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantFieldrefInfo>,
}

impl FieldrefCpAccessor<'_> {
    fn class(&self) -> ClassCpAccessor {
        match &self.info_or_err {
            Ok(info) => ClassCpAccessor::from(self.constant_pool, info.class_index),
            Err(e) => ClassCpAccessor::error(self.constant_pool, e)
        }
    }

    fn name_and_type(&self) -> NameAndTypeCpAccessor {
        match &self.info_or_err {
            Ok(info) => NameAndTypeCpAccessor::from(self.constant_pool, info.name_and_type_index),
            Err(e) => NameAndTypeCpAccessor::error(self.constant_pool, e)
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> FieldrefCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Fieldref(info)) => FieldrefCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => FieldrefCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_Fieldref structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => FieldrefCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> FieldrefCpAccessor<'a> {
        FieldrefCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct MethodrefCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantMethodrefInfo>,
}

impl MethodrefCpAccessor<'_> {
    fn class(&self) -> ClassCpAccessor {
        match &self.info_or_err {
            Ok(info) => ClassCpAccessor::from(self.constant_pool, info.class_index),
            Err(e) => ClassCpAccessor::error(self.constant_pool, e)
        }
    }

    fn name_and_type(&self) -> NameAndTypeCpAccessor {
        match &self.info_or_err {
            Ok(info) => NameAndTypeCpAccessor::from(self.constant_pool, info.name_and_type_index),
            Err(e) => NameAndTypeCpAccessor::error(self.constant_pool, e)
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> MethodrefCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Methodref(info)) => MethodrefCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => MethodrefCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_Methodref_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => MethodrefCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> MethodrefCpAccessor<'a> {
        MethodrefCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct InterfaceMethodrefCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantInterfaceMethodrefInfo>,
}

impl InterfaceMethodrefCpAccessor<'_> {
    fn class(&self) -> ClassCpAccessor {
        match &self.info_or_err {
            Ok(info) => ClassCpAccessor::from(self.constant_pool, info.class_index),
            Err(e) => ClassCpAccessor::error(self.constant_pool, e)
        }
    }

    fn name_and_type(&self) -> NameAndTypeCpAccessor {
        match &self.info_or_err {
            Ok(info) => NameAndTypeCpAccessor::from(self.constant_pool, info.name_and_type_index),
            Err(e) => NameAndTypeCpAccessor::error(self.constant_pool, e)
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> InterfaceMethodrefCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::InterfaceMethodref(info)) => InterfaceMethodrefCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => InterfaceMethodrefCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_InterfaceMethodref_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => InterfaceMethodrefCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> InterfaceMethodrefCpAccessor<'a> {
        InterfaceMethodrefCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}


pub struct NameAndTypeCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantNameAndTypeInfo>,
}

impl NameAndTypeCpAccessor<'_> {

    fn name(&self) -> Utf8CpAccessor {
        match &self.info_or_err {
            Ok(info) => Utf8CpAccessor::from(self.constant_pool, info.name_index),
            Err(e) => Utf8CpAccessor::error(self.constant_pool, e)
        }
    }

    fn descriptor(&self) -> Utf8CpAccessor {
        match &self.info_or_err {
            Ok(info) => Utf8CpAccessor::from(self.constant_pool, info.descriptor_index),
            Err(e) => Utf8CpAccessor::error(self.constant_pool, e)
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> NameAndTypeCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::NameAndType(info)) => NameAndTypeCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => NameAndTypeCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_NameAndType_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => NameAndTypeCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> NameAndTypeCpAccessor<'a> {
        NameAndTypeCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct MethodHandleCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantMethodHandleInfo>,
}

enum MethodHandleReference<'a> {
    Fieldref(FieldrefCpAccessor<'a>),
    Methodref(MethodrefCpAccessor<'a>),
    InterfaceMethodref(InterfaceMethodrefCpAccessor<'a>),
}

impl MethodHandleCpAccessor<'_> {
    fn reference(&self) -> Result<MethodHandleReference> {
        self.info_or_err.as_ref().map_err(|e| e.to_owned()).and_then(|method_handle_info| {
            get_constant_pool_info(self.constant_pool, method_handle_info.reference_index).and_then(|cp_info| {
                match cp_info {
                    CpInfo::Fieldref(info) => Ok(MethodHandleReference::Fieldref(FieldrefCpAccessor { constant_pool: self.constant_pool, info_or_err: Ok(info) })),
                    CpInfo::Methodref(info) => Ok(MethodHandleReference::Methodref(MethodrefCpAccessor { constant_pool: self.constant_pool, info_or_err: Ok(info) })),
                    CpInfo::InterfaceMethodref(info) => Ok(MethodHandleReference::InterfaceMethodref(InterfaceMethodrefCpAccessor { constant_pool: self.constant_pool, info_or_err: Ok(info) })),
                    other_info => error(format!("The index must refer to CONSTANT_Fieldref_info or CONSTANT_Methodref_info or CONSTANT_InterfaceMethodref_info, but {} found! index: {}", cp_info_name(other_info), method_handle_info.reference_index)),
                }
            })
        })
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> MethodHandleCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::MethodHandle(info)) => MethodHandleCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => MethodHandleCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_MethodHandle_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => MethodHandleCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> MethodHandleCpAccessor<'a> {
        MethodHandleCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct MethodTypeCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantMethodTypeInfo>,
}

impl MethodTypeCpAccessor<'_> {
    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> MethodTypeCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::MethodType(info)) => MethodTypeCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => MethodTypeCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_MethodType_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => MethodTypeCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> MethodTypeCpAccessor<'a> {
        MethodTypeCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct DynamicCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantDynamicInfo>,
}

impl DynamicCpAccessor<'_> {

    fn get_bootstrap_method_attr_index(&self) -> Result<u16> {
        self.info_or_err.as_ref()
            .map(|info| { info.bootstrap_method_attr_index})
            .map_err(|e| e.to_owned())
    }

    fn name_and_type(&self) -> NameAndTypeCpAccessor {
        match &self.info_or_err {
            Ok(info) => NameAndTypeCpAccessor::from(&self.constant_pool, info.name_and_type_index),
            Err(e) => NameAndTypeCpAccessor::error(&self.constant_pool, e)
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> DynamicCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Dynamic(info)) => DynamicCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => DynamicCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_MethodType_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => DynamicCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> DynamicCpAccessor<'a> {
        DynamicCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct InvokeDynamicCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantInvokeDynamicInfo>,
}

impl InvokeDynamicCpAccessor<'_> {

    fn get_bootstrap_method_attr_index(&self) -> Result<u16> {
        self.info_or_err.as_ref()
            .map(|info| { info.bootstrap_method_attr_index})
            .map_err(|e| e.to_owned())
    }

    fn name_and_type(&self) -> NameAndTypeCpAccessor {
        match &self.info_or_err {
            Ok(info) => NameAndTypeCpAccessor::from(&self.constant_pool, info.name_and_type_index),
            Err(e) => NameAndTypeCpAccessor::error(&self.constant_pool, e)
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> InvokeDynamicCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::InvokeDynamic(info)) => InvokeDynamicCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => InvokeDynamicCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_MethodType_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => InvokeDynamicCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> InvokeDynamicCpAccessor<'a> {
        InvokeDynamicCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct ModuleCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantModuleInfo>,
}

impl ModuleCpAccessor<'_> {
    fn name(&self) -> Utf8CpAccessor {
        match &self.info_or_err {
            Ok(info) => Utf8CpAccessor::from(self.constant_pool, info.name_index),
            Err(e) => Utf8CpAccessor::error(self.constant_pool, e)
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> ModuleCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Module(info)) => ModuleCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => ModuleCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_String_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => ModuleCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> ModuleCpAccessor<'a> {
        ModuleCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

pub struct PackageCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    info_or_err: Result<&'a ConstantPackageInfo>,
}

impl PackageCpAccessor<'_> {
    fn name(&self) -> Utf8CpAccessor {
        match &self.info_or_err {
            Ok(info) => Utf8CpAccessor::from(self.constant_pool, info.name_index),
            Err(e) => Utf8CpAccessor::error(self.constant_pool, e)
        }
    }

    fn from(constant_pool: &Vec<CpInfo>, index: u16) -> PackageCpAccessor {
        match get_constant_pool_info(constant_pool, index) {
            Ok(CpInfo::Package(info)) => PackageCpAccessor { constant_pool, info_or_err: Ok(&info) },
            Ok(other_info) => PackageCpAccessor {
                constant_pool,
                info_or_err: error(format!("The index must refer to CONSTANT_String_info structure, but {} found! index: {}", cp_info_name(other_info), index)),
            },
            Err(e) => PackageCpAccessor { constant_pool, info_or_err: Err(e) }
        }
    }

    fn error<'a>(constant_pool: &'a Vec<CpInfo>, e: &'a AccessError) -> PackageCpAccessor<'a> {
        PackageCpAccessor { constant_pool, info_or_err: Err(e.to_owned()) }
    }
}

#[test]
fn test() {
    let constant_pool: &Vec<CpInfo> = &vec![
        CpInfo::Methodref(ConstantMethodrefInfo { tag: 0x0a, class_index: 0x02, name_and_type_index: 0x03 }),
        CpInfo::Class(ConstantClassInfo { tag: 0x07, name_index: 0x04 }),
        CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 0x0c, name_index: 0x05, descriptor_index: 0x06 }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x10, bytes: "java/lang/Object".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x06, bytes: "<init>".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x03, bytes: "()V".as_bytes().to_vec() }),
        CpInfo::Methodref(ConstantMethodrefInfo { tag: 0x0a, class_index: 0x08, name_and_type_index: 0x09 }),
        CpInfo::Class(ConstantClassInfo { tag: 0x07, name_index: 0x0a }),
        CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 0x0c, name_index: 0x0b, descriptor_index: 0x0c }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x07, bytes: "Sample1".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x03, bytes: "add".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x05, bytes: "(II)I".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x04, bytes: "Code".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x0f, bytes: "LineNumberTable".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x04, bytes: "prog".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x03, bytes: "()I".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x0a, bytes: "SourceFile".as_bytes().to_vec() }),
        CpInfo::Utf8(ConstantUtf8Info { tag: 0x01, length: 0x0c, bytes: "Sample1.java".as_bytes().to_vec() }),
    ];

    let str = constant_pool.access_as_class(8).name().bytes_as_string();

    assert_eq!(str, Ok("Sample1".to_string()))
}