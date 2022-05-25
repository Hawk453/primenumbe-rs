//! Generate the nth prime number. Note: limit of n index stands at [1, 1_000_000).
//! # Algorithm
//! primenumbe-rs crate's algorithm is inspired by the the optimized version of the Sieve of Eratosthenes.
//!
/// ```no_run
/// use primenumbe_rs::Primenumber;
/// ```
/// # Example
///
/// ```no_run
///
/// let n: u64 = 100;
/// let result = Primenumber::nthprime(n);
/// assert_eq!(result, 541u64);
/// ```
///

pub struct Primenumber {
    num: u64,
}

impl Primenumber {
    pub fn nthprime(n: u64) -> u64 {
        get_nth_prime(&Primenumber { num: n })
    }
}

pub fn get_nth_prime(nth: &Primenumber) -> u64 {
    let mut total_prime: u64 = 0;
    let mut size_factor: u64 = 2;

    let mut s: u64 = nth.num * size_factor;
    let mut primes: Vec<u64> = Vec::new();

    let n: u64 = nth.num;

    while total_prime < n {
        primes = get_primes(s).to_vec();

        total_prime = primes[2..].iter().sum();
        size_factor += 1;
        s = n * size_factor;
    }

    count_prime(primes, n).unwrap()
}

fn get_primes(s: u64) -> Vec<u64> {
    let mut v: Vec<u64> = vec![1; s as usize];

    for index in 2..s {
        if v[index as usize] == 1 {
            for j in index..s {
                if index * j < s {
                    v[(index * j) as usize] = 0;
                } else {
                    break;
                }
            }
        }
    }
    v
}

fn count_prime(primes: Vec<u64>, n: u64) -> Option<u64> {
    let mut counter: u64 = 0;
    for i in 2..primes.len() {
        counter += primes[i];
        if counter == n {
            return Some(i as u64);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::Primenumber;

    #[test]
    fn it_works() {
        let n: u64 = 100;
        let result = Primenumber::nthprime(n);
        assert_eq!(result, 541u64);
    }
}
