use super::*;


// original constant_pool table is indexed from 1 to constant_pool_count - 1.
// Note that the Vec of this cp_infos structure is indexed from 0.
fn get_constant_pool_info(constant_pool: &Vec<CpInfo>, index: usize) -> Option<&CpInfo> {
    constant_pool.get(index - 1)
}

trait CpAccessor {
    fn access_as_class(&self, cp: &CpInfo) -> Option<ClassCpAccessor>;
}

impl CpAccessor for &Vec<CpInfo> {
    fn access_as_class(&self, cp: &CpInfo) -> ClassCpAccessor {
        let class_info: Option<&ConstantClassInfo> = match get_constant_pool_info(constant_pool, cp.name_index as usize) {
            Some(CpInfo::Class(info)) => Some(info),
            _ => None
        };
        ClassCpAccessor { constant_pool: *self, class_info }
    }
}

struct ClassCpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    class_info: Option<&'a ConstantClassInfo>,
}

impl ClassCpAccessor { // WIPWIPWIP expected lifetime parameter
    fn name(&self) -> Utf8CpAccessor {
        match self.class_info  {
            Some(class_info) => {
                match get_constant_pool_info(&self.constant_pool, class_info.name_index as usize) {
                    Some(CpInfo::Utf8(info)) => Utf8CpAccessor { constant_pool, utf8_info: Some(info) },
                    _ => Utf8CpAccessor { constant_pool, utf8_info: None }
                }
            }
            None => Utf8CpAccessor { constant_pool, utf8_info: None }
        }
    }
}

struct Utf8CpAccessor<'a> {
    constant_pool: &'a Vec<CpInfo>,
    utf8_info: Option<&'a ConstantUtf8Info>,
}

impl Utf8CpAccessor {
    fn bytes_as_string(&self) -> Option<String> {
        match self.utf8_info {
            Some(utf8_info) =>  String::from_utf8(utf8_info.bytes.clone()).ok(),
            None => None
        }
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

    let str = constant_pool.access_as_class(&constant_pool[8]).name().bytes_as_string();

    assert_eq!(str, Ok("Sample1".to_string()))
}