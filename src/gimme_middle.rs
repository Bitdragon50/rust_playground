use itertools::sorted;
fn main(){
    println!("{:#?}",gimme([2,9,5]))
}

fn gimme(input_array: [i32;3]) -> usize {
    input_array.iter().enumerate()
    .filter(|(_i,n)| n == &&sorted(input_array).collect::<Vec<i32>>()[1])
    .map(|(i,_n)| i+ 1 ).collect::<Vec<usize>>()[0] 
  }
  