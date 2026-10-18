use std::{
    env, fs::File, io::{self, Read, Write}, path::{Path, PathBuf},
};
pub mod cli;
pub mod encryption;

pub struct PositionDir(PathBuf);
impl PositionDir {
    pub fn init() -> Self {
        let path = env::current_dir()
            .expect("cannot access this file, because file not found or cannot get permissions");
        PositionDir(path)
    }
    pub fn get_dir_run(&self) -> &PathBuf {
        &self.0
    }
}

pub fn compress_file(src_path: &Path, output_path: &Path) -> io::Result<()> {
    let mut buff = Vec::new();
    File::open(src_path)?.read_to_end(&mut buff)?;
    let data_compress = encryption::BasicEncryption::compress(&buff);
    encryption::BasicEncryption::save_compress(output_path, &data_compress)
}

pub fn decompress_file(src_path: &Path, output_path: &Path) -> io::Result<()> {
    let data_compress = encryption::BasicEncryption::load_compress(src_path)?;
    let data_decompress = encryption::BasicEncryption::decompress(&data_compress);
    File::create_new(output_path)?.write_all(&data_decompress)
}
