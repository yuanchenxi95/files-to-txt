#[derive(PartialEq, Debug, Clone)]
pub struct FileToTextOptions<'a> {
    pub source_folder: &'a str,
    pub includes: Vec<&'a str>,
    pub excludes: Vec<&'a str>,
}
