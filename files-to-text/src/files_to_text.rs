use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use crate::file_reader::FileReader;
use crate::options::FileToTextOptions;

pub struct FilesToText;

impl FilesToText {
    pub fn read_all(options: FileToTextOptions) -> String {
        let mut buf_writer = BufWriter::new(Vec::new());
        let mut file_reader = FileReader::new(&mut buf_writer, options);
        file_reader.read_to_output().unwrap();
        buf_writer.flush().unwrap();
        return String::from_utf8(buf_writer.get_ref().clone()).unwrap();
    }

    pub fn read_to_file(options: FileToTextOptions, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let file = File::create(path)?;
        let mut buf_writer = BufWriter::new(file);
        let mut file_reader = FileReader::new(&mut buf_writer, options);
        file_reader.read_to_output()?;
        buf_writer.flush()?;

        Ok(())
    }
}
