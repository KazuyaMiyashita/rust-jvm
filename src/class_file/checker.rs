use super::structure::*;
use thiserror::Error;

pub fn check_class_file(class_file: ClassFile) -> Result<()> {
    class_file.check()
}

pub type Result<T> = std::result::Result<T, CheckError>;

#[derive(Error, Debug)]
#[error("Invalid class file. {message:}")]
pub struct CheckError {
    pub message: String,
}

pub(crate) trait Checker {
    fn check(&self) -> Result<()>;
}

// Checking format of some fields requires a ConstantPool reference.
pub(crate) trait CheckerWithConstantPool {
    fn check_with_constant_pool(&self, constant_pool: &ConstantPool) -> Result<()>;
}

// utils
fn error<T>(message: String) -> Result<T> {
    Err(CheckError { message })
}

// 4.1. The ClassFile Structure
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
//
// > The magic item supplies the magic number identifying the class file format; it has the value 0xCAFEBABE.
impl Checker for Magic {
    fn check(&self) -> Result<()> {
        match self.value {
            [0xca, 0xfe, 0xba, 0xbe] => Ok(()),
            _ => error("This is not a class file. The first byte array must be `cafebabe`".to_string())
        }
    }
}

// 4.1. The ClassFile Structure
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
//
// According to the above, Java SE 17 must support major_version 45 upto 61.
// Also,
// > For a class file whose major_version is 56 or above, the minor_version must be 0 or 65535.
// > For a class file whose major_version is between 45 and 55 inclusive, the minor_version may be any value.
impl Checker for Version {
    fn check(&self) -> Result<()> {
        match (self.major_version, self.minor_version) {
            (56..=61, 0 | 65535) => Ok(()),
            (56..=61, _) => error(format!("invalid class file minor version.\
                The version of this input is major: {}, minor: {}.", self.major_version, self.minor_version)),
            (45..=61, _) => Ok(()),
            _ => error(format!(
                "Not supported class file version. \
                The version of this input is major: {}, minor: {}.\
                This JVM is version 17. Class file major versions 45 upto 61 are supported.", self.major_version, self.minor_version))
        }
    }
}

// 4.7. Attributes
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7
//
// > For all attributes, the attribute_name_index item must be a valid unsigned 16-bit index into the constant pool of the class.
// > The constant_pool entry at attribute_name_index must be a CONSTANT_Utf8_info structure (§4.4.7) representing the name of the attribute.
impl CheckerWithConstantPool for CpUtf8Ref {
    fn check_with_constant_pool(&self, constant_pool: &ConstantPool) -> Result<()> {
        let cp_info: &CpInfo = constant_pool.get_constant_pool_info(self.value as usize);
        match cp_info {
            CpInfo::ConstantUtf8Info { .. } => Ok(()),
            _ => error("The constant_pool entry at attribute_name_index must be a CONSTANT_Utf8_info structure.".to_string())
        }
    }
}

impl<T: CheckerWithConstantPool> CheckerWithConstantPool for Vec<T> {
    fn check_with_constant_pool(&self, constant_pool: &ConstantPool) -> Result<()> {
        for t in self {
            t.check_with_constant_pool(constant_pool)?;
        }
        Ok(())
    }
}

// impl CheckerWithConstantPool for CpInfo::ConstantClassInfo {
//     fn check_with_constant_pool(&self, constant_pool: &ConstantPool) -> Result<()> {
//         self.name_index.get_constant_pool_info(constant_pool)?;
//         Ok(())
//     }
// }

impl CheckerWithConstantPool for AttributeInfo {
    fn check_with_constant_pool(&self, constant_pool: &ConstantPool) -> Result<()> {
        self.attribute_name_index.check_with_constant_pool(constant_pool)?;
        Ok(())
    }
}

impl CheckerWithConstantPool for FieldsInfo {
    fn check_with_constant_pool(&self, constant_pool: &ConstantPool) -> Result<()> {
        self.name_index.check_with_constant_pool(constant_pool)?;
        Ok(())
    }
}

impl CheckerWithConstantPool for CodeAttributeInfo {
    fn check_with_constant_pool(&self, constant_pool: &ConstantPool) -> Result<()> {
        self.attribute_name_index.check_with_constant_pool(constant_pool)?;
        Ok(())
    }
}

impl CheckerWithConstantPool for MethodInfo {
    fn check_with_constant_pool(&self, constant_pool: &ConstantPool) -> Result<()> {
        self.name_index.check_with_constant_pool(constant_pool)?;
        self.attributes.check_with_constant_pool(constant_pool)?;
        Ok(())
    }
}

// 4.8. Format Checking
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.8
impl Checker for ClassFile {
    fn check(&self) -> Result<()> {
        self.magic.check()?;
        self.version.check()?;

        self.attributes.check_with_constant_pool(&self.constant_pool)?;
        self.fields.check_with_constant_pool(&self.constant_pool)?;
        self.methods.check_with_constant_pool(&self.constant_pool)?;

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
        let constant_pool_constant_class_info_name_indexes = self.constant_pool.cp_infos.iter()
            .flat_map(|x| match x {
                CpInfo::ConstantClassInfo { name_index, .. } => Some(*name_index),
                _ => None
            });
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
}