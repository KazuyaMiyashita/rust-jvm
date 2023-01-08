use crate::class_file::structure::root::*;
use crate::class_file::structure::constant_pool::*;
use crate::class_file::structure::attribute::*;
use crate::class_file::reader::read_class_file;
use crate::class_file::checker::check_class_file;

#[test]
fn test() {
    // % cat module-info.java
    // module foo {
    //     requires java.net.http;
    // }
    // % javac --version
    // javac 17.0.5
    // % javac module-info.java
    // % od -An -t x1 module-info.class | sed -e 's/^[ \s]*//' -e 's/[ \s]*$//' -e 's/\([0-9|a-z][0-9|a-z]\)/0x\1,/g'
    let bytes: Vec<u8> = vec![
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x0d, 0x07, 0x00, 0x02, 0x01, 0x00, 0x0b,
        0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2d, 0x69, 0x6e, 0x66, 0x6f, 0x01, 0x00, 0x0a, 0x53, 0x6f,
        0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x10, 0x6d, 0x6f, 0x64, 0x75, 0x6c,
        0x65, 0x2d, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x6a, 0x61, 0x76, 0x61, 0x01, 0x00, 0x06, 0x4d, 0x6f,
        0x64, 0x75, 0x6c, 0x65, 0x13, 0x00, 0x07, 0x01, 0x00, 0x03, 0x66, 0x6f, 0x6f, 0x13, 0x00, 0x09,
        0x01, 0x00, 0x09, 0x6a, 0x61, 0x76, 0x61, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x01, 0x00, 0x06, 0x31,
        0x37, 0x2e, 0x30, 0x2e, 0x35, 0x13, 0x00, 0x0c, 0x01, 0x00, 0x0d, 0x6a, 0x61, 0x76, 0x61, 0x2e,
        0x6e, 0x65, 0x74, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x80, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x03, 0x00, 0x00, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x1c, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x08, 0x80, 0x00,
        0x00, 0x0a, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    let class_file = read_class_file(bytes).unwrap();


    println!("{}", class_file);

    assert_eq!(class_file, ClassFile {
        magic: [0xca, 0xfe, 0xba, 0xbe],
        minor_version: 0,
        major_version: 61,
        constant_pool_count: 13,
        constant_pool: vec![
            CpInfo::Class(ConstantClassInfo { tag: 7, name_index: 2 }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 11, bytes: "module-info".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 10, bytes: "SourceFile".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 16, bytes: "module-info.java".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 6, bytes: "Module".as_bytes().to_vec() }),
            CpInfo::Module(ConstantModuleInfo { tag: 19, name_index: 7 }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 3, bytes: "foo".as_bytes().to_vec() }),
            CpInfo::Module(ConstantModuleInfo { tag: 19, name_index: 9 }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 9, bytes: "java.base".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 6, bytes: "17.0.5".as_bytes().to_vec() }),
            CpInfo::Module(ConstantModuleInfo { tag: 19, name_index: 12 }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 13, bytes: "java.net.http".as_bytes().to_vec() }),
        ],
        access_flags: 0x8000,
        this_class: 1,
        super_class: 0,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 0,
        methods: vec![],
        attributes_count: 2,
        attributes: vec![
            Attribute::General(AttributeInfo {
                attribute_name_index: 3,
                attribute_length: 2,
                info: vec![0x00, 0x04],
            }),
            Attribute::Module(ModuleAttribute {
                attribute_name_index: 5,
                attribute_length: 28,
                module_name_index: 6,
                module_flags: 0,
                module_version_index: 0,
                requires_count: 2,
                requires: vec![
                    Require {
                        requires_index: 8,
                        requires_flags: 32768,
                        requires_version_index: 10,
                    },
                    Require {
                        requires_index: 11,
                        requires_flags: 0,
                        requires_version_index: 10,
                    },
                ],
                exports_count: 0,
                exports: vec![],
                opens_count: 0,
                opens: vec![],
                uses_count: 0,
                uses_index: vec![],
                provides_count: 0,
                provides: vec![],
            }),
        ],
    });

    check_class_file(&class_file).unwrap();
}
