# primenumbe-rs
Generate the nth prime number. <br>
primenube-rs algorithm is inspired by the the optimized version of the Sieve of Eratosthenes

### Example

```rust
use primenumbe_rs::Primenumber;

fn main(){
    let n: u128 = 100;
    let result = Primenumber::nthprime(n);
    println!("The {n}th prime is {result}");
}
```