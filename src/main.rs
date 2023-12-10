use std::process::exit;

use clap::{Parser, Subcommand};
use wax::{Glob, Pattern};

#[derive(Debug, Parser)]
#[command(name = "unrparc",author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        arg_required_else_help = true,
        about = "Scans files in the '.rpa' archive.\n\
        It outputs the files and their size into standard out.\n\
        If archive is not valid it will output an error."
    )]
    Scan {
        #[arg(help = "Location of the '.rpa' archive")]
        path: String,
        #[arg(short, long, help = "Glob pattern to filter the files by")]
        glob: Option<String>,
    },
    #[command(
        arg_required_else_help = true,
        about = "Extracts files from '.rpa' archive"
    )]
    Extract {
        #[arg(help = "Location of the '.rpa' archive")]
        source: String,
        #[arg(help = "The folder to extract the files into. Created if it doesn't exist")]
        destination: String,
        #[arg(short, long, help = "Glob pattern to extract only required files")]
        glob: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Scan { path, glob } => {
            let mut reader = get_file_reader(path);
            let files = match unrparc::scan(&mut reader) {
                Ok(files) => files,
                Err(error) => {
                    println!("{}", error);
                    exit(1)
                }
            };

            let mut glob_pattern: Option<Glob<'_>> = None;
            if let Some(glob) = glob.as_ref() {
                glob_pattern = Some(Glob::new(glob.as_str()).unwrap());
            }

            for file in files {
                if let Some(glob_pattern) = &glob_pattern {
                    if glob_pattern.is_match(file.name.as_str()) {
                        println!("{}: {} bytes", file.name, file.size);
                    }
                } else {
                    println!("{}: {} bytes", file.name, file.size);
                }
            }
        }
        Commands::Extract {
            source,
            destination,
            glob,
        } => {
            if !std::path::Path::new(&destination).exists() {
                std::fs::create_dir(&destination).unwrap();
            }

            let mut reader = get_file_reader(source);

            let files_res = match glob {
                Some(glob) => unrparc::extract_glob(glob.as_str(), &mut reader),
                None => unrparc::extract(&mut reader),
            };

            let files = match files_res {
                Ok(files) => files,
                Err(err) => {
                    println!("{}", err);
                    exit(1);
                }
            };

            let destination = std::path::Path::new(&destination);
            for (file, data) in files {
                let file_path = destination.join(file.name);
                if let Err(err) = std::fs::write(file_path, data) {
                    println!("{}", err);
                };
            }
        }
    }
}

fn get_file_reader(path: String) -> std::io::BufReader<std::fs::File> {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) => {
            println!("{}", error);
            exit(1)
        }
    };
    std::io::BufReader::new(file)
}
