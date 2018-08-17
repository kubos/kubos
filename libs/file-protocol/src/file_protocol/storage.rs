use super::*;

// Save new chunk in a temporary storage file
pub fn store_chunk(hash: &str, index: u32, data: &[u8]) -> Result<(), String> {
    // if data is type uint8_t[]
    // change data to ffi.string
    let file_name = format!("{:x}", index);
    let storage_path = Path::new("storage").join(hash).join(file_name);

    // may need to check directory existence
    fs::create_dir_all(&storage_path.parent().unwrap()).unwrap();
    let mut file = File::create(&storage_path).unwrap();
    file.write_all(data).unwrap();

    Ok(())
}

pub fn store_meta(hash: &str, num_chunks: u32) -> Result<(), String> {
    let data = vec![("num_chunks", num_chunks)];

    // let mut e = Encoder::from_memory();
    // e.encode(&data).unwrap();
    let vec = to_vec(&data).unwrap();

    let file_dir = Path::new("storage").join(hash);
    // Make sure the directory exists
    fs::create_dir_all(file_dir.clone())
        .map_err(|err| format!("Failed to create temp storage directory: {:?}", err))?;

    let meta_path = file_dir.join("meta");
    let temp_path = file_dir.join(".meta.tmp");

    File::create(&temp_path).unwrap().write_all(&vec).unwrap();

    fs::rename(temp_path, meta_path).unwrap();

    Ok(())
}

// Load a chunk from its temporary storage file
pub fn load_chunk(hash: &str, index: u32) -> Result<Vec<u8>, String> {
    let mut data = vec![];
    let path = Path::new("storage").join(hash).join(format!("{:x}", index));

    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    Ok(data)
}

pub fn load_meta(hash: &str) -> Result<Option<u32>, String> {
    let mut data = vec![];
    let meta_path = Path::new("storage").join(hash).join("meta");
    File::open(meta_path)
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();

    // cbor decode meta
    // let _d = Decoder::from_bytes(data);

    // TODO: Get real value
    Ok(Some(10))
}

// Verify that the local chunk files match the expected hash?
pub fn local_sync(hash: &str, num_chunks: Option<u32>) -> Result<(bool, Vec<u32>), String> {
    if let Some(num) = num_chunks {
        store_meta(hash, num).unwrap();
    } else {
        let _num_chunks = match load_meta(hash) {
            Ok(d) => match d {
                Some(d) => d,
                None => return Ok((false, vec![0, 1])),
            },
            Err(e) => return Err(format!("failed loading meta {:?}", e)),
        };
    }

    let mut _bits: Vec<u8> = vec![];

    let _hash_path = Path::new("storage").join(hash);

    // TODO
    Ok((false, vec![0, 1]))
}

/// Create temporary folder for chunks
/// Stream copy file from mutable space to immutable space
/// Move folder to hash of contents
pub fn local_import(source_path: &str) -> Result<(String, u32, u16), String> {
    let storage_path = String::from("storage");

    if let Err(e) = fs::metadata(source_path) {
        return Err(format!("failed to stat file {}: {:?}", source_path, e));
    }

    // Copy input file to storage area and calculate hash
    if let Err(e) = fs::create_dir_all(&storage_path) {
        return Err(format!("failed to create dir {}: {:?}", storage_path, e));
    }

    let temp_path = Path::new(&storage_path).join(format!(".{}", time::get_time().nsec));
    let mut hasher = Blake2s::new(16);
    {
        let mut input = File::open(&source_path).unwrap();
        let mut output = File::create(&temp_path).unwrap();

        // Need to bring in blake2fs here to create hash
        loop {
            let mut chunk = vec![0u8; CHUNK_SIZE];
            match input.read(&mut chunk) {
                Ok(n) => {
                    if n == 0 {
                        output.sync_all().unwrap();
                        break;
                    }
                    hasher.update(&chunk[0..n]);
                    if let Err(e) = output.write(&chunk[0..n]) {
                        return Err(format!("failed to write chunk {:?}", e));
                    }
                }
                Err(e) => return Err(format!("failed to read chunk from source {:?}", e)),
            }
        }
    }
    let hash_result = hasher.finalize();
    let mut hash = String::from("");
    for c in hash_result.as_bytes().iter() {
        hash = format!("{}{:02x}", hash, c);
    }

    let mut output = match File::open(&temp_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!(
                "failed to open temp file {:?} : {:?}",
                temp_path, e
            ))
        }
    };

    let mut index = 0;
    let mut offset = 0;

    loop {
        let mut chunk = vec![0u8; CHUNK_SIZE];
        match output.read(&mut chunk) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                store_chunk(&hash, index, &chunk[0..n]).unwrap();
                index = index + 1;
                offset = offset + n;
            }
            Err(e) => {
                return Err(format!(
                    "failed to read chunk from temp {:?}: {:?}",
                    temp_path, e
                ))
            }
        }
    }
    store_meta(&hash, index).unwrap();

    Ok((hash, index, 0))
}

// Copy temporary data chunks into permanent file?
pub fn local_export(_hash: &str, _target_path: &str, _mode: Option<u16>) -> Result<(), String> {
    println!("Implement me: local_export");
    Ok(())
}
