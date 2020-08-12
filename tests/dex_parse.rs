extern crate dex;

use dex::{DexReader, Result};
use memmap::MmapOptions;
use std::fs::File;
use std::path::Path;

#[test]
fn hello_world() {
    assert!(true);
}

#[test]
fn test_find_class_by_name() {
    let dex = DexReader::from_file("resources/classes.dex").expect("cannot open dex file");
    let mut count = 0;
    for class_def in dex.class_defs() {
        let class_def = class_def.expect("can't load class");
        let jtype = dex.get_type(class_def.class_idx()).expect("bad type");
        let result = dex.find_class_by_name(&jtype.type_descriptor().to_string());
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
        count += 1;
    }
    assert!(count > 0);
}

fn load_example_dex_as_vec<P: AsRef<Path>>(file: P) -> Result<Vec<u8>> {
    let map = unsafe { MmapOptions::new().map(&File::open(file.as_ref())?)? };
    let data = map.to_vec();
    Ok(data)
}

#[test]
fn test_find_class_by_name_from_vec() {
    let data: Vec<u8> = load_example_dex_as_vec("resources/classes.dex")
        .expect("Cannot load example file to a vec");
    let dex = DexReader::from_vec(data).expect("Cannot parse dex from vec");
    let mut count = 0;
    for class_def in dex.class_defs() {
        let class_def = class_def.expect("can't load class");
        let jtype = dex.get_type(class_def.class_idx()).expect("bad type");
        let result = dex.find_class_by_name(&jtype.type_descriptor().to_string());
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
        count += 1;
    }
    assert!(count > 0);
}
