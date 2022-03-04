use primenumbe_rs::Primenumber;

fn main() {
    let n = 100;
    let nn = Primenumber::nthprime(n);
    println!("The {n}th prime number is {}", nn);
}
