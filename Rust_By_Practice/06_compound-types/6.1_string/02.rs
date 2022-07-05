// 使用至少两种方法来修复错误
fn main() {
    // Solution 1
    // let s: Box<str> =  "hello, world".into();
    // greetings(&s)
    // Solution 2
    let s: Box<&str> =  "hello, world".into();
    greetings(*s)
}

fn greetings(s: &str) {
    println!("{}",s)
}