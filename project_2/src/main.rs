fn main() {

	let mut fib_vec: Vec<u32> = vec![1, 2];
	let max: u32 = 4*(10u32.pow(6));

	//while *fib_vec.last().unwrap() < max {
	loop {
		let vec_len = fib_vec.len();
		let next_val = fib_vec[vec_len-1] + fib_vec[vec_len-2];

		if next_val > max {
			break;
		}

		fib_vec.push(next_val);
		//println!("pushed value {}", *fib_vec.last().unwrap());
	}

	let sum = fib_vec.iter().filter(|&x| x % 2 == 0).fold(0, |sum, &x| sum + x);

	println!("even fibbonachi sum = {}", sum);
}
