use itertools::Itertools;



fn main(){
    println!("{:#?}", find_multiples(1, 2))
}

fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    (n..=limit).filter(|num| num % n == 0).collect_vec()
}