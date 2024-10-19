fn greet(name: &str) -> String {
    let mut greeting = String::from("Hello ");
    greeting.push_str(&name.to_owned()[..1].to_uppercase());
    greeting.push_str(&name.to_owned()[1..].to_lowercase());
    greeting.push('!');
    greeting
}

fn main(){
    println!("{:#?}", greet("alan"))
}