#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub struct ClassFile {
    magic: [u8; 4],
    minor_version: [u8; 2],
    major_version: [u8; 2],
    constant_pool_count: u16,
    constant_pool: Vec<CpInfo>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    fields: Vec<FieldsInfo>,
    methods_count: u16,
    methods: Vec<MethodInfo>,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    fn read_from_bytes(bytes: &[u8]) -> ClassFile {
        let mut offset: usize = 0;

        let magic: [u8; 4] = read_n(&bytes, &mut offset);

        let minor_version: [u8; 2] = read_n(&bytes, &mut offset);

        let major_version: [u8; 2] = read_n(&bytes, &mut offset);

        let constant_pool_count: u16 = read_u16(&bytes, &mut offset);

        let mut constant_pool: Vec<CpInfo> = Vec::new();
        // The constant_pool table is indexed from 1 to constant_pool_count - 1. .. https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
        for _ in 1..constant_pool_count {
            let cp = CpInfo::read(&bytes, &mut offset);
            constant_pool.push(cp);
        };

        let access_flags: u16 = read_u16(&bytes, &mut offset);

        let this_class: u16 = read_u16(&bytes, &mut offset);

        let super_class: u16 = read_u16(&bytes, &mut offset);

        let interfaces_count: u16 = read_u16(&bytes, &mut offset);

        let mut interfaces: Vec<u16> = Vec::new();
        for _ in 0..interfaces_count {
            let interface = read_u16(&bytes, &mut offset);
            interfaces.push(interface);
        }

        let fields_count: u16 = read_u16(&bytes, &mut offset);
        let mut fields: Vec<FieldsInfo> = Vec::new();
        for _ in 0..fields_count {
            fields.push(FieldsInfo::read(&bytes, &mut offset));
        }

        let methods_count: u16 = read_u16(&bytes, &mut offset);
        let mut methods: Vec<MethodInfo> = Vec::new();
        for _ in 0..methods_count {
            methods.push(MethodInfo::read(&bytes, &mut offset));
        }

        let attributes_count: u16 = read_u16(&bytes, &mut offset);
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::read(&bytes, &mut offset));
        }

        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        }
    }

    fn check_format(&self) -> () {
        if self.magic != [0xca, 0xfe, 0xba, 0xbe] {
            panic!("The first four bytes must contain the right magic number.")
        }
    }

    pub fn load(bytes: &[u8]) -> ClassFile {
        let cf = ClassFile::read_from_bytes(bytes);
        cf.check_format();
        cf
    }
}

type CpInfoTag = u8;

const CONSTANT_UTF8: CpInfoTag = 1;
const CONSTANT_INTEGER: CpInfoTag = 3;
const CONSTANT_FLOAT: CpInfoTag = 4;
const CONSTANT_LONG: CpInfoTag = 5;
const CONSTANT_DOUBLE: CpInfoTag = 6;
const CONSTANT_CLASS: CpInfoTag = 7;
const CONSTANT_STRING: CpInfoTag = 8;
const CONSTANT_FIELDREF: CpInfoTag = 9;
const CONSTANT_METHODREF: CpInfoTag = 10;
const CONSTANT_INTERFACE_METHODREF: CpInfoTag = 11;
const CONSTANT_NAME_AND_TYPE: CpInfoTag = 12;
const CONSTANT_METHOD_HANDLE: CpInfoTag = 15;
const CONSTANT_METHOD_TYPE: CpInfoTag = 16;
const CONSTANT_DYNAMIC: CpInfoTag = 17;
const CONSTANT_INVOKE_DYNAMIC: CpInfoTag = 18;
const CONSTANT_MODULE: CpInfoTag = 19;
const CONSTANT_PACKAGE: CpInfoTag = 20;

