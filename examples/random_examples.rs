use numrs::Array;

fn main() {
    let v1 = Array::rand(10);
    println!("{}", v1);
    let v2 = Array::uniform_rand(0.0..1.0, 10);
    let v3 = Array::uniform_rand(0..=10, 10);
    println!("{v2}");
    println!("{}", v2.l1_norm().unwrap());
    println!("{v3}");
}
