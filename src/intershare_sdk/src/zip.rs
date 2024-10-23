use std::{fs::File, io::BufReader, path::Path};

use zip::ZipArchive;

use crate::convert_os_str;

pub fn unzip_file(zip_file: File, destination: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Open the zip file
    let mut archive = ZipArchive::new(BufReader::new(zip_file))?;
    let mut written_files = vec![];

    // Iterate over the zip file contents
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = Path::new(destination).join(file.name());
        written_files.push(convert_os_str(out_path.clone().as_os_str()).expect("Failed to convert file path OS string to string"));

        if file.name().ends_with('/') {
            // It's a directory, create it
            std::fs::create_dir_all(&out_path)?;
        } else {
            // It's a file, create the parent directory if needed
            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }

            // Write the file content
            let mut outfile = File::create(&out_path)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        println!("Extracted file to {:?}", out_path);
    }

    Ok(written_files)
}
