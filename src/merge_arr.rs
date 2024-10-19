use std::collections::HashSet;

use itertools::{sorted, Itertools};

fn main() {
    println!("{:#?}", merge_arrays(&[1, 2, 3, 4, 5], &[6, 7, 8, 9, 10]))
}

fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged: Vec<i32> = vec![];
    merged.extend(arr1);
    merged.extend(arr2);
    sorted(
        merged
            .into_iter()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>(),
    )
    .collect_vec()
}
