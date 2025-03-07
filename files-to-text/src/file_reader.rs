use std::{
    fs::File,
    io::{self, Write},
};

use crate::options::FileToTextOptions;
use walkdir::WalkDir;

pub struct FileReader<'a, T: Write> {
    options: FileToTextOptions<'a>,
    write: &'a mut T,
}

impl<'a, T: Write> FileReader<'a, T> {
    pub(crate) fn new(write: &'a mut T, options: FileToTextOptions<'a>) -> Self {
        FileReader { write, options }
    }

    fn read_files(&self) -> Vec<String> {
        return WalkDir::new(&self.options.source_folder)
            .into_iter()
            .filter_entry(|e| {
                let file_type = e.file_type();
                if !file_type.is_file() {
                    return true;
                }

                let file_name = e.file_name().to_str().unwrap_or("");

                for exclude_file in self.options.excludes.iter() {
                    if file_name.contains(exclude_file) {
                        return false;
                    }
                }

                if self.options.includes.is_empty() {
                    return true;
                }

                for include_file in self.options.includes.iter() {
                    if file_name.contains(include_file) {
                        return true;
                    }
                }

                return false;
            })
            .filter_map(|e| {
                let Ok(entry) = e else {
                    return None;
                };

                if !entry.file_type().is_file() {
                    return None;
                }
                let file_path = entry.path().to_str().unwrap_or("");
                return Some(file_path.to_string());
            })
            .collect();
    }

    pub(crate) fn read_to_output(&mut self) -> io::Result<()> {
        let files = self.read_files();
        for file in files {
            let file_path_without_prefix = file.replace(self.options.source_folder, "");
            self.write.write(file_path_without_prefix.as_bytes())?;
            self.write.write("=========================\n".as_bytes())?;
            let mut file = File::open(file)?;
            io::copy(&mut file, &mut self.write)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Cursor};

    use super::*;

    #[test]
    fn test_read_files_recursively() {
        let mut buf_writter = BufWriter::new(Vec::new());
        let file_reader = FileReader::new(
            &mut buf_writter,
            FileToTextOptions {
                source_folder: "./",
                includes: vec![".toml"],
                excludes: Vec::new(),
            },
        );

        // Read files recursively
        let files = file_reader.read_files();

        assert_eq!(files, vec!["./Cargo.toml".to_string()]);
    }

    #[test]
    fn test_read_to_output() {
        let mut buf_writter = BufWriter::new(Vec::new());
        let mut file_reader = FileReader::new(
            &mut buf_writter,
            FileToTextOptions {
                source_folder: "./",
                includes: vec![".toml"],
                excludes: Vec::new(),
            },
        );

        // Read files recursively
        let files = file_reader.read_files();

        assert_eq!(files, vec!["./Cargo.toml".to_string()]);

        // Read files to output
        file_reader.read_to_output().unwrap();

        buf_writter.flush().unwrap();
        let data = buf_writter.get_ref();
        assert_eq!(String::from_utf8(data.clone()).unwrap(), "");
    }
}
