use ahash::{AHashMap, AHasher};
use config::SLICE_SIZE;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use std::{
    fs,
    io::{prelude::*, BufWriter},
    path::Path,
};
use std::{
    fs::{read, File},
    os::unix::prelude::MetadataExt,
};
use std::{hash::Hasher, io::BufReader};
mod config;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let default_filename = String::from("urls.10G.txt");
    let filename = args.get(1).unwrap_or(&default_filename);
    let slice_count = split_file(&filename)?;
    let mut heap = read_and_process_slices(slice_count)?;
    let mut res = heap.drain().collect::<Vec<_>>();
    res.sort();

    for i in res.iter().take(3) {
        println!("{}", i.0 .0);
        println!("{}", String::from_utf8_lossy(&i.0 .1));
    }
    Ok(())
}

fn split_file<P: AsRef<Path>>(input: P) -> std::io::Result<u64> {
    let f = File::open(input)?;
    let meta = f.metadata()?;
    let file_size = meta.size();
    let slice_count: u64 = (file_size as f64 / SLICE_SIZE as f64).ceil() as u64;
    let buffer_size: usize = config::WRITE_BUFFER_SIZE / slice_count as usize;
    let _err = fs::create_dir("data");
    let mut file_handles: Vec<BufWriter<File>> = Vec::new();
    for i in 0..slice_count {
        let f = File::create(format!("data/url{}", i))?;
        file_handles.push(BufWriter::with_capacity(buffer_size, f));
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

    Ok(slice_count)
}

fn read_and_process_slices(
    slice_count: u64,
) -> std::io::Result<BinaryHeap<Reverse<(usize, Vec<u8>)>>> {
    let mut heap: BinaryHeap<Reverse<(usize, Vec<u8>)>> = BinaryHeap::new();
    for i in 0..slice_count {
        let f = File::open(format!("data/url{}", i))?;
        let mut file_reader = BufReader::with_capacity(config::SLICE_READ_BUFFER_SIZE, f);
        let mut sub_heap = process_slice(&mut file_reader)?;
        heap.append(&mut sub_heap);
        while heap.len() > config::TOP_URL_COUNT {
            heap.pop();
        }
    }
    Ok(heap)
}

fn process_slice(
    reader: &mut BufReader<File>,
) -> std::io::Result<BinaryHeap<Reverse<(usize, Vec<u8>)>>> {
    let mut map: AHashMap<Vec<u8>, usize> = AHashMap::new();
    loop {
        let mut buf: Vec<u8> = Vec::new();
        let len = reader.read_until(b'\n', &mut buf)?;
        if len == 0 {
            break;
        }
        let cnt = map.entry(buf).or_insert(0);
        *cnt += 1;
    }
    let mut heap = BinaryHeap::new();
    for (k, v) in map.drain() {
        if heap.len() < config::TOP_URL_COUNT {
            heap.push(Reverse((v, k)));
        } else {
            let top = heap.peek();
            if let Some(top) = top {
                if top.0 .0 < v {
                    heap.pop();
                    heap.push(Reverse((v, k)));
                }
            }
        }
    }
    Ok(heap)
}
