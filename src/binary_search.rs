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

pub fn range_search(array: &[u8], token: [u8; 35], end: u64) -> Option<usize> {
    let mut low: u64 = 0;
    let mut high: u64 = end - 1;
    let mut iterations: u32 = 0;
    let buf: &mut [u8; 35] = &mut [0; 35];

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
    let line_start = if mid == 0 {
        0
    } else {
        let mut pos = mid;
        let mut byte = [0u8; 1];
        loop {
            pos -= 1;
            file.read_exact_at(&mut byte, pos)?;
            if byte[0] == b'\n' {
                break pos + 1;
            }
            if pos == 0 {
                break 0;
            }
        }
    };
    file.read_exact_at(buf, line_start)
}

fn array_seek(array: &[u8], buf: &mut [u8; 35], mid: u64) -> io::Result<()> {
    let mid = mid as usize;
    let line_start = match array[..mid].iter().rposition(|&b| b == b'\n') {
        Some(pos) => pos + 1,
        None => 0,
    };
    if line_start + 35 > array.len() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "read out of bounds"));
    }
    buf.copy_from_slice(&array[line_start..line_start + 35]);
    Ok(())
}
