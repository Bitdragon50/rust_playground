
fn run(student_list: Vec<i32>) -> i32 {
	//
	// Write your code below; return type and arguments should be according to the problem\'s requirements
	//
	

	let single_student_number: i32 = student_list.iter().filter(|student| student_list.iter().filter(|n| n == student).count() == 1).sum();

	single_student_number
}

fn main() {
    let student_list = vec![1,2,2,1,6];
    println!("{:#?}",run(student_list));
}