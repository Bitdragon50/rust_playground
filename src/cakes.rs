use std::collections::HashMap;
fn main(){
    let recipe = vec![("flour", 500), ("sugar", 200), ("eggs", 1)].into_iter().collect::<HashMap<&str, u32>>();
    let available = vec![("flour", 1200), ("sugar", 1200), ("eggs", 5), ("milk", 200)].into_iter().collect::<HashMap<&str, u32>>();
    println!("{:#?}", cakes(&recipe, &available))
}

fn cakes(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {
    //find the ingredients in the 
    match recipe.keys().all(|key| available.keys().collect::<Vec<&&str>>().contains(&key)){
        false => 0,
        true => {
            let mut cakes: Vec<u32> = vec![];
            for ingredient in recipe.keys() {
                let proportion_of_ingredient = 
                available.get(ingredient).unwrap()/recipe.get(ingredient).unwrap();
                cakes.push(proportion_of_ingredient)
            }
            cakes.into_iter().min().unwrap()
        }
    }
}