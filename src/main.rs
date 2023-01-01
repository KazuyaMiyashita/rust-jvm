pub mod class_file;

use class_file::reader;

fn main() {
    let class_file = reader::read_class_file(&[]);

    println!("{:?}", class_file);

    println!("Hello, world!");
}
