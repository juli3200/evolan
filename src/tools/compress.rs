
use rar:: Archive;
use std::path::Path;

pub fn compress(path: String){

    let folder_to_compress = &format!("{}", path);
    let output_rar = "./output_archive.rar";

    let archive = Archive::into(self);

    if let Ok(mut writer) = archive.archive(output_rar) {
        if let Err(err) = writer.append_path_with_name(Path::new(folder_to_compress), "") {
            eprintln!("Failed to compress folder: {}", err);
        } else {
            if let Err(err) = writer.finalize() {
                eprintln!("Failed to save archive: {}", err);
            } else {
                println!("Folder compressed to {}", output_rar);
            }
        }
    } else {
        eprintln!("Failed to create archive");
    }
}