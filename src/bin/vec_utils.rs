use competitive_rust_libraries::vec_utils;

fn main() {
    let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    eprintln!("{}", vec_utils::string_2dim_vec(&v));
}
