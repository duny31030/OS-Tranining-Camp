// 让代码工作：修改 `assert!` 中的 `4` 
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    // 单元类型 不占用任何内存
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}