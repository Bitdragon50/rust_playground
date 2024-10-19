fn nb_dig(n: i32, d: i32) -> i32 {
    let vec_acc = (0..=n).map(|num| num * num).filter(|square| square.to_string().contains(&d.to_string())).collect::<Vec<i32>>();

    vec_acc.into_iter().map(|number| format!("{}", number).chars().filter(|c| c == &std::char::from_digit(d as u32, 10).unwrap() ).collect::<String>().len() as i32 ).sum()
}

fn main(){
    println!("{:#?}", nb_dig(25,1))
}