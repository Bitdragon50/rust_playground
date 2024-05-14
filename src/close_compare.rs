fn main() {
    println!("{:#?}", close_compare(4.0, 5.0, 0.0))
}

fn close_compare(a: f64, b: f64, margin: f64) -> i8 {
    match (a - b).abs() <= margin {
        true => return 0,
        false => {
            match a > b {
                true => return 1,
                false => return -1
            }
        }
    }
} 