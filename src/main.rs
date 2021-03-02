use ahash::{AHashMap, AHasher};
use config::SLICE_SIZE;
use rayon::prelude::*;
use std::{
    fs,
    io::{prelude::*, BufWriter},
};
use std::{
    fs::{read, File},
    os::unix::prelude::MetadataExt,
};
use std::{hash::Hasher, io::BufReader};
mod config;
fn main() -> std::io::Result<()> {
    let f = File::open("urls.1G.txt")?;
    let meta = f.metadata()?;
    let file_size = meta.size();
    let slice_count: u64 = (file_size as f64 / SLICE_SIZE as f64).ceil() as u64;
    let _err = fs::create_dir("data");
    let mut file_handles: Vec<BufWriter<File>> = Vec::new();
    for i in 0..slice_count {
        let f = File::create(format!("data/url{}", i))?;
        file_handles.push(BufWriter::with_capacity(config::WRITE_BUFFER_SIZE, f));
    }
    let mut reader = BufReader::with_capacity(config::READ_BUFFER_SIZE, f);
    loop {
        let mut buf: Vec<u8> = Vec::new();
        let len = reader.read_until(b'\n', &mut buf)?;
        if len == 0 {
            break;
        }
        let mut hasher = AHasher::default();
        hasher.write(&buf);
        let hash = hasher.finish();
        let bucket_number = hash % slice_count;
        let writer = &mut file_handles[bucket_number as usize];
        writer.write_all(&buf)?;
    }

    Ok(())
}
