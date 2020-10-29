use std::{env, fs};
use std::fs::DirEntry;
use std::path::PathBuf;
use std::io;
use regex::Regex;

mod checker;

fn main() {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("test_data");

    scan_dir(current_dir);
}

fn scan_dir(dir: PathBuf) {
    match fs::read_dir(dir.clone()) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                handle_dir_entry(entry);
            }
        },
        Err(e) => println!("Failed to read dir: {} - {}", dir.to_str().unwrap(), e),
    };    
}

fn handle_dir_entry(entry: DirEntry) {
    let file_type = entry.file_type().expect("failed to get file_type");

    if file_type.is_dir() {
        scan_dir(entry.path());
    }  else {
        read_file(entry.path());
    }
}

fn read_file(path: PathBuf) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path.as_path())?;

    let re = Regex::new(r#"((http|ftp|https)://([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:/~+#-]*[\w@?^=%&/~+#-])?)"#).unwrap();

    for str_match in re.captures_iter(&contents) {
        match str_match.get(1) {
            Some(res) => {
                let url = res.as_str();
                match checker::test_url(url) {
                    Ok(_) => println!("src: {} - {} - OK", path.to_str().unwrap(), url),
                    Err(_) => println!("{} - Failed", url),
                };
            },
            None => panic!("This should never happen - if we get a match .get(1) should be fine"),
        }
    }

    Ok(contents)
}
