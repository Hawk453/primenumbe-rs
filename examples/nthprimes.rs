use primenumbe_rs::Primenumber;

fn main() {
    let n: u64 = 100;
    let nn = Primenumber::nthprime(n);
    println!("The {n}th prime number is {}", nn);
}
