use crate::Array;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::ops::RangeBounds;

/// 辅助函数，用于创建均匀分布的随机向量和随机矩阵
/// `R` 既可以是 `Range<T>` 也可以是 `RangeInclusive<T>`
pub(crate) fn uniform_rand_generator<T, R>(range: R, n: usize) -> Vec<T>
where
    // Range<T> & RangeInclusive<T> : RangeBounds<T>
    R: RangeBounds<T>,
    T: SampleUniform + Send,
    // Uniform<T> : From<Range<T>> & From<RangeInclusive<T>>
    Uniform<T>: From<R>,
    <T as SampleUniform>::Sampler: Sync,
{
    // TODO 错误处理，避免发生 panic
    let uniform = Uniform::from(range); // Panic when start_bound <= end_bound
    (0..n)
        .into_par_iter()
        .map_init(thread_rng, |rng, _| uniform.sample(rng))
        .collect()
}

impl Array {
    /// [0, 1] 之间的均匀分布随机向量
    pub fn rand(n: usize) -> Self {
        let data = uniform_rand_generator(0.0..=1.0, n);
        Array::from(data)
    }
}

impl<T: num::Num> Array<T> {
    pub fn uniform_rand<R>(range: R, n: usize) -> Self
    where
        R: RangeBounds<T>, // 对调用 from 无影响
        T: SampleUniform + Send,
        Uniform<T>: From<R>,
        <T as SampleUniform>::Sampler: Sync,
    {
        let data = uniform_rand_generator(range, n);
        Self::from(data)
    }
}
