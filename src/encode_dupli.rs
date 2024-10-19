//use std::collections::HashMap;

fn duplicate_encode(word:&str)->String {
    let word = word.to_lowercase();
    word.chars().map(|c| if word.len() - word.replace(c, "").len() > 1 { ")".to_string() } else { "(".to_string() }  )
    .collect::<Vec<String>>().join("")
}

fn main(){
    println!("{}", duplicate_encode("recEde"))
}