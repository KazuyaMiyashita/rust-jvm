use crate::class_file::structure::*;
use crate::class_file::reader::read_class_file;
use crate::class_file::checker::check_class_file;

#[test]
fn test() {
    // % cat Person.java
    // class Person {
    //
    //     String name;
    //     private int age;
    //
    //     Person(String name, int age) {
    //         this.name = name;
    //         this.age = age;
    //     }
    //
    // }
    // % javac --version
    // javac 17.0.5
    // % javac Person.java
    // % od -An -t x1 Person.class | sed -e 's/^[ \s]*//' -e 's/[ \s]*$//' -e 's/\([0-9|a-z][0-9|a-z]\)/0x\1,/g'
    let bytes: &[u8] = &[
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x16, 0x0a, 0x00, 0x02, 0x00, 0x03, 0x07,
        0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e,
        0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x09, 0x00, 0x08, 0x00, 0x09, 0x07, 0x00,
        0x0a, 0x0c, 0x00, 0x0b, 0x00, 0x0c, 0x01, 0x00, 0x06, 0x50, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x01,
        0x00, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x01, 0x00, 0x12, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x09, 0x00, 0x08, 0x00, 0x0e,
        0x0c, 0x00, 0x0f, 0x00, 0x10, 0x01, 0x00, 0x03, 0x61, 0x67, 0x65, 0x01, 0x00, 0x01, 0x49, 0x01,
        0x00, 0x16, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74,
        0x72, 0x69, 0x6e, 0x67, 0x3b, 0x49, 0x29, 0x56, 0x01, 0x00, 0x04, 0x43, 0x6f, 0x64, 0x65, 0x01,
        0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c,
        0x65, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00,
        0x0b, 0x50, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x2e, 0x6a, 0x61, 0x76, 0x61, 0x00, 0x20, 0x00, 0x08,
        0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x0f, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x00, 0x11, 0x00, 0x01,
        0x00, 0x12, 0x00, 0x00, 0x00, 0x33, 0x00, 0x02, 0x00, 0x03, 0x00, 0x00, 0x00, 0x0f, 0x2a, 0xb7,
        0x00, 0x01, 0x2a, 0x2b, 0xb5, 0x00, 0x07, 0x2a, 0x1c, 0xb5, 0x00, 0x0d, 0xb1, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x13, 0x00, 0x00, 0x00, 0x12, 0x00, 0x04, 0x00, 0x00, 0x00, 0x06, 0x00, 0x04, 0x00,
        0x07, 0x00, 0x09, 0x00, 0x08, 0x00, 0x0e, 0x00, 0x09, 0x00, 0x01, 0x00, 0x14, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x15, ];

    let class_file = read_class_file(bytes).unwrap();

    // println!("{}", class_file);

    assert_eq!(class_file, ClassFile {
        magic: [0xca, 0xfe, 0xba, 0xbe],
        minor_version: 0,
        major_version: 61,
        constant_pool_count: 22,
        constant_pool: vec![
            CpInfo::Methodref(ConstantMethodrefInfo { tag: 10, class_index: 2, name_and_type_index: 3 }),
            CpInfo::Class(ConstantClassInfo { tag: 7, name_index: 4 }),
            CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 12, name_index: 5, descriptor_index: 6 }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 16, bytes: "java/lang/Object".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 6, bytes: "<init>".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 3, bytes: "()V".as_bytes().to_vec() }),
            CpInfo::Fieldref(ConstantFieldrefInfo { tag: 9, class_index: 8, name_and_type_index: 9 }),
            CpInfo::Class(ConstantClassInfo { tag: 7, name_index: 10 }),
            CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 12, name_index: 11, descriptor_index: 12 }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 6, bytes: "Person".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 4, bytes: "name".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 18, bytes: "Ljava/lang/String;".as_bytes().to_vec() }),
            CpInfo::Fieldref(ConstantFieldrefInfo { tag: 9, class_index: 8, name_and_type_index: 14 }),
            CpInfo::NameAndType(ConstantNameAndTypeInfo { tag: 12, name_index: 15, descriptor_index: 16 }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 3, bytes: "age".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 1, bytes: "I".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 22, bytes: "(Ljava/lang/String;I)V".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 4, bytes: "Code".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 15, bytes: "LineNumberTable".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 10, bytes: "SourceFile".as_bytes().to_vec() }),
            CpInfo::Utf8(ConstantUtf8Info { tag: 1, length: 11, bytes: "Person.java".as_bytes().to_vec() }),
        ],
        access_flags: 0x0020,
        this_class: 8,
        super_class: 2,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 2,
        fields: vec![
            FieldsInfo {
                access_flags: 0x0000,
                name_index: 11,
                descriptor_index: 12,
                attributes_count: 0,
                attributes: vec![
                ]
            },
            FieldsInfo {
                access_flags: 0x0002,
                name_index: 15,
                descriptor_index: 16,
                attributes_count: 0,
                attributes: vec![
                ]
            },
        ],
        methods_count: 1,
        methods: vec![
            MethodInfo {
                access_flags: 0x0000,
                name_index: 5,
                descriptor_index: 17,
                attributes_count: 1,
                attributes: vec![
                    CodeAttributeInfo {
                        attribute_name_index: 0x0012,
                        attribute_length: 51,
                        max_stack: 2,
                        max_locals: 3,
                        code_length: 15,
                        code: vec![0x2a, 0xb7, 0x00, 0x01, 0x2a, 0x2b, 0xb5, 0x00, 0x07, 0x2a, 0x1c, 0xb5, 0x00, 0x0d, 0xb1],
                        exception_table_length: 0,
                        exception_table: vec![],
                        attributes_count: 1,
                        attributes: vec![
                            AttributeInfo {
                                attribute_name_index: 0x0013,
                                attribute_length: 18,
                                info: vec![0x00, 0x04, 0x00, 0x00, 0x00, 0x06, 0x00, 0x04, 0x00, 0x07, 0x00, 0x09, 0x00, 0x08, 0x00, 0x0e, 0x00, 0x09],
                            },
                        ]
                    },
                ]
            },
        ],
        attributes_count: 1,
        attributes: vec![
            AttributeInfo {
                attribute_name_index: 0x0014,
                attribute_length: 2,
                info: vec![0x00, 0x15],
            },
        ],
    });

    check_class_file(&class_file).unwrap();
}
