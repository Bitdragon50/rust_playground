use std::time::Instant;


fn main(){
    let start = Instant::now();
    println!("{:#?}", reverse_letters("kri1shn666666666a"));
    //do my stuff
    let exec_time = Instant::now().checked_duration_since(start);
    println!("Exec Time: {:#?}", exec_time)

}

fn reverse_letters(s: &str) -> String {
    s.rmatches(char::is_alphabetic).collect()
}
/* 

fn reverse_letters(s: &str) -> String {
    s.chars().rev().filter(|char| char.is_alphabetic()).collect()
}
*/