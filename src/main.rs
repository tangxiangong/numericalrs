use numericalrs::Array;
use numericalrs::array;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let v = Vec::from([0; 4]);
    let arr1 = Array::from(v);
    let arr2 = array![1, 2, 3, 4];
    let arr3 = Array::<i32>::one(5);
    println!("{}", arr1.size());
    println!("{}", arr2.size());
    println!("{}", arr3.size());

    let r1 = (&arr1 + &arr2)?;
    println!("{}", r1);
    let r2 = (&arr1 + &arr3)?;
    println!("{}", r2);
    Ok(())
}