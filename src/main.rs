use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use walkdir::WalkDir;
use rayon::prelude::*;

fn main() -> io::Result<()> {
    // Retrieve the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 4 {
        eprintln!("Usage: {} <directory> <target_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let search_dir = &args[1];
    let target_file = &args[2];
    let output_file = &args[3];
    // Open or create the output file, truncating it if it already exists
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_file)?;
    let mut writer = BufWriter::new(file);

    // Write the header line
    writeln!(writer, "\"parent_directory\",\"line\"")?;

    // Collect all entries first to enable parallel processing
    let entries: Vec<_> = WalkDir::new(search_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            if let Some(file_name) = e.file_name().to_str() {
                file_name == target_file
            } else {
                false
            }
        })
        .collect();

    // Use parallel iterator for processing files
    let results: Vec<_> = entries.par_iter()
        .map(|entry| process_file(entry))
        .collect();

    // Write results to the output file
    for result in results {
        match result {
            Ok(lines) => {
                for (parent_dir, line) in lines {
                    writeln!(writer, "\"{}\",\"{}\"", parent_dir, line)?;
                }
            },
            Err(e) => eprintln!("Failed to process file: {}", e),
        }
    }

    println!("Processing complete.");
    Ok(())
}

fn process_file(entry: &walkdir::DirEntry) -> io::Result<Vec<(String, String)>> {
    let path = entry.path();
    let parent_dir = path.parent().unwrap().to_string_lossy().to_string();
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        results.push((parent_dir.clone(), line));
    }
    Ok(results)
}
