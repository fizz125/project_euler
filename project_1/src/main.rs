fn main() {
//    let div_3_or_5 = (1..1001).filter(|&x| (x % 3 == 0 || x % 5 == 0)).collect::<Vec<u32>>();

//	for(i,x) in div_3_or_5.iter().enumerate() {
//		println!("div_3[{}] = {}", i, x);
//	}

	let sum_mult_3_or_5 = (1..1000).filter(|&x| (x % 3 == 0 || x % 5 == 0)).fold(0, |sum, x| sum + x);
	println!("sum = {}", sum_mult_3_or_5);
}
