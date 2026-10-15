use std::{fs::File, io::{self, ErrorKind::UnexpectedEof, Read, Write}};

const MAX_U8: usize = u8::MAX as usize;
pub fn compress(data: &[u8]) -> Vec<(u8, u8)> {
    let mut encode = Vec::<(u8, u8)>::with_capacity(10);
    if data.is_empty() {
        return encode;
    }
    let mut count = 1usize; // menghitung byte yang sama
    let mut current_index = 0; // index untuk menggambil value didalam array
    loop {
        if (current_index + count) >= data.len() {
            encode.push((count as u8, data[current_index]));
            break;
        }
        // membandingkan dua byte, dari current_index dengan current_index ditambah count
        if data[current_index] == data[current_index + count] && count < MAX_U8 {
            count += 1;
        } else {
            encode.push((count as u8, data[current_index]));
            current_index += count;
            count = 1;
            continue;
        };
    }
    encode
}

pub fn save_compress(path: &str, data: &[(u8, u8)]) -> io::Result<()> {
    let mut file = File::create_new(path)?;
    let mut buff = [0u8; 2];
    for &(counter, c) in data {
        buff[0] = counter;
        buff[1] = c;
        file.write_all(&buff)?;
    }
    Ok(())
}

pub fn decompress(data: &[(u8, u8)]) -> Vec<u8> {
    let mut decode = Vec::<u8>::with_capacity(10);
    if data.is_empty() {
        return decode;
    };
    for &row in data {
        for _line in 0..row.0 {
            decode.push(row.1);
        }
    }
    decode
}
pub fn load_compress(path: &str) -> io::Result<Vec<(u8, u8)>> {
    let mut decode = Vec::<(u8, u8)>::with_capacity(10);
    let mut file_compress = File::open(path)?;
    let mut buff: [u8; 2] = [0u8; 2];
    loop {
        let n = loop {
            match file_compress.read(&mut buff[..1]) {
                Ok(n) => break n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        };
        if n == 0 {
            break;
        }
        if let Err(e) = file_compress.read_exact(&mut buff[1..]) {
            match e.kind() {
                UnexpectedEof => {
                    return Err(io::Error::new(
                        UnexpectedEof,
                        format!("{} invalid format data", path),
                    ));
                }
                _ => return Err(e),
            }
        }
        if buff[0] == 0 {
            return Err(io::Error::new(
                UnexpectedEof,
                format!("{} invalid format data", path),
            ));
        }
        decode.push((buff[0], buff[1]));
    }
    Ok(decode)
}


pub fn compress_file(
    src_path: &str,
    output_path: &str
) -> io::Result<()> {
    let mut buff = Vec::new();
    File::open(src_path)?.read_to_end(&mut buff)?;
    let data_compress = compress(&buff);
    save_compress(output_path, &data_compress)
}

pub fn decompress_file(
    src_path: &str,
    output_path: &str
) -> io::Result<()> {
    let data_compress = load_compress(src_path)?;
    let data_decompress = decompress(&data_compress);
    File::create_new(output_path)?.write_all(&data_decompress)
}