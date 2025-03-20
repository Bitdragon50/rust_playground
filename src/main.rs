use itertools::Itertools;


fn main(){
    let arr1 = [5, 3, 2, 8, 1, 4]; //[7, 1];
    let arr2 = [5, 8, 6, 3, 4];
    let arr3 = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    println!("{:?}", sort_odd_in_array(&arr1));
    println!("{:?}", sort_odd_in_array(&arr2));
    println!("{:?}", sort_odd_in_array(&arr3));
}

fn sort_odd_in_array(arr: &[i32]) -> Vec<i32> {
    let arr_map: Vec<(usize, &i32)> = arr.iter().enumerate()
        .filter(|(_index, num)| **num % 2 == 0 )
        .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
        .rev().collect();
    let mut arr_odd: Vec<i32> = arr.iter().cloned()
        .filter(|&x| x % 2 != 0).sorted().collect();
    
    // println!("Arr odd:{:?}", arr_odd);
    // println!("Arr map:{:?}", arr_map);
    arr_map.iter().for_each(|(index, num)| {
        // println!("Arr odd:{:?}", arr_odd);
        // println!("index: {:?}\nnumber: {}", index, num);
        arr_odd.insert(*index, **num)
    });
    arr_odd
}