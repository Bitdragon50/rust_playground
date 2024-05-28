
fn main(){
    println!("{:#?}",sum_dig_pow(1, 100));
}

fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    (a..=b).filter(|r| {

        &r.to_string().char_indices().map(|(i,c )| {
            (c.to_digit(10).expect("Expected a number") as u64).pow((i+1) as u32)
        }).sum::<u64>()  == r 
    
    }).collect()
}
