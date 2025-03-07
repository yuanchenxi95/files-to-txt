use clap::{arg, Command};
use files_to_text::{FileToTextOptions, FilesToText};

fn cli() -> Command {
    Command::new("ftt")
        .about("File to text cli")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("gen")
                .about("write the contents of all the files to the target file")
                .arg(
                    arg!(-o --output_file <OUTPUT_FILE> "The output file to write")
                )
                .arg(
                    arg!(-i --include <INCLUDED_FILES> "The files to include")
                        .value_delimiter(','),

                )
                .arg(arg!(-e --exclude <EXCLUDED_FILES> "The files to exclude").value_delimiter(','))
                .arg(arg!(<SOURCE_DIR> "The source directory to read all the files"))
                .arg_required_else_help(true),
        )
}


fn main() {
    let matches = cli().get_matches();
    
    match matches.subcommand() {
        Some(("gen", sub_matches)) => {
            let input_dir = sub_matches.get_one::<String>("SOURCE_DIR").unwrap();
            let output_file = sub_matches.get_one::<String>("output_file").unwrap();
            let include_files = sub_matches
                .get_many::<String>("include")
                .map(|v| v.map(|s| s.as_str()).collect::<Vec<&str>>());
            let exclude_files = sub_matches
                .get_many::<String>("exclude")
                .map(|v| v.map(|s| s.as_str()).collect::<Vec<&str>>());
            let result = FilesToText::read_to_file(FileToTextOptions {
                source_folder: input_dir,
                includes: include_files.unwrap_or_else(||Vec::new()),
                excludes: exclude_files.unwrap_or_else(||Vec::new()),
            }, output_file);

            match result {
                Ok(_) => {
                    println!("Done!");
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
        _ => unreachable!(),
    }
}
