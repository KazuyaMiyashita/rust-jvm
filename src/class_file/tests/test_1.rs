use crate::class_file::structure::root::*;
use crate::class_file::structure::constant_pool::*;
use crate::class_file::structure::attribute::*;
use crate::class_file::reader::read_class_file;
use crate::class_file::checker::check_class_file;

#[test]
fn test() {
    // % cat Sample1.java
    // class Sample1 {
    //
    //     public static int prog() {
    //         var a = 1;
    //         var b = 42;
    //         var c = add(a, b);
    //         return c;
    //     }
    //
    //     public static int add(int a, int b) {
    //         return a + b;
    //     }
    //
    // }
    // % javac --version
    // javac 17.0.5
    // % javac Sample1.java
    // % od -An -t x1 Sample1.class | sed -e 's/^[ \s]*//' -e 's/[ \s]*$//' -e 's/\([0-9|a-z][0-9|a-z]\)/0x\1,/g'
    let bytes: Vec<u8> = vec![
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x13, 0x0a, 0x00, 0x02, 0x00, 0x03, 0x07,
        0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e,
        0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x0a, 0x00, 0x08, 0x00, 0x09, 0x07, 0x00,
        0x0a, 0x0c, 0x00, 0x0b, 0x00, 0x0c, 0x01, 0x00, 0x07, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x31,
        0x01, 0x00, 0x03, 0x61, 0x64, 0x64, 0x01, 0x00, 0x05, 0x28, 0x49, 0x49, 0x29, 0x49, 0x01, 0x00,
        0x04, 0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62,
        0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x01, 0x00, 0x04, 0x70, 0x72, 0x6f, 0x67, 0x01, 0x00,
        0x03, 0x28, 0x29, 0x49, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c,
        0x65, 0x01, 0x00, 0x0c, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x31, 0x2e, 0x6a, 0x61, 0x76, 0x61,
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
        0x00, 0x01, 0x00, 0x11, 0x00, 0x00, 0x00, 0x02, 0x00, 0x12, ];

    let class_file = read_class_file(bytes).unwrap();

    println!("{}", class_file);

    assert_eq!(class_file, ClassFile {
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
        methods: vec![
            MethodInfo {
                access_flags: 0x0000,
                name_index: 5,
                descriptor_index: 6,
                attributes_count: 1,
                attributes: vec![
                    Attribute::Code(CodeAttributeInfo {
                        attribute_name_index: 0x000d,
                        attribute_length: 29,
                        max_stack: 1,
                        max_locals: 1,
                        code_length: 5,
                        code: vec![0x2a, 0xb7, 0x00, 0x01, 0xb1],
                        exception_table_length: 0,
                        exception_table: vec![],
                        attributes_count: 1,
                        attributes: vec![
                            Attribute::General(AttributeInfo {
                                attribute_name_index: 0x000e,
                                attribute_length: 6,
                                info: vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x01],
                            }),
                        ],
                    }),
                ],
            },
            MethodInfo {
                access_flags: 0x0009,
                name_index: 15,
                descriptor_index: 16,
                attributes_count: 1,
                attributes: vec![
                    Attribute::Code(CodeAttributeInfo {
                        attribute_name_index: 0x000d,
                        attribute_length: 49,
                        max_stack: 2,
                        max_locals: 3,
                        code_length: 13,
                        code: vec![0x04, 0x3b, 0x10, 0x2a, 0x3c, 0x1a, 0x1b, 0xb8, 0x00, 0x07, 0x3d, 0x1c, 0xac],
                        exception_table_length: 0,
                        exception_table: vec![],
                        attributes_count: 1,
                        attributes: vec![
                            Attribute::General(AttributeInfo {
                                attribute_name_index: 0x000e,
                                attribute_length: 18,
                                info: vec![0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x02, 0x00, 0x05, 0x00, 0x05, 0x00, 0x06, 0x00, 0x0b, 0x00, 0x07],
                            }),
                        ],
                    }),
                ],
            },
            MethodInfo {
                access_flags: 0x0009,
                name_index: 11,
                descriptor_index: 12,
                attributes_count: 1,
                attributes: vec![
                    Attribute::Code(CodeAttributeInfo {
                        attribute_name_index: 0x000d,
                        attribute_length: 28,
                        max_stack: 2,
                        max_locals: 2,
                        code_length: 4,
                        code: vec![0x1a, 0x1b, 0x60, 0xac],
                        exception_table_length: 0,
                        exception_table: vec![],
                        attributes_count: 1,
                        attributes: vec![
                            Attribute::General(AttributeInfo {
                                attribute_name_index: 0x000e,
                                attribute_length: 6,
                                info: vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x0b],
                            }),
                        ],
                    }),
                ],
            },
        ],
        attributes_count: 1,
        attributes: vec![
            Attribute::General(AttributeInfo {
                attribute_name_index: 0x0011,
                attribute_length: 2,
                info: vec![0x00, 0x12],
            }),
        ],
    });

    check_class_file(&class_file).unwrap();
}
