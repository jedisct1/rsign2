use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{BufReader, BufWriter};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use minisign::*;

pub fn open_data_file<P>(data_path: P) -> Result<BufReader<File>>
where
    P: AsRef<Path>,
{
    let data_path = data_path.as_ref();
    let file = OpenOptions::new()
        .read(true)
        .open(data_path)
        .map_err(|e| PError::new(ErrorKind::Io, e))?;

    Ok(BufReader::new(file))
}

pub fn create_dir<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    DirBuilder::new()
        .recursive(true)
        .create(&path)
        .map_err(|e| {
            PError::new(
                ErrorKind::Io,
                format!("while creating: {} - {}", path.as_ref().display(), e),
            )
        })?;
    Ok(())
}

#[cfg(not(unix))]
pub fn create_file<P>(path: P, _mode: u32) -> Result<BufWriter<File>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| {
            PError::new(
                ErrorKind::Io,
                format!("while creating: {} - {}", path.display(), e),
            )
        })?;
    Ok(BufWriter::new(file))
}

#[cfg(unix)]
pub fn create_file<P>(path: P, mode: u32) -> Result<BufWriter<File>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let file = OpenOptions::new()
        .mode(mode)
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| {
            PError::new(
                ErrorKind::Io,
                format!("while creating: {} - {}", path.display(), e),
            )
        })?;
    Ok(BufWriter::new(file))
}

pub fn create_sig_file<P>(path: P) -> Result<BufWriter<File>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| {
            PError::new(
                ErrorKind::Io,
                format!("while creating: {} - {}", path.display(), e),
            )
        })?;
    Ok(BufWriter::new(file))
}

pub fn unix_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("system clock is incorrect");
    since_the_epoch.as_secs()
}

pub fn is_printable(s: &str) -> bool {
    for c in s.chars() {
        match c {
            '\t' => continue,
            '\x00'..='\x1f' | '\x7f'..='\u{9f}' => return false,
            _ => continue,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printable_normal_text() {
        assert!(is_printable("hello world"));
        assert!(is_printable("timestamp:1234567890"));
    }

    #[test]
    fn test_printable_rejects_escape_sequence() {
        assert!(!is_printable("\x1b[31mred\x1b[0m"));
    }

    #[test]
    fn test_printable_rejects_control_characters() {
        assert!(!is_printable("\x00"));
        assert!(!is_printable("a\x07b"));
        assert!(!is_printable("\x7f"));
    }

    #[test]
    fn test_printable_rejects_newlines() {
        assert!(!is_printable("line1\nline2"));
        assert!(!is_printable("line1\r\nline2"));
    }

    #[test]
    fn test_printable_allows_tab() {
        assert!(is_printable("a\tb"));
    }

    #[test]
    fn test_printable_allows_unicode() {
        assert!(is_printable("héllo 世界"));
    }

    #[test]
    fn test_printable_rejects_c1_control() {
        assert!(!is_printable("\u{80}"));
        assert!(!is_printable("\u{9f}"));
    }
}
