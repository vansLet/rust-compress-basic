use rust_compress_basic::{compress, decompress, load_compress, save_compress};

fn main() {
    let original = vec![65, 65, 65, 66, 66, 67];

    let compress = compress(&original);

    // let decompress = decompress(&compress);

    save_compress("coba.lre", &compress).unwrap();
    let file_decompress = load_compress("coba.lre").unwrap();
    let decompress = decompress(&file_decompress);



    assert_eq!(original, decompress);

}

