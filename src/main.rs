use chrono::{DateTime, Local};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;
fn main() {
    let raw = std::env::args().nth(1);
    let default = String::from(".");
    let arg = raw.unwrap_or(default);

    if let Err(ref e) = run(Path::new(&arg)) {
        println!("{}", e);
        process::exit(1)
    }

    process::exit(0)
}

fn run(dir: &Path) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;

            if entry.path().is_dir() {
                let dirname = entry
                    .file_name()
                    .into_string()
                    .or_else(|f| Err(format!("Invalid: {:?}", f)))?;
                let metadata = entry.metadata()?;
                let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

                println!(
                    "{: <10} | {} | {}",
                    "Directory",
                    modified.format("%_d %b %H:%M").to_string(),
                    dirname
                );

                continue;
            }

            let filename = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid: {:?}", f)))?;

            let metadata = entry.metadata()?;
            let size = metadata.len();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

            println!(
                "{: <4} Bytes | {} | {}",
                size,
                modified.format("%_d %b %H:%M").to_string(),
                filename
            );
        }
    }

    Ok(())
}
