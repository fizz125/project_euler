
/*
 * This code works, but is very slow compared to the seive
 * There may be a way to optimize it, but for now it's
 * just here for historical purposes, once the seive method
 * becomes untenable
 */
/*
//To do - ony check up to the sqrt of test
fn find_next_prime(primes: &Vec<u64>) -> u64 {

    let mut test: u64 = (*primes.last().unwrap()) + 2;
    let mut result: u64;

    loop {
        result = 0;
        for x in primes.iter() {
            if test % x == 0 {
                result = *x;
            }
        }

        if result == 0 {
            break;
        }
        test += 1;
    }

    test
}
*/

/* 
 * Seive of Eratosthenes
 * Maximum usability unknown. Probalby not much more than
 * a couple hundred million (1e6 - 1e7)
 */
pub fn seive(max: usize) -> Vec<u64> {

    let mut is_prime: Vec<bool> = vec![true; max];

    is_prime[0] = false;
    is_prime[1] = false;

    let mut cur_prime = 2;

    'outer: loop {
        let mut found_unprime = false;
//      println!("cur_prime = {}", cur_prime);

        let mut i = 2*cur_prime;
        while i < max {
            is_prime[i] = false;
            found_unprime = true;
            i = i + cur_prime;
        }

        if !found_unprime {
            break;
        }

        for i in cur_prime+1..max {
            if is_prime[i] {
                cur_prime = i;
                continue 'outer;
            }
        }

        break;
    }

    let mut primes: Vec<u64> = Vec::new();

    for (i, b) in is_prime.iter().enumerate() {
        if *b {
            //println!("{}", i);
            primes.push(i as u64);
        }
    }

    primes
}

pub fn hello_world() {
	println!("Hello World!");
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
		hello_world();
    }

	#[test]
	fn test_seive() {
		let primes: Vec<u64> = seive(1000);

		assert_eq!(primes.len(), 168);
		assert_eq!(primes[167], 997);

	}
}
