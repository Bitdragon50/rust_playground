use std::fs;
use std::path::Path;

fn main() {
    let dir = Path::new("/path/to/directory");
    let pattern = "pattern*.txt"; // change to your desired pattern
}


fn find_like_file(dir: &Path, pattern: &str) {
    for entry in fs::read_dir(dir) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.file_name().unwrap().to_string_lossy() == pattern {
                    println!("{}", path.display());
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}