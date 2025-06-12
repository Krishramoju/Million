use std::fs::{File, metadata};
use std::io::{Read, Write, BufReader, BufWriter};
use flate2::write::GzEncoder;
use flate2::Compression;

/// Compress a file and return (original_size, compressed_size) in bytes
pub fn compress_file(input_path: &str, output_path: &str) -> std::io::Result<(u64, u64)> {
    // Get original file size
    let original_size = metadata(input_path)?.len();

    let input_file = File::open(input_path)?;
    let buffered_reader = BufReader::new(input_file);

    let output_file = File::create(output_path)?;
    let buffered_writer = BufWriter::new(output_file);

    let mut encoder = GzEncoder::new(buffered_writer, Compression::default());
    let mut buffer = Vec::new();

    // Read all input into buffer
    buffered_reader.take(10_000_000).read_to_end(&mut buffer)?;

    // Write compressed data
    encoder.write_all(&buffer)?;
    encoder.finish()?; // Finish encoding

    // Get compressed file size
    let compressed_size = metadata(output_path)?.len();

    Ok((original_size, compressed_size))
}
