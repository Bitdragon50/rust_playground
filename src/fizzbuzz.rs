
 
#[allow(unused_variables)]
fn run(N: i32, M: i32) -> String {
	//
	// Write your code below; return type and arguments should be according to the problem\'s requirements
	//

	(N as usize..= M as usize).map(|num| match (num % 3 == 0, num% 5 == 0) {
                                                    (true, true) => ",FizzBuzz".to_owned(),
                                                    (false, true) => ",Buzz".to_owned(),
                                                    (true, false) => ",Fizz".to_owned(),
                                                    (false, false) => format!(",{}",num)
    } ).collect::<String>().trim_start_matches(',').to_owned()
}

fn main(){
    run(1, 10);
}