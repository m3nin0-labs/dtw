
use dtw::dtw;

fn main() {
    let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let b = vec![vec![2.0, 3.0, 4.0], vec![5.0, 6.0, 7.0]];
    
    let distance = dtw(&a, &b, 1.0);
    
    println!("DTW: {}", distance);
}
