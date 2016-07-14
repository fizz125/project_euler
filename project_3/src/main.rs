extern crate libprime;

fn main() {

//	let mut primes: Vec<u64> = vec![2, 3, 5, 7];
	let mut max_factor: u64 = 0;

	//let val = 13195;
	let val = 600851475143;
	let max_pos_factor = (val as f64).sqrt() as u64 + 1;

	println!("val = {}", val);
	println!("max pos prime factor = {}", max_pos_factor);

	let primes: Vec<u64> = libprime::seive(max_pos_factor as usize);

/*
	while max_pos_factor > *primes.last().unwrap() {

		let new_prime = find_next_prime(&primes);
		primes.push(new_prime);
		println!("New prime = {}", new_prime);

	}
*/

	let mut i = 1;
	for x in primes.iter() {
		if val % x == 0 {
			println!("Factor #{} = {}", i, x);
			max_factor = *x;
			i += 1;
		}
	}

	println!("Max Factor = {}", max_factor);

}

