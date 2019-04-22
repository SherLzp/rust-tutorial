#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate tar;

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::fs::file;
use tar::Archive;
use std::path::PathBuf;

error_chain! {
  foreign_links {
    Io(std::io::Error);
    StripPrefixError(::std::path::StripPrefixError);
  }
}

pub fn decompressTarball() -> Result<(), std::io::Error> {
    let path = "archive.tar.gz";

    let tar_gz = File::open(path);
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}

pub fn compressDirectoryIntoTarball() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive.tar.gz");
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}

pub fn decompressTarballWhileRemovingPrefix() -> Result<(), std::io::Error> {
    let file = File::open("archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "bundle/logs";

    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}