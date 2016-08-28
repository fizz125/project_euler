pub fn u32_to_array(int: u32) -> Vec<u32> {

	let mut ary: Vec<u32> = Vec::new();
	let mut remainder = int;

	while remainder > 10 {
		let tens = remainder / 10;
		let ones = remainder - (tens*10);
	
		//println!("Remainder = {}. Tens = {}. Ones = {}", remainder, tens, ones);

		ary.push(ones);
		remainder = tens;
	}

	ary.push(remainder);
	ary
}

pub fn u64_to_array(int: u64) -> Vec<u32> {

	let mut ary: Vec<u32> = Vec::new();
	let mut remainder = int;

	while remainder > 10 {
		let tens = remainder / 10;
		let ones = remainder - (tens*10);
	
		//println!("Remainder = {}. Tens = {}. Ones = {}", remainder, tens, ones);

		ary.push(ones as u32);
		remainder = tens;
	}

	ary.push(remainder as u32);
	ary

}

pub fn array_to_u32(ary: Vec<u32>) -> u32 {

	let mut val: u32 = 0;
	let mut remainder = ary.clone();

	let mut idx = 0;
	while remainder.len() > 0 {
		let x = remainder.remove(0);
		val = val + x * ((10 as u32).pow(idx));
		idx += 1;
	}
	val
}

pub fn array_to_u64(ary: Vec<u32>) -> u64 {

	let mut val: u64 = 0;
	let mut remainder = ary.clone();

	let mut idx = 0;
	while remainder.len() > 0 {
		let x: u64 = remainder.remove(0) as u64;
		val = val + x * ((10 as u64).pow(idx));
		idx += 1;
	}
	val

}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
    }

	#[test]
	fn test_itoary() {
		let ary = u32_to_array(123456);
		assert_eq!(ary[5], 1);
		assert_eq!(ary[4], 2);
		assert_eq!(ary[3], 3);
		assert_eq!(ary[2], 4);
		assert_eq!(ary[1], 5);
		assert_eq!(ary[0], 6);
	}

	#[test]
	fn test_arytoi() {
		let val = array_to_u32(vec![1, 2, 3, 4, 5, 6]);
		//println!("Val = {}", val);
		assert_eq!(val, 654321);
	}

	#[test]
	fn test_itoary64() {
		let ary = u64_to_array(567000000123);
		assert_eq!(ary, [3, 2, 1, 0, 0, 0, 0, 0, 0, 7, 6, 5]);
	}

	#[test]
	fn test_arytoi64() {
		let val = array_to_u64(vec![3, 2, 1, 0, 0, 0, 0, 0, 0, 7, 6, 5]);
		assert_eq!(val, 567000000123);
	}
}
