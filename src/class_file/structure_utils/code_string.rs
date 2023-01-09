use crate::class_file::structure::constant_pool::*;
use crate::class_file::structure::root::*;

enum Elem {
    Struct(
        &'static str,
        Vec<Elem>,
        &'static str,
    ),
    Line(String),
}

fn _string_lines(elem: &Elem, indent: usize) -> Vec<String> {
    match elem {
        Elem::Struct(header, content, footer) => {
            if content.is_empty() {
                vec!["    ".repeat(indent) + header + footer]
            } else {
                let mut content_lines: Vec<String> = content.iter().flat_map(|x| _string_lines(x, indent + 1)).collect();
                content_lines.insert(0, "    ".repeat(indent) + header);
                content_lines.push("    ".repeat(indent) + footer);
                content_lines
            }
        }
        Elem::Line(line) => vec!["    ".repeat(indent) + line]
    }
}

trait CodeString {
    fn to_elem(&self) -> Elem;
    fn code_string(&self) -> String {
        _string_lines(&self.to_elem(), 0).join("\n")
    }
}

impl CodeString for ClassFile {
    fn to_elem(&self) -> Elem {
        Elem::Struct(
            "ClassFile {",
            vec![
                Elem::Line(format!("magic: [{}],", self.magic.iter().map(|x| format!("{:#04x?}", x)).collect::<Vec<String>>().join(", "))),
                Elem::Line(format!("minor_version: {},", self.minor_version)),
                Elem::Line(format!("constant_pool_count: {},", self.constant_pool_count)),
                Elem::Struct(
                    "constant_pool: vec![",
                    self.constant_pool.iter().map(|x| x.to_elem()).collect(),
                    "],",
                ),
                Elem::Line(format!("access_flags: {},", self.access_flags)),
            ],
            "}",
        )
    }
}

impl CodeString for CpInfo {
    fn to_elem(&self) -> Elem {
        let str = match self {
            CpInfo::Utf8(info) => {
                format!("CpInfo::Utf8(ConstantUtf8Info {{ tag: {}, length: {}, bytes: \"{}\".as_bytes().to_vec() }})",
                        info.tag, info.length, String::from_utf8(info.bytes.clone()).unwrap()
                )
            }
            CpInfo::Integer(info) => {
                format!("CpInfo::Integer(ConstantIntegerInfo {{ tag: {}, bytes: {}_i32.to_be_bytes() }})",
                        info.tag, i32::from_be_bytes(info.bytes)
                )
            }
            CpInfo::Float(info) => {
                format!("CpInfo::Float(ConstantFloatInfo {{ tag: {}, bytes: {}_f32.to_be_bytes() }})",
                        info.tag, f32::from_be_bytes(info.bytes)
                )
            }
            CpInfo::Long(info) => {
                let value = i64::from_be_bytes([info.high_bytes, info.low_bytes].concat().try_into().unwrap());
                format!("CpInfo::Long(ConstantLongInfo {{ tag: {}, high_bytes: {}_i64.to_be_bytes()[0..=3], low_bytes: {}_i64.to_be_bytes()[4..=7] }})",
                        info.tag, value, value
                )
            }
            CpInfo::Double(info) => {
                let value = f64::from_be_bytes([info.high_bytes, info.low_bytes].concat().try_into().unwrap());
                format!("CpInfo::Double(ConstantDoubleInfo {{ tag: {}, high_bytes: {}_f64.to_be_bytes()[0..=3], low_bytes: {}_f64.to_be_bytes()[4..=7] }})",
                        info.tag, value, value
                )
            }
            CpInfo::Class(info) => {
                format!("CpInfo::Class(ConstantClassInfo {{ tag: {}, name_index: {} }})",
                        info.tag, info.name_index
                )
            }
            CpInfo::String(info) => {
                format!("CpInfo::String(ConstantStringInfo {{ tag: {}, string_index: {} }})",
                        info.tag, info.string_index
                )
            }
            CpInfo::Fieldref(info) => {
                format!("CpInfo::Fieldref(ConstantFieldrefInfo {{ tag: {}, class_index: {}, name_and_type_index: {} }})",
                        info.tag, info.class_index, info.name_and_type_index
                )
            }
            CpInfo::Methodref(info) => {
                format!("CpInfo::Methodref(ConstantMethodrefInfo {{ tag: {}, class_index: {}, name_and_type_index: {} }})",
                        info.tag, info.class_index, info.name_and_type_index
                )
            }
            CpInfo::InterfaceMethodref(info) => {
                format!("CpInfo::InterfaceMethodref(ConstantInterfaceMethodrefInfo {{ tag: {}, class_index: {}, name_and_type_index: {} }})",
                        info.tag, info.class_index, info.name_and_type_index
                )
            }
            CpInfo::NameAndType(info) => {
                format!("CpInfo::NameAndType(ConstantNameAndTypeInfo {{ tag: {}, name_index: {}, descriptor_index: {} }})",
                        info.tag, info.name_index, info.descriptor_index
                )
            }
            CpInfo::MethodHandle(info) => {
                format!("CpInfo::MethodHandle(ConstantMethodHandleInfo {{ tag: {}, reference_kind: {}, reference_index: {} }})",
                        info.tag, info.reference_kind, info.reference_index
                )
            }
            CpInfo::MethodType(info) => {
                format!("CpInfo::MethodType(ConstantMethodTypeInfo {{ tag: {}, descriptor_index: {}, }})",
                        info.tag, info.descriptor_index
                )
            }
            CpInfo::Dynamic(info) => {
                format!("CpInfo::Dynamic(ConstantDynamicInfo {{ tag: {}, bootstrap_method_attr_index: {}, name_and_type_index: {} }})",
                        info.tag, info.bootstrap_method_attr_index, info.name_and_type_index
                )
            }
            CpInfo::InvokeDynamic(info) => {
                format!("CpInfo::InvokeDynamic(ConstantInvokeDynamicInfo {{ tag: {}, bootstrap_method_attr_index: {}, name_and_type_index: {} }})",
                        info.tag, info.bootstrap_method_attr_index, info.name_and_type_index
                )
            }
            CpInfo::Module(info) => {
                format!("CpInfo::Module(ConstantModuleInfo {{ tag: {}, name_index: {} }})",
                        info.tag, info.name_index
                )
            }
            CpInfo::Package(info) => {
                format!("CpInfo::Package(ConstantPackageInfo {{ tag: {}, name_index: {} }})",
                        info.tag, info.name_index
                )
            }
        };
        Elem::Line(str)
    }
}

