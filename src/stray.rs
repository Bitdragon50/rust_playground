use std::collections::HashMap;

fn main(){
    println!("{:#?}", stray(&vec![1,1,3,1]))
}

fn stray(arr: &[u32]) -> u32 {
    //let num_set = arr.into_iter().collect::<HashSet<&u32>>();
    let mut map: HashMap<&u32, u8> = HashMap::new();
    for num in arr {        
            *map.entry(num).or_insert(0) += 1;
            println!("{:#?}",map);
    }
    //let mut stray: u32 = 0;
    let mut stray_vec = map.into_iter().collect::<Vec<(&u32,u8)>>();

    stray_vec.sort_by(|(_a1,a2,),(_b1,b2)|
                    a2.cmp(b2)
                    );

            //get the key whose value is 1
    *stray_vec[0].0
}
