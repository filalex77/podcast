use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::PathBuf;


pub fn already_downloaded(dir: &str) -> BTreeSet<String> {
    let mut result = BTreeSet::new();

    let mut path = get_podcast_dir();
    path.push(dir);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                match entry.file_name().into_string() {
                    Ok(val) => {
                        result.insert(String::from(val.trim_right_matches(".mp3")));
                    }
                    Err(err) => {
                        println!("OsString: {:?} couldn't be converted to String", err);
                    }
                }
            }
        }
    }
    result
}

pub fn get_podcast_dir() -> PathBuf {
    match env::var_os("PODCAST") {
        Some(val) => PathBuf::from(val),
        None => {
            let mut path = env::home_dir().unwrap();
            path.push("Podcasts");
            path
        }
    }
}
