use crate::class_file::raw_structure::*;
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

    println!("{:#04x?}", class_file);

    assert_eq!(class_file, ClassFile {
        magic: Magic { value: [0xca, 0xfe, 0xba, 0xbe, ] },
        version: Version {
            minor_version: 0x00,
            major_version: 0x3d,
        },
        constant_pool: ConstantPool {
            constant_pool_count: 0x16,
            cp_infos: vec![
                CpInfo::ConstantMethodrefInfo { tag: 0x0a, class_index: 0x02, name_and_type_index: 0x03 },
                CpInfo::ConstantClassInfo { tag: 0x07, name_index: 0x04 },
                CpInfo::ConstantNameAndTypeInfo { tag: 0x0c, name_index: 0x05, descriptor_index: 0x06 },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x10, bytes: "java/lang/Object".to_string() },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x06, bytes: "<init>".to_string() },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x03, bytes: "()V".to_string() },
                CpInfo::ConstantFieldrefInfo { tag: 0x09, class_index: 0x08, name_and_type_index: 0x09 },
                CpInfo::ConstantClassInfo { tag: 0x07, name_index: 0x0a },
                CpInfo::ConstantNameAndTypeInfo { tag: 0x0c, name_index: 0x0b, descriptor_index: 0x0c },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x06, bytes: "Person".to_string() },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x04, bytes: "name".to_string() },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x12, bytes: "Ljava/lang/String;".to_string() },
                CpInfo::ConstantFieldrefInfo { tag: 0x09, class_index: 0x08, name_and_type_index: 0x0e },
                CpInfo::ConstantNameAndTypeInfo { tag: 0x0c, name_index: 0x0f, descriptor_index: 0x10 },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x03, bytes: "age".to_string() },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x01, bytes: "I".to_string() },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x16, bytes: "(Ljava/lang/String;I)V".to_string() },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x04, bytes: "Code".to_string() },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x0f, bytes: "LineNumberTable".to_string() },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x0a, bytes: "SourceFile".to_string() },
                CpInfo::ConstantUtf8Info { tag: 0x01, length: 0x0b, bytes: "Person.java".to_string() },
            ],
        },
        access_flags: 0x20,
        this_class: 0x08,
        super_class: 0x02,
        interfaces_count: 0x00,
        interfaces: vec![],
        fields_count: 0x02,
        fields: vec![
            FieldsInfo {
                access_flags: 0x00,
                name_index: 0x0b,
                descriptor_index: 0x0c,
                attributes_count: 0x00,
                attributes: vec![],
            },
            FieldsInfo {
                access_flags: 0x02,
                name_index: 0x0f,
                descriptor_index: 0x10,
                attributes_count: 0x00,
                attributes: vec![],
            },
        ],
        methods_count: 0x01,
        methods: vec![
            MethodInfo {
                access_flags: 0x00,
                name_index: 0x05,
                descriptor_index: 0x11,
                attributes_count: 0x01,
                attributes: vec![
                    CodeAttributeInfo {
                        attribute_name_index: 0x12,
                        attribute_length: 0x33,
                        max_stack: 0x02,
                        max_locals: 0x03,
                        code_length: 0x0f,
                        code: vec![0x2a, 0xb7, 0x00, 0x01, 0x2a, 0x2b, 0xb5, 0x00, 0x07, 0x2a, 0x1c, 0xb5, 0x00, 0x0d, 0xb1],
                        exception_table_length: 0x00,
                        exception_table: vec![],
                        attributes_count: 0x01,
                        attributes: vec![
                            AttributeInfo {
                                attribute_name_index: 0x13,
                                attribute_length: 0x12,
                                info: vec![0x00, 0x04, 0x00, 0x00, 0x00, 0x06, 0x00, 0x04, 0x00, 0x07, 0x00, 0x09, 0x00, 0x08, 0x00, 0x0e, 0x00, 0x09],
                            },
                        ],
                    },
                ],
            },
        ],
        attributes_count: 0x01,
        attributes: vec![
            AttributeInfo {
                attribute_name_index: 0x14,
                attribute_length: 0x02,
                info: vec![0x00, 0x15],
            },
        ],
    });

    check_class_file(class_file).unwrap();
}
