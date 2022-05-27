use std::io::prelude::*;
use zip::write::FileOptions;
use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use zip::result;

// pub fn create_zip_all(filepaths: Vec<&str>, mut output_path: String, zip_filename: String) -> zip::result::ZipResult<()>{
//     if output_path.chars().last().unwrap() != '/'{
//         output_path += "/";
//     }
    
//     for filepath in filepaths {

//     }
// }

fn calculate_path_of_first_part(filepath: &str) -> usize{
    let mut path_of_last_cnt = 0;
    let rev_path = filepath.chars().rev();
    for c in rev_path {
        if c != '/' {
            path_of_last_cnt += 1;
        }else{
            break;
        }
    }
    filepath.len() - path_of_last_cnt
}

pub fn create_zip(filepath: &str, mut output_path: String) -> zip::result::ZipResult<String> {

    if output_path.chars().last().unwrap() != '/' {
        output_path = output_path + "/";
    }
    create_dir_all(&output_path)?;

    let path = Path::new(filepath);
    let zip_filename = path.file_stem().unwrap().to_str().unwrap().to_string() + ".zip"; 
    let file = match std::fs::File::create(output_path.clone() + &zip_filename){
        Ok(file) => file,
        Err(e) => return Err(result::ZipError::Io(e)),
    };
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    let first_part_of_path = calculate_path_of_first_part(filepath);

    if path.is_file() {
        let buf = match read_file(filepath){
            Ok(buf) => buf,
            Err(e) => return Err(e),
        };
        zip.start_file(path.file_name().unwrap().to_str().unwrap(), options)?;
        println!("{}", path.file_name().unwrap().to_str().unwrap());
        zip.write_all(&buf)?;
    }else{
        let mut paths = vec![];
        visit_dir(filepath, paths.as_mut())?;
        println!("{:?}", paths);
        for i in 0..paths.len() {
            let origin_path = &paths[i];
            let origin_path = Path::new(&origin_path);
            let path = paths[i].to_string_lossy();
            let path = Path::new(&path[first_part_of_path..]);
            println!("{:?}", path);
            if origin_path.is_file(){
                let buf = match read_file(origin_path.to_str().unwrap()){
                    Ok(buf) => buf,
                    Err(e) => return Err(e),
                };
                zip.start_file(path.to_path_buf().to_string_lossy(), options)?;
                zip.write_all(&buf)?;
            }
        }
        zip.finish()?;
    }

    Ok(output_path + &zip_filename)
}

fn read_file(filename: &str) -> zip::result::ZipResult<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn extract_zip(filename: &str) -> result::ZipResult<String> {

    let fname = std::path::Path::new(filename);
    let file = fs::File::open(&fname);
    let file = match file {
        Ok(f) => f,
        Err(e) => return Err(result::ZipError::Io(e))
    };
    let mut unzip_msg: String = String::new();

    let archive = zip::ZipArchive::new(file);
    let mut archive = match archive {
        Ok(zipfile) => zipfile,
        Err(e) => return Err(e)
    };

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        if (*file.name()).ends_with('/') {
            //println!("File {} extracted to \"{}\"", i, outpath.display());
            unzip_msg += &format!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            unzip_msg += &format!("File {} extracted to \"{}\" ({} bytes)", i, outpath.display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    Ok(unzip_msg)
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