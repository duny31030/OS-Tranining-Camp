// 使用两种方法让代码工作起来
// Solution 1
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2;
//         x
//     };
 
//     assert_eq!(v, 3);
// }
// Solution 2
fn main() {
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, ());
}