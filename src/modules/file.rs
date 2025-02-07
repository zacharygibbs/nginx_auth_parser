use std::fs::{OpenOptions, File, read_dir, remove_file};
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use flate2::read::GzDecoder;
use regex::Regex;
use std::io;

pub fn get_dir_files(folder_path: &Path, base_log_file: &str, extension: &str) -> Vec<PathBuf>{
    let mut out_vec: Vec<PathBuf> = vec![];
    for (ind, entry) in read_dir(folder_path)
        .unwrap_or_else(|_| panic!("cant find folder path: {:?}", &folder_path)).enumerate()
    {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            let _file_extension = file_path.extension().unwrap_or_default()
                .to_str().unwrap_or_default();
            let file_name = file_path.file_name().unwrap_or_default()
                .to_str().unwrap_or_default();
            if Regex::new(&format!(
                r"{base_log_file}.*\.\.?[0-9]+?\.?({extension})$"
            )).unwrap().is_match(file_name) {
                println!("{} {} {} {}, {:?}", base_log_file, extension, ind, file_name, &file_path);
                out_vec.push(file_path);
            }
            // if file_extension
        }
    }
    out_vec
}

pub fn combine_files(auth_directory: &Path, base_log_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let gz_files = get_dir_files(auth_directory, base_log_file, "gz");
    let log_files = get_dir_files(auth_directory, base_log_file, "");

    // println!("{:?}", log_files.iter().filter(|p| p.extension().unwrap() == "log").collect());
    // println!("{:?}", gz_files);
    println!("Combining Text Files: {:?}", log_files);
    for text_file in log_files {
        let access_log_path = PathBuf::from(auth_directory);
        if !text_file.eq(&access_log_path) {
            append_file_to_file(access_log_path.join(base_log_file), &text_file).unwrap();
            remove_file(text_file).unwrap()
        }
    }
    for gz_file in gz_files {
        let access_log_path = PathBuf::from(auth_directory);
        append_file_to_file(access_log_path.join(base_log_file), &gz_file).unwrap();
        remove_file(gz_file).unwrap()
    }

    Ok(())
}

pub fn append_file_to_file(file1: PathBuf, file2: &PathBuf) -> Result<(), io::Error> {
    println!("{:?}", file1);
    let mut file1 = OpenOptions::new()
        .append(true)
        .open(&file1)
        .unwrap_or_else(|_| panic!("cannot find file {:?}", &file1));

    if file2.extension().unwrap().to_str().unwrap() == "gz" {
        let file2_str = decode_reader(file2).expect("error on reading file2");
        let _ = file1.write_all(file2_str.as_bytes());
    } else{
        let mut file2 = OpenOptions::new()
            .read(true)
            .open(file2)
            .unwrap_or_else(|_| panic!("cannot find file {:?}", &file2));
        let _ = io::copy(&mut file2, &mut file1);
    }

    Ok(())
}

pub fn decode_reader(gz_file_path: &PathBuf) -> io::Result<String> {
    let gz_file = File::open(gz_file_path)?;
    let mut gz = GzDecoder::new(gz_file);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    Ok(s)
}