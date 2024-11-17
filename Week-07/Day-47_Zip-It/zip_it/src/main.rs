use std::fs::File;
use std::io::{Read, Write, Result};
use std::collections::HashMap;

const MIN_FRAGMENT_LENGTH: usize = 3;
const MAX_DICTIONARY_SIZE: usize = 256;

fn compress(source_filename: &str, target_filename: &str) -> Result<()> {
    let mut source_file = File::open(source_filename)?;
    let mut buffer = Vec::new();
    source_file.read_to_end(&mut buffer)?;

    let mut dictionary: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut compressed = Vec::new();
    let mut i = 0;

    while i < buffer.len() {
        let mut longest_match = 0;
        let mut longest_index = 0;

        for (index, fragment) in dictionary.iter() {
            if buffer[i..].starts_with(fragment) && fragment.len() > longest_match {
                longest_match = fragment.len();
                longest_index = *index;
            }
        }

        if longest_match >= MIN_FRAGMENT_LENGTH {
            compressed.push(longest_index);
            compressed.push(longest_match as u8);
            i += longest_match;
        } else {
            compressed.push(255); // Marker for literal byte
            compressed.push(buffer[i]);
            i += 1;
        }

        if dictionary.len() < MAX_DICTIONARY_SIZE {
            let end = (i + MIN_FRAGMENT_LENGTH).min(buffer.len());
            let fragment = buffer[i..end].to_vec();
            if fragment.len() >= MIN_FRAGMENT_LENGTH {
                dictionary.insert(dictionary.len() as u8, fragment);
            }
        }
    }

    let mut target_file = File::create(target_filename)?;
    target_file.write_all(&compressed)?;
    Ok(())
}

fn uncompress(source_filename: &str, target_filename: &str) -> Result<()> {
    let mut source_file = File::open(source_filename)?;
    let mut buffer = Vec::new();
    source_file.read_to_end(&mut buffer)?;

    let mut dictionary: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut uncompressed = Vec::new();
    let mut i = 0;

    while i < buffer.len() {
        if buffer[i] == 255 {
            uncompressed.push(buffer[i + 1]);
            i += 2;
        } else {
            let index = buffer[i];
            let length = buffer[i + 1] as usize;
            if let Some(fragment) = dictionary.get(&index) {
                uncompressed.extend_from_slice(&fragment[..length]);
            }
            i += 2;
        }

        if dictionary.len() < MAX_DICTIONARY_SIZE {
            let end = uncompressed.len();
            let start = end.saturating_sub(MIN_FRAGMENT_LENGTH);
            let fragment = uncompressed[start..end].to_vec();
            if fragment.len() >= MIN_FRAGMENT_LENGTH {
                dictionary.insert(dictionary.len() as u8, fragment);
            }
        }
    }

    let mut target_file = File::create(target_filename)?;
    target_file.write_all(&uncompressed)?;
    Ok(())
}

fn main() -> Result<()> {
    compress("input.txt", "compressed.bin")?;
    uncompress("compressed.bin", "output.txt")?;
    Ok(())
}