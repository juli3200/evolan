use std::{fs::File, io};
use tar::Builder;
use lz4_flex;
use walkdir::WalkDir;

///
/// creates files with either extension .envolan or .evolan1
/// .evolan1 files are tar files
/// .evolan files arer .tar.lz4 files
/// 


fn archive(input_path: &String, output_path: &String) -> Result<(), Box<dyn std::error::Error>>{
    // create tar file at output location
    // add a 1 to the file
    // eg project.evolan1
    // indicates that it isn't compressed
    let tar_path = format!("{}1", output_path);

    let tar_file =File::create(&tar_path)?;

    let mut archive = Builder::new(tar_file);


     // Iterate through the directory and add files to the TAR archive
     for entry in WalkDir::new(&tar_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            println!("done 22{}", file_name);
            archive.append_path_with_name(path, file_name)?;
        }
    }


    Ok(())
}

fn compress(path: &String) ->  Result<(), Box<dyn std::error::Error>>{
    // open the previously created tar_file
    let mut tar_file = File::open(format!("{}1", path))?;
    let lz4_file = File::create(path)?;

    let mut lz4_compressed = lz4_flex::frame::FrameEncoder::new(lz4_file);

    io::copy(&mut tar_file, &mut lz4_compressed)?;

    lz4_compressed.finish()?;

    drop(tar_file);

    std::fs::remove_file(format!("{}1", path))?;

    Ok(())
}

/*
param input_path: is a path to folder
param output_path: path to compressed archive ends with .evolan
fn: calls archive and compress 
return: Ok or Error
*/
pub fn save(input_path: &String, output_path: &String) -> Result<(), Box<dyn std::error::Error>>{
    archive(input_path, output_path)?;
    compress(output_path)?;

    Ok(())
}