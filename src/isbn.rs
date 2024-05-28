use std::time::Instant;
//use rayon::prelude::*;


fn main() {
    let now = Instant::now();
    println!("{:#?}",valid_isbn10("153936559X"));
    println!("{:#?}",now.elapsed());
}

fn valid_isbn10(isbn: &str) -> bool {
    if isbn.len() == 10 && isbn.char_indices().all(|(i, c)| c.is_numeric() || i == 9 && c == 'X')
    {
        isbn.char_indices()
            .map(|(i, c)| {
                if c == 'X' { (i + 1) * (10) } 
                else { (i + 1) * (c.to_digit(10).unwrap() as usize) }
            }).sum::<usize>() % 11 == 0
    } 
    else { false }
}
