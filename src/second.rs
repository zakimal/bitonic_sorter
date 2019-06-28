pub fn sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up)
    }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)] // 以下のモジュールは `cargo test` を実行した時のみコンパイルされることを表すアトリビュート
mod tests { // `second` モジュールの子供として `tests` モジュールを定義
use super::sort; // 親モジュール `second` の `sort` 関数を用いることを宣言している

    #[test] // `cargo test` を実行した時に実行されることを表すアトリビュート
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, true);
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, false);
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        sort(&mut x, true);
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        sort(&mut x, false);
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }

//    #[test]
//    fn sort_f64() {
//        let mut x = vec![20.0, -30.0, 11.0, 10.0];
//        sort(&mut x, true)
//    }

//    #[test]
//    fn sort_mixed() {
//        let mut x = vec![10, 30, "1", "2"];
//        sort(&mut x, ture)
//    }

}