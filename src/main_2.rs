use std::collections::HashMap;

fn main(){
    println!("{:#?}", josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1))
}

fn josephus<T:Clone+Copy>(xs:Vec<T>,k:usize)-> Vec<T> {
    //initialise a map
    let mut map = HashMap::new();
    let mut acc = vec![];
    let mut xsx = xs.clone();

        for (i, x) in xsx.iter().enumerate().step_by(k){
            if !map.contains_key(&i) {
                map.insert(i, x);
                acc.push(*x);
                //xsx.remove(i);
            }
        }
        acc
}