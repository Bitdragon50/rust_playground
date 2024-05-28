
fn main(){
    println!("{:#?}", to_alternating_case("Hello wORld"))
}

fn to_alternating_case(s: &str) -> String {
    s.chars().map(|char| {
        match char {
            'a'..='z' => char.to_uppercase().to_string(),
            'A'..='Z' => char.to_lowercase().to_string(),
            _ => char.to_string()
        }
    }).collect()
}

/* 
fn to_alternating_case1(s: &str) -> String {
    s.chars().map(|char| {
        match char.is_alphabetic() {
            false => char.to_string(),
            true => {
                if char.is_ascii_lowercase() {
                    char.to_uppercase().to_string()
                } else {
                    char.to_lowercase().to_owned().to_string()
                }
            }
        }
    }).collect()
}
*/