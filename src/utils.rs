use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    // `rng.sample_iter(&Standard)` で一様分布の乱数を生成するイテレータを返す
    // `.take(n)` で最初の `n` 個を取り出すイテレータを返す
    // `collect()` でイテレータから値を収集して，コレクションに格納する
    rng.sample_iter(&Standard).take(n).collect()
}

//pub fn new_u32_vec(n: usize) -> Vec<u32> {
//    // RNGを `0` が16個の配列で初期化
//    let mut rng = Pcg64Mcg::from_seed([0; 16]);
//    // `n` 個の乱数を格納できるベクタ `v` を用意
//    let mut v = Vec::with_capacity(n);
//    for _ in 0..n {
//        v.push(rng.sample(&Standard)); // 分布は `Standard` （一様分布）
//    }
//    v // ベクタを返す
//}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    // `windows(2)` で「イテレータから1要素2要素つづ」取り出す
    // `all()` でイテレータから値を取り出してクロージャに渡す
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}