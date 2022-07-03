// 使用两种方法来让下面代码工作
// Solution 1
// fn main() {
//     assert!(0.1_f32+0.2_f32==0.3_f32);
// }
// Solution 2
fn main() {
    assert!((0.1_f64+0.2_f64-0.3_f64).abs() <= 0.00001);
}