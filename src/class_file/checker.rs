use super::structure::*;
use thiserror::Error;

pub fn check_class_file(class_file: ClassFile) -> Result<()> {
    Checker::check(class_file)
}

pub type Result<T> = std::result::Result<T, CheckError>;

#[derive(Error, Debug)]
#[error("Invalid class file. {message:}")]
pub struct CheckError {
    message: String,
}

trait IChecker<T> {
    fn check(content: T) -> Result<()>;
}

struct Checker {}

// 4.8. Format Checking
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.8
impl IChecker<ClassFile> for Checker {

    fn check(content: ClassFile) -> Result<()> {
        // The first four bytes must contain the right magic number.
        if content.magic != [0xca, 0xfe, 0xba, 0xbe] {
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
        let all_attribute_name_index_iter = content.attributes.iter().map(|x| x.attribute_name_index)
            .chain(content.fields.iter().flat_map(|a| a.attributes.iter().map(|y| y.attribute_name_index)))
            .chain(content.methods.iter().flat_map(|a| a.attributes.iter().map(|y| y.attribute_name_index)));
        // TODO: add Code_attribute https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.3
        // TODO: add record_component_info https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.30
        for attribute_name_index in all_attribute_name_index_iter {
            let cp_info = content.constant_pool.get_constant_pool_info(attribute_name_index as usize);
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
        let constant_pool_constant_class_info_name_indexes = content.constant_pool.cp_infos.iter()
            .flat_map(|x| match x {
                CpInfo::ConstantClassInfo { name_index, .. } => Some(*name_index),
                _ => None
            });
        for constant_pool_constant_class_info_name_index in constant_pool_constant_class_info_name_indexes {
            match content.constant_pool.get_constant_pool_info(constant_pool_constant_class_info_name_index as usize) {
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