#[test]
fn test() {
    let class_file = ClassFile {
        magic: [0xca, 0xfe, 0xba, 0xbe],
        minor_version: 0,
        major_version: 61,
        constant_pool_count: 19,
        constant_pool: vec![
            CpInfo::Methodref(ConstantMethodrefInfo { tag: 10, class_index: 2, name_and_type_index: 3 }),
            CpInfo::Class(ConstantClassInfo { tag: 7, name_index: 4 }),
            CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 12, name_index: 5, descriptor_index: 6 }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 16, bytes: "java/lang/Object".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 6, bytes: "<init>".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 3, bytes: "()V".as_bytes().to_vec() }),
            CpInfo::Methodref(ConstantMethodrefInfo { tag: 10, class_index: 8, name_and_type_index: 9 }),
            CpInfo::Class(ConstantClassInfo { tag: 7, name_index: 10 }),
            CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 12, name_index: 11, descriptor_index: 12 }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 7, bytes: "Sample1".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 3, bytes: "add".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 5, bytes: "(II)I".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 4, bytes: "Code".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 15, bytes: "LineNumberTable".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 4, bytes: "prog".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 3, bytes: "()I".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 10, bytes: "SourceFile".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 12, bytes: "Sample1.java".as_bytes().to_vec() }),
        ],
        access_flags: 0x0020,
        this_class: 8,
        super_class: 2,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 3,
        methods: vec![],
        attributes_count: 1,
        attributes: vec![],
    };

    let code_string = class_file.code_string();

    assert_eq!(code_string, r#"ClassFile {
    magic: [0xca, 0xfe, 0xba, 0xbe],
    minor_version: 0,
    constant_pool_count: 19,
    constant_pool: vec![
        CpInfo::Methodref(ConstantMethodrefInfo { tag: 10, class_index: 2, name_and_type_index: 3 })
        CpInfo::Class(ConstantClassInfo { tag: 7, name_index: 4 })
        CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 12, name_index: 5, descriptor_index: 6 })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 16, bytes: "java/lang/Object".as_bytes().to_vec() })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 6, bytes: "<init>".as_bytes().to_vec() })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 3, bytes: "()V".as_bytes().to_vec() })
        CpInfo::Methodref(ConstantMethodrefInfo { tag: 10, class_index: 8, name_and_type_index: 9 })
        CpInfo::Class(ConstantClassInfo { tag: 7, name_index: 10 })
        CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 12, name_index: 11, descriptor_index: 12 })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 7, bytes: "Sample1".as_bytes().to_vec() })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 3, bytes: "add".as_bytes().to_vec() })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 5, bytes: "(II)I".as_bytes().to_vec() })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 4, bytes: "Code".as_bytes().to_vec() })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 15, bytes: "LineNumberTable".as_bytes().to_vec() })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 4, bytes: "prog".as_bytes().to_vec() })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 3, bytes: "()I".as_bytes().to_vec() })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 10, bytes: "SourceFile".as_bytes().to_vec() })
        CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 12, bytes: "Sample1.java".as_bytes().to_vec() })
    ],
    access_flags: 32,
}"#)
}