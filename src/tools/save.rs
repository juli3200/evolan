use std::{fs::File, io};
use rayon::iter::Take;
use tar::Builder;
use lz4_flex;
use walkdir::WalkDir;

///
/// creates files with either extension .envolan or .evolan1
/// .evolan1 files are tar files
/// .evolan files arer .tar.lz4 files
/// 

static TAR_PATH: &str ="cache/0.evolan1";

fn archive(input_path: &String) -> Result<(), Box<dyn std::error::Error>>{
    // create tar file at output location
    // add a 1 to the file
    // eg project.evolan1
    // indicates that it isn't compressed
    // is stored in the cache

    let tar_file =File::create(TAR_PATH)?;

    let mut archive = Builder::new(tar_file);


     // Iterate through the directory and add files to the TAR archive
     for entry in WalkDir::new(&input_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            println!("done: {}", file_name);
            archive.append_path_with_name(path, file_name)?;
        }
    }


    Ok(())
}

fn compress(path: &String) ->  Result<(), Box<dyn std::error::Error>>{
    // open the previously created tar_file
    let mut tar_file = File::open(TAR_PATH)?;
    let lz4_file = File::create(path)?;

    let mut lz4_compressed = lz4_flex::frame::FrameEncoder::new(lz4_file);

    io::copy(&mut tar_file, &mut lz4_compressed)?;

    lz4_compressed.finish()?;

    drop(tar_file);

    std::fs::remove_file(TAR_PATH)?;

    Ok(())
}

/*
param input_path: is a path to folder
param output_path: path to compressed archive ends with .evolan
fn: calls archive and compress 
return: Ok or Error
*/
pub fn save(input_path: &String, output_path: &String) -> Result<(), Box<dyn std::error::Error>>{
    archive(input_path)?;
    compress(output_path)?;

    Ok(())
}