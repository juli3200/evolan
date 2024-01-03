use std::{fs::File, io};
use rayon::iter::Take;
use tar::Builder;
use lz4_flex;
use walkdir::WalkDir;

///
/// creates files with extension .envolan 
/// .evolan files arer .tar.lz4 files
/// 


fn archive(name: &str) -> Result<(), Box<dyn std::error::Error>>{
    // create tar file at output location
    // add a 1 to the file
    // eg project.evolan1
    // indicates that it isn't compressed
    // is stored in the cache

    let tar_file =File::create(format!("cache/worlds/{name}.tar"))?;

    let mut archive = Builder::new(tar_file);


     // Iterate through the directory and add files to the TAR archive
     for entry in WalkDir::new(format!("cache/worlds/{name}")).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            println!("done: {}", file_name);
            archive.append_path_with_name(path, file_name)?;
        }
    }


    Ok(())
}

fn compress(name: &str, path: &str) ->  Result<(), Box<dyn std::error::Error>>{
    // open the previously created tar_file
    let mut tar_file = File::open(format!("cache/worlds/{name}.tar"))?;
    let lz4_file = File::create(path)?;

    let mut lz4_compressed = lz4_flex::frame::FrameEncoder::new(lz4_file);

    io::copy(&mut tar_file, &mut lz4_compressed)?;

    lz4_compressed.finish()?;

    drop(tar_file);

    std::fs::remove_file(format!("cache/worlds/{name}.tar"))?;

    Ok(())
}

/*
param input_path: is a path to folder
param output_path: path to compressed archive ends with .evolan
fn: calls archive and compress 
return: Ok or Error
*/
pub fn save(name: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>>{
    archive(name)?;
    compress(name, output_path)?;

    Ok(())
}