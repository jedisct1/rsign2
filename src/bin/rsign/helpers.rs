use minisign::*;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::BufWriter;
#[cfg(not(windows))]
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

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

#[cfg(windows)]
pub fn create_file<P>(path: P, _mode: u32) -> Result<BufWriter<File>>
where
    P: AsRef<Path>,
{
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| {
            PError::new(
                ErrorKind::Io,
                format!("while creating: {} - {}", path.as_ref().display(), e),
            )
        })?;
    Ok(BufWriter::new(file))
}

#[cfg(not(windows))]
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
        .open(path)
        .map_err(|e| {
            PError::new(
                ErrorKind::Io,
                format!("while creating: {} - {}", path.display(), e),
            )
        })?;
    Ok(BufWriter::new(file))
}
