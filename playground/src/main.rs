use files_to_text::{FileToTextOptions, FilesToText};

fn main() {
    let options = FileToTextOptions {
        source_folder: "./",
        includes: vec![".rs"],
        excludes: Vec::new(),
    };
    let text = FilesToText::read_all(options);
    println!("{}", text);
}
