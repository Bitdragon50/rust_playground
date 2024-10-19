use std::collections::HashMap;

use itertools::Itertools;

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if k > strarr.len() || strarr.is_empty() {
        return "".into()
    }
    // let acc = vec![];
    // let mut array = strarr.clone();
    // Find the first longest word, 
    let index_of_longest = first_longest(&strarr);
    let mut k_longest_forward = String::new();
    if index_checker(&strarr, (index_of_longest+k)) { 
         k_longest_forward = strarr.clone()[index_of_longest..(index_of_longest+k)].to_vec().join("");
    } else {
         k_longest_forward = strarr.clone()[(index_of_longest-k)..=index_of_longest].to_vec().join("");
    };
    dbg!(&index_of_longest);
    k_longest_forward
    // let k_longest_backward = strarr.clone()[(index_of_longest-k+1)..=index_of_longest].to_vec().join("");
    // match k_longest_backward.len() > k_longest_forward.len() {
    //     true => k_longest_backward,
    //     false => k_longest_forward
    // }
}

fn first_longest(strarr: &Vec<&str>) -> usize {
    let length_longest = strarr.iter().max_by_key(|word| word.len()).unwrap().len();
    dbg!(&length_longest);
    strarr.iter().enumerate().find_map(| (index ,word) | if word.len() == length_longest {
        Some(index)
    } else {
        None
    } ).unwrap()
}

fn index_checker(strarr: &Vec<&str>, size: usize) -> bool {
    strarr.len() > size && size > 0
}


fn main(){
    let result = longest_consec(vec!["zone", "abigail", "theta", "forma", "libe", "zas"], 2);
    dbg!(result);
}