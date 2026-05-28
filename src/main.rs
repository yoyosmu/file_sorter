use std::fs;

fn main() -> std::io::Result<()> {

    let downloads = dirs::download_dir().unwrap();
    let paths = fs::read_dir(&downloads).unwrap();

    let pdf_dir = downloads.join("PDFs");
    let img_dir = downloads.join("IMGs");
    let audio_dir = downloads.join("AUDIOs");
    let archive_dir = downloads.join("ARCHIVEs");
    
    fs::create_dir_all(&pdf_dir)?;
    fs::create_dir_all(&img_dir)?;
    fs::create_dir_all(&audio_dir)?;
    fs::create_dir_all(&archive_dir)?;
    
    for entry in paths {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_string_lossy();
        let images = ["jpg", "png", "webp", "gif", "avif", "tiff", "bmp", "raw", "heif", "heic"];
        let docs = ["docx", "pdf", "doc", "odt", "txt", "rtf", "xps", "xlsx", "csv", "ods"];
        let audio = ["mp3", "mp4", "mp5", "wav", "aac", "flac", "ogg", "wma", "aiff", "m4a", "dts", "opus"];
        let archive = ["zip", "tar", "gz", "bz2", "7z", "rar", "xz", "lzh", "lha", "taz", "pkg", "deb", "tgz", "lzip"];

        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            
            if docs.contains(&ext.as_str()) {
                    let dest = pdf_dir.join(file_name.as_ref());
                    fs::rename(&path, &dest)?;
                }
            else if images.contains(&ext.as_str()) {
                    let dest = img_dir.join(file_name.as_ref());            
                    fs::rename(&path, &dest)?;
                }     
            else if audio.contains(&ext.as_str()) {
                    let dest = audio_dir.join(file_name.as_ref());                 
                    fs::rename(&path, &dest)?;
                }
            else if archive.contains(&ext.as_str()) {
                    let dest = archive_dir.join(file_name.as_ref());                 
                    fs::rename(&path, &dest)?;
                }    
        }
    }
    Ok(())
}
