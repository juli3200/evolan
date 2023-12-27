use std::{fs::File, io};
use tar::Archive;
use lz4_flex;

static TAR_PATH: &str ="cache/0.evolan1";

fn extract_archive(output_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let tar_file = File::open(TAR_PATH)?;
    let mut archive = Archive::new(tar_file);

    for entry in archive.entries()? {
        let mut file = entry?;

        let file_path = file.path()?;
        let file_dest = output_path.to_owned() + file_path.to_str().unwrap();

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(output_path.to_owned() + parent.to_str().unwrap())?;
        }

        let mut output_file = File::create(&file_dest)?;
        io::copy(&mut file, &mut output_file)?;
    }

    std::fs::remove_file(TAR_PATH)?;

    Ok(())
}

fn decompress(input_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let lz4_file = File::open(input_path)?;
    let mut tar_file = File::create(TAR_PATH)?;

    let mut lz4_decompressed = lz4_flex::frame::FrameDecoder::new(lz4_file);
    io::copy(&mut lz4_decompressed, &mut tar_file)?;

    Ok(())
}

pub fn load_into_folder(input_path: &String, output_path: &String) -> Result<(), Box<dyn std::error::Error>>{
    decompress(input_path)?;
    extract_archive(&format!("{}/", output_path))?;
    Ok(()) 
}