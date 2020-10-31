use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use regex::Regex;
use checker::{TestResult};
use colored::*;
use clap::{App, Arg};

mod checker;

fn main() {
    let args = App::new("Doclink Checker")
                    .author("Lewis Boon")
                    .about("Check documentation, like Markdown, for broken links")
                    .arg(Arg::with_name("INPUT")
                        .help("The input directory, or file, to check")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input = args.value_of("INPUT").unwrap_or(".");

    let input = Path::new(input);
    if !input.exists() {
        println!("File doesn't exist: {}", input.to_string_lossy());
        std::process::exit(1);
    }

    if input.metadata().unwrap().is_dir() {
        scan_dir(input);
    } else {
        read_file(input.to_path_buf());
    }
}

/// Loops through the contents of the directory. Errors are printed, but not propagated.
fn scan_dir(dir: &Path) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                handle_dir_entry(entry);
            }
        },
        Err(e) => println!("Failed to read dir: {} - {}", dir.to_str().unwrap(), e),
    };    
}

/// Handles whether it is file or directory
fn handle_dir_entry(entry: DirEntry) {
    let file_type = entry.file_type().expect("failed to get file_type");

    if file_type.is_dir() {
        scan_dir(&entry.path());
    }  else {
        read_file(entry.path());
    }
}

/// Finds all the URLs in a file
fn read_file(path: PathBuf) {
    let contents = fs::read_to_string(path.as_path())
        .unwrap_or_else(|_| panic!("failed to read file: '{}'", path.to_str().unwrap()));

    let re = Regex::new(r#"((http|ftp|https)://([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:/~+#-]*[\w@?^=%&/~+#-])?)"#).unwrap();

    for str_match in re.captures_iter(&contents) {
        let str_match = str_match.get(1).expect("failed to get first match out of a Match");
        let url = str_match.as_str();
        handle_test_result(path.to_str().unwrap(), url, checker::test_url(url));
    }
}

/// Handles the result of testing a URL. Currently just prints the result.
fn handle_test_result(path: &str, url: &str, result: TestResult) {
    print!("{} - {} - ", path.blue(), url);

    match result {
        TestResult::Ok => println!("{}", "OK".green()),
        TestResult::NotFound => println!("{}", "Not Found".red()),
        TestResult::Redirect(redirect) => println!("{}: {}", "Redirect Found".yellow(), redirect),
        TestResult::Error(e) => println!("{}: {}", "Failed".red(), e),
    }
}