use std::{env, fs};
use std::path::PathBuf;
use std::{io, fmt};
use regex::Regex;
use reqwest::StatusCode;
use std::error::Error;

fn main() {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("docs");

    scan_dir(current_dir);
}

fn scan_dir(dir: PathBuf) -> Result<String, io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        read_file(path);
    }

    Ok(String::new())
}

fn read_file(path: PathBuf) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path.as_path())?;

    let re = Regex::new(r#"((http|ftp|https)://([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:/~+#-]*[\w@?^=%&/~+#-])?)"#).unwrap();

    for str_match in re.captures_iter(&contents) {
        match str_match.get(1) {
            Some(res) => {
                let url = res.as_str();
                match test_url(url) {
                    Ok(_) => println!("src: {} - {} - OK", path.to_str().unwrap(), url),
                    Err(_) => println!("{} - Failed", url),
                };
            },
            None => panic!("This should never happen - if we get a match .get(1) should be fine"),
        }
        println!("Match in {} - {}", path.to_str().unwrap(), str_match.get(1).unwrap().as_str());
    }

    Ok(contents)
}

#[derive(Debug)]
struct UrlTestError {
    url: String,
    problem: String,
}

impl fmt::Display for UrlTestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not reach url '{}'. Problem: {}", self.url, self.problem)
    }
}

impl Error for UrlTestError {}


fn test_url(url: &str) -> Result<(), UrlTestError> {
    let resp = match reqwest::blocking::get(url) {
        Ok(resp) => resp,
        Err(_) => return Err(UrlTestError{url: url.to_string(), problem: "Failed to make request".to_string()}),
    };

    if resp.status() == StatusCode::NOT_FOUND {
        return Err(UrlTestError{url: url.to_string(), problem: "Not found".to_string()});
    }

    Ok(())
}

// fn get_diff() {
//     let repo = match Repository::open("/home/lewis/rust/simple") {
//         Ok(repo) => repo,
//         Err(e) => panic!("Failed to open repo: {}", e),
//     };

//     let head = match repo.head() {
//         Ok(head) => head,
//         Err(e) => panic!("Failed to get head: {}", e),
//     };

//     println!("Head ref is: {}", head.name().unwrap());

//     println!("Head commit is: {}", head.peel_to_commit().unwrap().id());


//     let diff = match repo.diff_tree_to_tree(Some(&head.peel_to_tree().unwrap()), None, None) {
//         Ok(diff) => diff,
//         Err(e) => panic!("Failed to create diff: {}", e),
//     };

//     diff.foreach(
//         &mut |d, i| -> bool {
//             println!("file_cb: {} - {}", d.old_file().path().unwrap().to_str().unwrap(), i);
//             true
//         },
//         None,
//         None,
//         None
//         // &mut |d, b| -> bool {
//         //     println!("binary_cb: {} - {}", d, b)
//         //     true
//         // },
//         // &mut | d, h | -> bool {
//         //     println!("hunk_cb: {} -{}", d, h)
//         //     true
//         // },
//         // &mut |d, l| -> bool {
//         //     println!("line_cb: {} - {}", d, l)
//         //     true
//         // }
//     ).unwrap();
// }