use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::os::unix::fs::FileExt;

pub fn file_search(file: File, token: [u8; 40], end: u64) -> Option<usize> {
    let mut low: u64 = 0;
    let mut high: u64 = end - 1;
    let mut iterations: u32 = 0;
    let buf: &mut [u8; 40] = &mut [0; 40];

    while low <= high {
        iterations += 1;
        let mid: u64 = (low + high) / 2;

        match file_seek(&file, buf, mid) {
            Err(e) => {
                println!(
                    "Failed to read {} bytes at position {} due to error {}.",
                    buf.len(),
                    mid,
                    e
                );
                return None;
            }
            Ok(_) => match token.cmp(buf) {
                Ordering::Equal => {
                    println!(
                        "Found token {:?} after {} iterations",
                        str::from_utf8(&token).unwrap(),
                        iterations
                    );
                    return Some(usize::try_from(mid).unwrap());
                }
                Ordering::Less => high = mid - 1,
                Ordering::Greater => low = mid + 1,
            },
        }
    }
    None
}

pub fn range_search(array: &[u8], token: [u8; 40], end: u64) -> Option<usize> {
    let mut low: u64 = 0;
    let mut high: u64 = end - 1;
    let mut iterations: u32 = 0;
    let buf: &mut [u8; 40] = &mut [0; 40];

    while low <= high {
        iterations += 1;
        let mid: u64 = (low + high) / 2;

        match array_seek(array, buf, mid) {
            Err(e) => {
                println!(
                    "Failed to read {} bytes at position {} due to error {}.",
                    buf.len(),
                    mid,
                    e
                );
                return None;
            }
            Ok(_) => match token.cmp(buf) {
                Ordering::Equal => {
                    println!(
                        "Found token {:?} after {} iterations",
                        str::from_utf8(&token).unwrap(),
                        iterations
                    );
                    return Some(usize::try_from(mid).unwrap());
                }
                Ordering::Less => high = mid - 1,
                Ordering::Greater => low = mid + 1,
            },
        }
    }
    None
}

fn file_seek(file: &File, buf: &mut [u8; 40], mid: u64) -> io::Result<()> {
    let res = file.read_exact_at(buf, mid);
    let colon_index = buf.iter().position(|&r| r == b':');
    let newline_index = buf.iter().position(|&r| r == b'\n');
    let index: u64;

    if colon_index.is_some() {
        index = colon_index.unwrap().try_into().unwrap();
    } else if newline_index.is_some() {
        index = newline_index.unwrap().try_into().unwrap();
    } else {
        return res;
    }
    let offset = mid + index - 40;
    file_seek(file, buf, offset)
}

fn array_seek(array: &[u8], buf: &mut [u8; 40], mid: u64) -> io::Result<()> {
    let mid_usize = mid as usize;
    if mid_usize + 40 > array.len() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "read out of bounds"));
    }
    buf.copy_from_slice(&array[mid_usize..mid_usize + 40]);

    let colon_index = buf.iter().position(|&r| r == b':');
    let newline_index = buf.iter().position(|&r| r == b'\n');
    let index: u64;

    if colon_index.is_some() {
        index = colon_index.unwrap().try_into().unwrap();
    } else if newline_index.is_some() {
        index = newline_index.unwrap().try_into().unwrap();
    } else {
        return Ok(());
    }
    let offset = mid + index - 40;
    array_seek(array, buf, offset)
}
