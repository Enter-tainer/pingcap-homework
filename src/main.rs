use ahash::AHashMap;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use std::io::BufReader;
use std::{
    fs,
    io::{prelude::*, BufWriter},
    path::Path,
};
use std::{fs::File, os::unix::prelude::MetadataExt};
mod config;
mod hash;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let default_filename = String::from("urls.10G.txt");
    let filename = args.get(1).unwrap_or(&default_filename);
    let slice_count = split_file(&filename)?;
    let mut heap = read_and_process_pieces(slice_count)?;
    let mut res = heap.drain().collect::<Vec<_>>();
    res.sort();

    for i in res.iter().take(100) {
        println!("{}", i.0 .0);
        println!("{}", String::from_utf8_lossy(&i.0 .1));
    }
    Ok(())
}

/// This function read the original file and split it into pieces which is not larger than 500MB.
///
/// Specifically, it use BufReader with a huge buffer size, and process the file line by line. It will calculate hash of the string, and use it to determine which piece it will go to.
///
/// It returns the number of the pieces.
fn split_file<P: AsRef<Path>>(input: P) -> std::io::Result<u64> {
    let f = File::open(input)?;
    let meta = f.metadata()?;
    let file_size = meta.size();
    let slice_count: u64 = (file_size as f64 / config::PIECE_SIZE as f64).ceil() as u64;
    let buffer_size: usize = config::WRITE_BUFFER_SIZE / slice_count as usize;
    let _err = fs::create_dir("data");
    let mut file_handles: Vec<BufWriter<File>> = Vec::new();
    for i in 0..slice_count {
        let f = File::create(format!("data/url{}", i))?;
        file_handles.push(BufWriter::with_capacity(buffer_size, f));
    }
    let reader = BufReader::with_capacity(config::READ_BUFFER_SIZE, f);
    reader.split(b'\n').for_each(|f| {
        let buf = f.unwrap();
        let hash = hash::hash(&buf);
        let bucket_number = hash % slice_count;
        let writer = &mut file_handles[bucket_number as usize];
        let _err = writer.write_all(&buf);
        let _err = writer.write_all(b"\n");
    });
    Ok(slice_count)
}

/// UrlHeap is a type alias for the heap that store the url, and its frequency
type UrlHeap = BinaryHeap<Reverse<(usize, Vec<u8>)>>;

/// This function read every pieces generated by `split_file`, and process it.
///
/// Specifically, it reads pieces one by one, and use a binary heap to maintain the top `TOP_URL_COUNT`(100 in this program) entries.
///
/// It returns a heap containing the top `TOP_URL_COUNT` urls.
fn read_and_process_pieces(slice_count: u64) -> std::io::Result<UrlHeap> {
    let mut heap: UrlHeap = BinaryHeap::new();
    for i in 0..slice_count {
        let f = File::open(format!("data/url{}", i))?;
        let mut file_reader = BufReader::with_capacity(config::SLICE_READ_BUFFER_SIZE, f);
        let mut sub_heap = process_piece(&mut file_reader)?;
        heap.append(&mut sub_heap);
        while heap.len() > config::TOP_URL_COUNT {
            heap.pop();
        }
    }
    Ok(heap)
}

/// This function actually process a piece.
///
/// It read the file line by line, and add each line into a `HashMap`.
/// And then use a heap to select the top `TOP_URL_COUNT`(100 in this program) entries
///
/// It returns a heap containing top `TOP_URL_COUNT` urls in this piece.
fn process_piece(reader: &mut BufReader<File>) -> std::io::Result<UrlHeap> {
    let mut map: AHashMap<Vec<u8>, usize> = AHashMap::new();
    reader.split(b'\n').for_each(|f| {
        let buf = f.unwrap();
        let cnt = map.entry(buf).or_insert(0);
        *cnt += 1;
    });
    let mut heap: UrlHeap = BinaryHeap::new();
    for (k, v) in map.drain() {
        // drain the map to avoid unnecessary data copy
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
