use dex;
use dex::{Dex, DexReader};
use rc_zip::{prelude::*, EntryContents};
use std::fmt;
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

struct ApkArchive {
    archive: rc_zip::Archive,
    file: fs::File,
}

type DexFile = Dex<Vec<u8>>;

#[allow(dead_code)]
pub struct Apk {
    pub path: String,
    apk_archive: ApkArchive,
    pub dex_files: Vec<DexFile>,
}

#[derive(Debug)]
struct ApkPathIsInvalid {
    file: PathBuf,
}

impl fmt::Display for ApkPathIsInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("The apk {:?} is not a valid path.", self.file))
    }
}

impl std::error::Error for ApkPathIsInvalid {}

impl Apk {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let s = {
            let s = path.as_ref().to_str();
            match s {
                Some(s) => String::from(s),
                None => {
                    return Err(Box::new(ApkPathIsInvalid {
                        file: path.as_ref().to_path_buf(),
                    }))
                }
            }
        };
        let file = fs::File::open(&path)?;
        let archive = file.read_zip()?;

        let apk_archive = ApkArchive { archive, file };
        let dex_files_result = apk_archive.load_dex_files();

        let mut dex_files: Vec<DexFile> = Vec::new();
        for dex in dex_files_result {
            match dex {
                Ok(dex) => dex_files.push(dex),
                Err(e) => return Err(e),
            }
        }

        Ok(Apk {
            path: s,
            apk_archive: apk_archive,
            dex_files: dex_files,
        })
    }
}

impl ApkArchive {
    fn load_dex_files(&self) -> Vec<Result<DexFile, Box<dyn std::error::Error>>> {
        let mut dex_files: Vec<Result<DexFile, Box<dyn std::error::Error>>> = Vec::new();
        for name in self.archive.entries() {
            if name.name().ends_with(".dex") {
                dex_files.push(self.load_dex_file(name.name()))
            }
        }
        dex_files
    }

    fn load_dex_file(&self, dexname: &str) -> Result<DexFile, Box<dyn std::error::Error>> {
        let mut bytearray: Vec<u8> = Vec::new();
        read_file_contents(&self, dexname, &mut bytearray)?;
        let dex = DexReader::from_vec(bytearray)?;
        Ok(dex)
    }
}

fn read_file_contents(
    apk: &ApkArchive,
    filename: &str,
    buf: &mut Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let EntryContents::File(f) = apk.archive.by_name(filename).unwrap().contents() {
        let mut r = f
            .entry
            .reader(|offset| positioned_io::Cursor::new_pos(&apk.file, offset));
        r.read_to_end(buf)?;
        Ok(())
    } else {
        Err(Box::new(DexFileIsNotFileError {
            file: String::from(filename),
        }))
    }
}

#[derive(Debug)]
struct DexFileIsNotFileError {
    file: String,
}

impl fmt::Display for DexFileIsNotFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "The dex {} is not actually a file in the apk.",
            self.file
        ))
    }
}

impl std::error::Error for DexFileIsNotFileError {}

#[cfg(test)]
mod tests {
    use super::{Apk, *};
    use dex::DexReader;

    #[test]
    fn create_struct() -> Result<(), Box<dyn std::error::Error>> {
        let _ = Apk::from_path("resources/test01.apk")?;
        Ok(())
    }

    #[test]
    fn get_dex() -> Result<(), Box<dyn std::error::Error>> {
        let apk = Apk::from_path("resources/test01.apk")?;
        let dex = DexReader::from_file("resources/testapk/classes.dex")?;
        assert_eq!(
            apk.dex_files[0].header().checksum(),
            dex.header().checksum()
        );
        Ok(())
    }
}
