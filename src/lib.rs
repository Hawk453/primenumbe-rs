//! Generate the nth prime number.
//! primenumbe-rs crate's algorithm is inspired by the the optimized version of the Sieve of Eratosthenes.
//! 
/// ```no_run
/// use primenumbe_rs::Primenumber;
/// ```
/// # Example 
/// 
/// ```no_run
/// 
/// let n: u128 = 100;
/// let result = Primenumber::nthprime(n);
/// assert_eq!(result, 541u128);
/// ```
/// 




pub struct Primenumber {
    num : u128,
}

impl Primenumber {
    
    pub fn nthprime(n : u128) -> u128 {

        get_nth_prime(&Primenumber { num: n })
    }
}



pub fn get_nth_prime(nth: &Primenumber) -> u128{

    let mut total_prime: u128 = 0;
    let mut size_factor: u128 = 2;

    let mut s : u128 = nth.num * size_factor;
    let mut primes : Vec<u128> = Vec::new();
    
    
    let  n: u128 = nth.num ;
    
    while total_prime < n {
        primes = get_primes(s).iter().copied().collect();
        
        total_prime = primes[2..].iter().sum();
        size_factor +=1;
        s = n * size_factor;
    }

    let nth_prime = count_prime(primes, n).unwrap();


    return nth_prime;
}

fn get_primes(s : u128) -> Vec<u128> {
    let mut v: Vec<u128> = vec![1; s as usize];

    for index in 2..s {
        if v[index as usize] == 1 {
            for j in index..s {
                if index * j < s {
                    v[(index*j) as usize ] = 0;
                }
                else {
                    break;
                }
            }
        }
    }
    return v;
}


fn count_prime(primes : Vec<u128>, n : u128) ->Option<u128> {
    let mut counter: u128 = 0;
    for i in 2..primes.iter().count() {
        counter = counter + primes.iter().nth(i).unwrap();
        if counter == n {
            return Some(i as u128);
        }
    }
    return None;
}



#[cfg(test)]
mod tests {
    use crate::Primenumber;

    #[test]
    fn it_works() {
        let n: u128 = 100;
        let result = Primenumber::nthprime(n);
        assert_eq!(result, 541u128);
    }
}
