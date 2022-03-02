# primenumbe-rs
Generate the nth prime number. <br>
primenumbe-rs algorithm is inspired by the the optimized version of the Sieve of Eratosthenes. Note: limit of n index stands at [1, 1_000_000).

### Example

```rust
use primenumbe_rs::Primenumber;

fn main(){
    let n: u64 = 100;
    let result = Primenumber::nthprime(n);
    println!("The {n}th prime is {result}");
}
```