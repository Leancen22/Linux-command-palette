use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

pub fn cp_command(source: &str, destination: &str) -> io::Result<()> {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);

    if source_path.is_file() {
        let mut source_file = BufReader::new(File::open(source_path)?);
        let mut dest_file = BufWriter::new(File::create(dest_path.join(source_path.file_name().unwrap()))?);
        io::copy(&mut source_file, &mut dest_file)?;
    } else {
        let mut entries = fs::read_dir(source_path)?.peekable();

        if !dest_path.exists() {
            fs::create_dir_all(&dest_path)?;
        }

        while let Some(entry) = entries.next() {
            let entry = entry?;

            let mut path = entry.path();
            let mut dest = dest_path.to_path_buf();

            if entry.file_type()?.is_dir() {
                path.push(".");
                dest.push(entry.file_name());
                fs::create_dir_all(&dest)?;
                entries = fs::read_dir(&path)?.peekable();
            } else {
                dest.push(entry.file_name());
                let mut source_file = BufReader::new(File::open(&path)?);
                let mut dest_file = BufWriter::new(File::create(&dest)?);

                io::copy(&mut source_file, &mut dest_file)?;
            }
        }
    }

    Ok(())
}
