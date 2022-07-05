// 使用至少两种方法来修复错误
fn main() {
    // Solution 1
    // let s =  "hello, world".into();
    // Solution 2
    // let s =  "hello, world".to_string();
    // Solution 3
    let s =  String::from("hello, world");
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}
