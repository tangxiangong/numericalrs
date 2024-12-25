use numericalrs::Array;
use numericalrs::{
    array,
    Norm::{LInf, L1, L2},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v = vec![[1, 2, 4], [2, 3, 4]];
    let arr1 = Array::new(); // 创建空向量
    let arr2 = Array::from(vec![0.0; 5]); // 从动态数组创建
    let arr3 = Array::from([0., 1., 2., 3., 4.]); // 从静态数组创建
    let arr4 = array![1.0; 5]; // 使用宏创建
    let arr5 = Array::ones(5); // 全1向量
    let arr6 = Array::zeros(4); // 全0向量
                                // 不同维度相加，抛出错误
    match &arr1 + &arr2 {
        Ok(_) => println!("Hello World"),
        Err(e) => println!("{e}"),
    }
    let (_a, _b, _c) = (arr3.norm(L1), arr3.norm(L2), arr3.norm(LInf)); // L1, L2, L inf 范数
                                                                        // 迭代器
    for &value in arr3.iter() {
        print!("value = {value}, ");
    }
    let _result1 = (&arr4 + 1.0)?; // 加法广播
    let _result2 = (3.1 * &arr4)?; // 数乘
    let _result3 = (&arr5 * &arr6)?; // 内积

    Ok(())
}
