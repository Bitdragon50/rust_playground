use std::time::Instant;

use itertools::Itertools;



fn main(){
    let mut arr = [3,2,6,1,4,10,2,5,6,9,2,51,8,4];
    let mut start = Instant::now();
    arr.sort();    
    let mut stop_time: Option<std::time::Duration>;

    start = Instant::now();
    println!("{:?}", linear_search(&arr, &51));
    stop_time = Instant::now().checked_duration_since(start);
    println!("Exec Time for linear search: {:#?}", stop_time.unwrap());

    start = Instant::now();
    println!("{:?}", &arr.binary_search(&51));
    stop_time = Instant::now().checked_duration_since(start);
    println!("Exec Time for binary search: {:#?}", stop_time.unwrap());

    start = Instant::now();
    println!("{:?}", binary_search(&arr, &51));
    stop_time = Instant::now().checked_duration_since(start);
    println!("Exec Time for binary search: {:#?}", stop_time.unwrap());
}

fn linear_search(arr: &[u8], num: &u8) -> Option<usize> {
    for (index, number) in arr.iter().enumerate() {
        if number == num {
            return Some(index)
        }
    }
    None
}


fn binary_search(arr: &[u8], num: &u8) -> Option<usize> {
    let mut inner_arr = arr.to_vec();
    inner_arr.sort();
    let found = None;
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = (low + high)/2;

        match inner_arr[mid].cmp(num) {
            std::cmp::Ordering::Equal => { return Some(mid) }
            std::cmp::Ordering::Greater => { high = mid },
            std::cmp::Ordering::Less => { low = mid + 1 }
        }
    }
    found
}