#[derive(Debug, PartialEq)]
pub enum CpInfo {
    ConstantClassInfo {
        tag: CpInfoTag,
        name_index: u16,
    },
    ConstantUtf8Info {
        tag: CpInfoTag,
        length: u16,
        bytes: Vec<u8>,
    },
    ConstantMethodrefInfo {
        tag: CpInfoTag,
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantNameAndTypeInfo {
        tag: CpInfoTag,
        name_index: u16,
        descriptor_index: u16,
    },
}

impl CpInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> CpInfo {
        let tag: CpInfoTag = read_u8(&bytes, &mut *offset);
        match tag {
            CONSTANT_UTF8 => {
                let length = read_u16(&bytes, &mut *offset);
                CpInfo::ConstantUtf8Info {
                    tag,
                    length,
                    bytes: read_u8_vec(&bytes, &mut *offset, length as usize),
                }
            }
            CONSTANT_INTEGER => { todo!() }
            CONSTANT_FLOAT => { todo!() }
            CONSTANT_LONG => { todo!() }
            CONSTANT_DOUBLE => { todo!() }
            CONSTANT_CLASS => {
                CpInfo::ConstantClassInfo {
                    tag,
                    name_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_STRING => { todo!() }
            CONSTANT_FIELDREF => { todo!() }
            CONSTANT_METHODREF => {
                CpInfo::ConstantMethodrefInfo {
                    tag,
                    class_index: read_u16(&bytes, &mut *offset),
                    name_and_type_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_INTERFACE_METHODREF => { todo!() }
            CONSTANT_NAME_AND_TYPE => {
                CpInfo::ConstantNameAndTypeInfo {
                    tag,
                    name_index: read_u16(&bytes, &mut *offset),
                    descriptor_index: read_u16(&bytes, &mut *offset),
                }
            }
            CONSTANT_METHOD_HANDLE => { todo!() }
            CONSTANT_METHOD_TYPE => { todo!() }
            CONSTANT_DYNAMIC => { todo!() }
            CONSTANT_INVOKE_DYNAMIC => { todo!() }
            CONSTANT_MODULE => { todo!() }
            CONSTANT_PACKAGE => { todo!() }
            _ => panic!("unsupported tag {}", tag)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldsInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl FieldsInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> FieldsInfo {
        let access_flags: u16 = read_u16(&bytes, &mut *offset);
        let name_index: u16 = read_u16(&bytes, &mut *offset);
        let descriptor_index: u16 = read_u16(&bytes, &mut *offset);
        let attributes_count: u16 = read_u16(&bytes, &mut *offset);
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::read(&bytes, &mut *offset));
        }
        FieldsInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> MethodInfo {
        let access_flags: u16 = read_u16(&bytes, &mut *offset);
        let name_index: u16 = read_u16(&bytes, &mut *offset);
        let descriptor_index: u16 = read_u16(&bytes, &mut *offset);
        let attributes_count: u16 = read_u16(&bytes, &mut *offset);
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::read(&bytes, &mut *offset));
        }
        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

impl AttributeInfo {
    fn read(bytes: &[u8], offset: &mut usize) -> AttributeInfo {
        let attribute_name_index: u16 = read_u16(&bytes, &mut *offset);
        let attribute_length: u32 = read_u32(&bytes, &mut *offset);
        let mut info: Vec<u8> = Vec::new();
        for _ in 0..attribute_length {
            let i = read_u8(&bytes, &mut *offset);
            info.push(i)
        }
        AttributeInfo {
            attribute_name_index,
            attribute_length,
            info,
        }
    }
}

fn read_n<const N: usize>(bytes: &[u8], offset: &mut usize) -> [u8; N] {
    let next = *offset + N;
    let a: [u8; N] = bytes[*offset..next].try_into().unwrap();
    *offset = next;
    a
}

fn read_u8(bytes: &[u8], offset: &mut usize) -> u8 {
    u8::from_be_bytes(read_n::<1>(&bytes, &mut *offset))
}

fn read_u16(bytes: &[u8], offset: &mut usize) -> u16 {
    u16::from_be_bytes(read_n::<2>(&bytes, &mut *offset))
}

fn read_u32(bytes: &[u8], offset: &mut usize) -> u32 {
    u32::from_be_bytes(read_n::<4>(&bytes, &mut *offset))
}

fn read_u8_vec(bytes: &[u8], offset: &mut usize, length: usize) -> Vec<u8> {
    let next = *offset + length;
    let a: Vec<u8> = bytes[*offset..next].to_vec();
    *offset = next;
    a
}

#[test]
fn test() {
    let bytes: &[u8] = &[
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x13, 0x0a, 0x00, 0x02, 0x00, 0x03, 0x07,
        0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e,
        0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x0a, 0x00, 0x08, 0x00, 0x09, 0x07, 0x00,
        0x0a, 0x0c, 0x00, 0x0b, 0x00, 0x0c, 0x01, 0x00, 0x07, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x32,
        0x01, 0x00, 0x03, 0x61, 0x64, 0x64, 0x01, 0x00, 0x05, 0x28, 0x49, 0x49, 0x29, 0x49, 0x01, 0x00,
        0x04, 0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62,
        0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x01, 0x00, 0x04, 0x70, 0x72, 0x6f, 0x67, 0x01, 0x00,
        0x03, 0x28, 0x29, 0x49, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c,
        0x65, 0x01, 0x00, 0x0c, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x32, 0x2e, 0x6a, 0x61, 0x76, 0x61,
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
        0x00, 0x01, 0x00, 0x11, 0x00, 0x00, 0x00, 0x02, 0x00, 0x12];

    let class_file = ClassFile::load(bytes);

    assert_eq!(class_file, ClassFile {
        magic: [0xca, 0xfe, 0xba, 0xbe],
        minor_version: [0x00, 0x00],
        major_version: [0x00, 0x3e],
        constant_pool_count: 19,
        constant_pool: vec![
            CpInfo::ConstantMethodrefInfo { tag: 10, class_index: 2, name_and_type_index: 3 },
            CpInfo::ConstantClassInfo { tag: 7, name_index: 4 },
            CpInfo::ConstantNameAndTypeInfo { tag: 12, name_index: 5, descriptor_index: 6 },
            CpInfo::ConstantUtf8Info { tag: 1, length: 16, bytes: vec![0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74] },
            CpInfo::ConstantUtf8Info { tag: 1, length: 6, bytes: vec![0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e] },
            CpInfo::ConstantUtf8Info { tag: 1, length: 3, bytes: vec![0x28, 0x29, 0x56] },
            CpInfo::ConstantMethodrefInfo { tag: 10, class_index: 8, name_and_type_index: 9 },
            CpInfo::ConstantClassInfo { tag: 7, name_index: 10 },
            CpInfo::ConstantNameAndTypeInfo { tag: 12, name_index: 11, descriptor_index: 12 },
            CpInfo::ConstantUtf8Info { tag: 1, length: 7, bytes: vec![0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x32] },
            CpInfo::ConstantUtf8Info { tag: 1, length: 3, bytes: vec![0x61, 0x64, 0x64] },
            CpInfo::ConstantUtf8Info { tag: 1, length: 5, bytes: vec![0x28, 0x49, 0x49, 0x29, 0x49] },
            CpInfo::ConstantUtf8Info { tag: 1, length: 4, bytes: vec![0x43, 0x6f, 0x64, 0x65] },
            CpInfo::ConstantUtf8Info { tag: 1, length: 15, bytes: vec![0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65] },
            CpInfo::ConstantUtf8Info { tag: 1, length: 4, bytes: vec![0x70, 0x72, 0x6f, 0x67] },
            CpInfo::ConstantUtf8Info { tag: 1, length: 3, bytes: vec![0x28, 0x29, 0x49] },
            CpInfo::ConstantUtf8Info { tag: 1, length: 10, bytes: vec![0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65] },
            CpInfo::ConstantUtf8Info { tag: 1, length: 12, bytes: vec![0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x32, 0x2e, 0x6a, 0x61, 0x76, 0x61] },
        ],
        access_flags: 32,
        this_class: 8,
        super_class: 2,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 3,
        methods: vec![
            MethodInfo {
                access_flags: 0x00,
                name_index: 0x05,
                descriptor_index: 0x06,
                attributes_count: 0x01,
                attributes: vec![
                    AttributeInfo {
                        attribute_name_index: 0x0d,
                        attribute_length: 0x1d,
                        info: vec![0, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01],
                    }
                ],
            },
            MethodInfo {
                access_flags: 0x09,
                name_index: 0x0f,
                descriptor_index: 0x10,
                attributes_count: 0x01,
                attributes: vec![
                    AttributeInfo {
                        attribute_name_index: 0x0d,
                        attribute_length: 0x31,
                        info: vec![0, 0x02, 0x00, 0x03, 0x00, 0x00, 0x00, 0x0d, 0x04, 0x3b, 0x10, 0x2a, 0x3c, 0x1a, 0x1b, 0xb8, 0x00, 0x07, 0x3d, 0x1c, 0xac, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x12, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x02, 0x00, 0x05, 0x00, 0x05, 0x00, 0x06, 0x00, 0x0b, 0x00, 7],
                    }],
            },
            MethodInfo {
                access_flags: 0x09,
                name_index: 0x0b,
                descriptor_index: 0x0c,
                attributes_count: 0x01,
                attributes: vec![
                    AttributeInfo {
                        attribute_name_index: 0x0d,
                        attribute_length: 0x1c,
                        info: vec![0, 0x02, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x1a, 0x1b, 0x60, 0xac, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0b],
                    }],
            },
        ],
        attributes_count: 1,
        attributes: vec![
            AttributeInfo {
                attribute_name_index: 17,
                attribute_length: 2,
                info: vec![0x00, 0x12],
            }],
    })
}