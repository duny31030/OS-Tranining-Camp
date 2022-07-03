fn main() {
    // 使用尽可能多的方法来通过编译
    // let x = String::from("hello, world");
    // Solution 1
    // let y = x.clone();
    // Solution 2
    // let y = &x;
    // Solution 3
    // let x = "hello, world";
    // let y = x;
    // Solution 4
    let x = 1;
    let y = x;
    println!("{},{}",x,y);
}
