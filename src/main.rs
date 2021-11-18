fn main() {
    // コメント
    println!("Hello Rust!");

    let mut x = 10;
    let y = 5.55;
    println!("x:i32 = {}, y:i64 = {}",x ,y);

    x = 20;
    println!("x is mutable: x = {}", x);

    let b = true;
    let t = (13, false);
    let s = "hello world!";
    println!("{} {} {} {}", b, t.0, t.1, s);
}