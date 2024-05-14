use std::{ fs::{self, File}, io::{Error, Read, Write}};

fn main() {
    let text = read_file("src/example.txt");
    println!("{:#?}",text);
    let success_state = write_file("src/output.txt", text.unwrap());
}



fn read_file(file_path: &str) -> Result<String, Error>{
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);
    Ok(buffer)
}

fn write_file(file_path: &str, text: String) -> Result<(),Error>{
    let mut file = match File::create(file_path){
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    write!(file,"{}", text)
}

