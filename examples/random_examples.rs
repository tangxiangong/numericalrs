use numrs::Array;
use rand::{thread_rng, Rng};

fn main() {
    let v = Array::rand(10);
    println!("{}", v);
    let mut rng = thread_rng();
    let s = rng.gen_range(-4.0..4.0);
    println!("{s}");
}
