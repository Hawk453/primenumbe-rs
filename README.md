# primenumbe-rs

[![Crates.io](https://img.shields.io/crates/v/primenumbe-rs?style=flat-square)](https://crates.io/crates/primenumbe-rs)
![Crates.io](https://img.shields.io/crates/l/primenumbe-rs?style=flat-square)

This crate provides a **beautifully simplistic API** for generating the nth prime number. <br>
primenumbe-rs algorithm is inspired by the the optimized version of the Sieve of Eratosthenes. 

## Notice

Note: limit of n index stands at [1, 1_000_000). <br>
**Disclaimer** Not meant for Cryptographic & Security use.   

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
primenumbe-rs = "0.1.2"
```

### How To Use

```rust
use primenumbe_rs::Primenumber;

fn main(){
    let n: u64 = 100;
    let result = Primenumber::nthprime(n);
    println!("The {n}th prime is {result}");
}
```

# License

Licensed under either of

* Apache License, Version 2.0

* MIT license

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.