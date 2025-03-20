use std::time::Instant;



fn main(){
    let arr = &[3,2,6,1,4,10,2,5,6,9,2,51,8,4];
    let mut start = Instant::now();
    
    let mut stop_time: Option<std::time::Duration>;
    start = Instant::now();
    println!("{:?}", insertion_sort(arr));
    stop_time = Instant::now().checked_duration_since(start);
    println!("Exec Time for insertion sort: {:#?}", stop_time);
    start = Instant::now();
    println!("{:?}", bubble_sort(arr));
    stop_time = Instant::now().checked_duration_since(start);
    println!("Exec Time for bubble sort: {:#?}", stop_time);
    start = Instant::now();
    println!("{:?}", merge_sort(arr));
    stop_time = Instant::now().checked_duration_since(start);
    println!("Exec Time for merge sort: {:#?}", stop_time);

}

fn insertion_sort(arr: &[u8]) -> Vec<u8> { 
    let mut temp_vec = arr.to_vec(); 
    for index in 1..temp_vec.len() { 
        let current = temp_vec[index]; 
        let mut look_back = index as isize - 1; 
        // Move elements of temp_vec[0..index-1], that are greater than current, 
        // to one position ahead of their current position 
        while look_back >= 0 && current < temp_vec[look_back as usize ] { 
            temp_vec[look_back as usize + 1] = temp_vec[look_back as usize ]; 
            look_back -= 1; 
        }
        println!("Look back: {:#?}", look_back);
        temp_vec[(look_back + 1) as usize  ] = current; 
    }
    temp_vec 
}


fn merge_sort(arr: &[u8]) -> Vec<u8> {
    if arr.len() <= 1 {
        return arr.to_vec()
    }
    println!("Running with {:?}",arr);
    let vec_arr = arr.to_vec();
    let half_length = vec_arr.len()/2;
    let left_split = merge_sort(&vec_arr[..half_length]);
    let right_split = merge_sort(&vec_arr[half_length..]);
//     merge_the_sorted(left_split,right_split)
// }

// fn merge_the_sorted(left_split: Vec<u8>, right_split: Vec<u8>) -> Vec<u8> {

    let mut left_index = 0;
    let mut right_index = 0;
    let mut acc_vec = Vec::new();
    while left_index < left_split.len() && right_index < right_split.len() {
        if left_split[left_index] < right_split[right_index] {
            println!("Taking from the left {}", left_split[left_index]);
            acc_vec.push(left_split[left_index]);
            left_index += 1
        } 
        else {
            println!("Taking from the right {}", right_split[right_index]);
            acc_vec.push(right_split[right_index]);
            right_index += 1
        }
    }
    acc_vec.extend_from_slice(&left_split[left_index..]);
    acc_vec.extend_from_slice(&right_split[right_index..]);
    acc_vec
}

fn bubble_sort(arr: &[u8]) -> Vec<u8> {
    let mut acc_vec = arr.to_vec();
    let mut counter = 0;

    while counter < acc_vec.len() {
        for index in 0..arr.len() - 1 {
            let current = acc_vec[index];
            let next = acc_vec[index+1];
            if current > next {
                let temp = current;
                acc_vec[index] = next;
                acc_vec[index+1] = temp;
            }
        }
        counter += 1;
    }
    acc_vec
}