use std::path::{Path, PathBuf};

#[test]
fn test_compress() {
    use rust_compress_basic::encryption::BasicEncryption;

    let comp = BasicEncryption::compress(&[65, 66, 66, 67]);
    assert_eq!(vec![(1, 65), (2, 66), (1, 67)], comp);

    let comp = BasicEncryption::compress(&[65, 66, 33, 77, 77, 77, 7, 77, 77, 77]);
    assert_eq!(
        vec![(1, 65), (1, 66), (1, 33), (3, 77), (1, 7), (3, 77)],
        comp
    );
    let mut data = Vec::new();
    for _ in 1..=256 {
        data.push(65u8);
    }
    let comp = BasicEncryption::compress(&data);
    assert_eq!(vec![(255, 65), (1, 65)], comp);
}

#[test]
fn test_decompress() {
    use rust_compress_basic::encryption::BasicEncryption;
    let dummy: Vec<(u8, u8)> = vec![(3, 66), (100, 70), (255, 60), (1, 60)];

    let decompress = BasicEncryption::decompress(&dummy);
    let first = decompress.iter().filter(|&&val| val == 66).count() as u8;
    assert_eq!(dummy[0].0, first);

    let second = decompress.iter().filter(|&&val| val == 70).count() as u8;
    assert_eq!(dummy[1].0, second);

    let thrid = decompress.iter().filter(|&&val| val == 60).count();
    assert_eq!(256, thrid);
}

#[test]
fn test_save_compress() {
    use rust_compress_basic::encryption::BasicEncryption;
    let dummy: Vec<(u8, u8)> = vec![(3, 66), (100, 70), (255, 60), (1, 60)];
    let compress = BasicEncryption::save_compress(&PathBuf::from("coba.lre"), &dummy);
    assert!(compress.is_ok());
}

#[test]
fn test_load_compress() {
    use rust_compress_basic::encryption::BasicEncryption;
    let load = BasicEncryption::load_compress(&PathBuf::from("coba.lre"));
    assert!(load.is_ok());
}

#[test]
fn test_compress_file() {
    use rust_compress_basic::compress_file;
    let result = compress_file(&PathBuf::from("hello.txt"), &PathBuf::from("hello(1).lre"));
    assert!(result.is_ok());
}

#[test]
fn test_decompress_file() {
    use rust_compress_basic::decompress_file;
    let result = decompress_file(
        &PathBuf::from("hello(1).lre"),
        &PathBuf::from("hello(2).txt"),
    );
    assert!(result.is_ok());
}
