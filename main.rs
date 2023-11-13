use std::fs;
use std::io::{Read, Write};
use bzip2::write::BzEncoder;
use bzip2::Compression;
use std::thread;

fn compress_file(file_path: &str) {
    let mut input_file = fs::File::open(file_path).expect("Unable to open file");
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer).expect("Unable to read file");

    let output_path = format!("{}.bz2", file_path);
    let output_file = fs::File::create(output_path).expect("Unable to create output file");
    let mut encoder = BzEncoder::new(output_file, Compression::best());
    encoder.write_all(&buffer).expect("Unable to compress file");
}

fn main() {
    let entries = fs::read_dir(".");

    let handles: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.expect("Unable to read directory entry");
            let file_name = entry.file_name();
            if let Some(file_name) = file_name.to_str() {
                if file_name.ends_with(".bsp") {
                    let file_path = entry.path().into_os_string().into_string().unwrap();
                    Some(thread::spawn(move || {
                        compress_file(&file_path);
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
