use cbor::{Decoder, Encoder};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn store_chunk(hash: &str, index: u32, data: &[u8]) -> Result<(), String> {
    // if data is type uint8_t[]
    // change data to ffi.string
    let file_name = format!("{:x}", index);
    let storage_path = Path::new("storage").join(hash).join(file_name);

    // may need to check directory existence
    let mut file = File::create(storage_path).unwrap();
    file.write_all(data).unwrap();

    Ok(())
}

fn store_meta(hash: &str, num_chunks: u32) -> Result<(), String> {
    let data = vec![("num_chunks", num_chunks)];

    let mut e = Encoder::from_memory();
    e.encode(&data).unwrap();

    let meta_path = Path::new("storage").join(hash).join("meta");
    let temp_path = Path::new("storage").join(hash).join(".meta.tmp");

    // may need to check directory existence
    File::create(&temp_path)
        .unwrap()
        .write_all(e.as_bytes())
        .unwrap();

    fs::rename(temp_path, meta_path).unwrap();

    Ok(())
}

fn load_chunk(hash: &str, index: u32) -> Result<Vec<u8>, String> {
    let mut data = vec![];
    let path = Path::new("storage").join(hash).join(format!("{:x}", index));

    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    Ok(data)
}

fn load_meta(hash: &str) -> Result<Option<u32>, String> {
    let mut data = vec![];
    let meta_path = Path::new("storage").join(hash).join("meta");
    File::open(meta_path)
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();

    let mut d = Decoder::from_bytes(data);

    Ok(Some(10))
}

fn local_sync(hash: &str, num_chunks: Option<u32>) -> Result<(bool, u32), String> {
    if let Some(num) = num_chunks {
        store_meta(hash, num);
    } else {
        let num_chunks = match load_meta(hash) {
            Ok(d) => match d {
                Some(d) => d,
                None => return Ok((false, 0)),
            },
            Err(e) => return Err(format!("failed loading meta {:?}", e)),
        };
    }

    let mut bits: Vec<u8> = vec![];

    let hash_path = Path::new("storage").join(hash);

    Ok((true, 0))
}
