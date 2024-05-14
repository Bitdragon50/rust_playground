use std::fs;

fn read_file(file_path: &str) -> String{
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?  
}




fn write_file(){}