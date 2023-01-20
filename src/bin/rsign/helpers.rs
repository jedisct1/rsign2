use std::fs::{File, OpenOptions};
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

#[cfg(not(unix))]
pub fn create_file<P>(path: P, _mode: u32, force: bool) -> Result<BufWriter<File>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            PError::new(
                ErrorKind::Io,
                format!("while creating: {} - {}", path.display(), e),
            )
        })?;
    }
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .create_new(!force)
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
pub fn create_file<P>(path: P, mode: u32, force: bool) -> Result<BufWriter<File>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            PError::new(
                ErrorKind::Io,
                format!("while creating: {} - {}", path.display(), e),
            )
        })?;
    }
    let file = OpenOptions::new()
        .mode(mode)
        .write(true)
        .create(true)
        .create_new(!force)
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
