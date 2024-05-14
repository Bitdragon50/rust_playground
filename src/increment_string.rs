use num::BigUint;
//use num_traits::FromPrimitive;

fn main(){
    println!("{}",increment_string("HMZMEbMkoBpG3ov2W2Ug7YcnQ6AqKJyGq9KDik8fK1MgrbxU2yYfq0jEfJJmocf49efHwRnJyGTioegpj4ODbB19629989559991965999496484599698949625501194499259269949936789491209359692"))
}

fn increment_string(s: &str) -> String {
    let mut digits = "".to_string();
    for char in s.chars().rev() {
        if char.is_ascii_digit() {
            digits.push(char)
        } else {
            break
        }
    }
    digits = digits.chars().rev().collect();
    let new_digits: BigUint = digits.parse::<BigUint>().unwrap_or(BigUint::from(0 as u8)) + BigUint::from(1 as u8) ;

    match digits.chars().nth(0) {
        Some(x) => {
            let string_digits = &format!("{}",new_digits);
            if x != '0'
            { s.replace(&digits, "") + string_digits } 
                           else 
                         { s.replace(&digits, "") + &"0".repeat(digits.len() - format!("{}",new_digits).len()) + string_digits }
                        },
        None => s.replace(&digits, "") + &format!("{}",new_digits)
    }
}