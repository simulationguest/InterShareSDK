use std::{fs::File, io::BufReader, path::Path};

use zip::ZipArchive;

pub fn unzip_file(zip_file: File, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the zip file
    let mut archive = ZipArchive::new(BufReader::new(zip_file))?;

    // Iterate over the zip file contents
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = Path::new(destination).join(file.name());

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

    Ok(())
}
