use std::io::prelude::*;
use zip::write::FileOptions;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

pub fn create_zip(filepath: &str) -> zip::result::ZipResult<String> {
    let path = Path::new(filepath);
    let zip_filepath = path.file_stem().unwrap().to_str().unwrap().to_string() + ".zip";

    let file = std::fs::File::create(&zip_filepath).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    if path.is_file() {
        let buf = match read_file(filepath){
            Ok(buf) => buf,
            Err(e) => return Err(e),
        };
        zip.start_file(path.file_name().unwrap().to_str().unwrap(), options)?;
        zip.write_all(&buf)?;
        zip.finish()?;
    }else{
        let mut paths = vec![];
        visit_dir(filepath, paths.as_mut())?;
        for i in 0..paths.len() {
            let path = Path::new(&paths[i]);
            if path.is_dir(){
                println!("File {} extracted to \"{}\"", i, path.display());
                fs::create_dir_all(&path).unwrap();
            }else{
                let buf = match read_file(path.to_str().unwrap()){
                    Ok(buf) => buf,
                    Err(e) => return Err(e),
                };
                if let Some(p) = path.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }
                zip.start_file(path.to_string_lossy(), options)?;
                zip.write_all(&buf)?;
            }
        }
        zip.finish()?;
    }

    Ok(zip_filepath)
}

fn read_file(filename: &str) -> zip::result::ZipResult<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn extract_zip(filename: &str) -> zip::result::ZipResult<()> {

    let fname = std::path::Path::new(filename);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    Ok(())
}

fn visit_dir<P: AsRef<Path>>(path: P, paths: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            visit_dir(entry.path(), paths)?;
        }
        paths.push(entry.path());
    }
    Ok(())
}