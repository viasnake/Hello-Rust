const PI_F32: f32 = 3.141592653589793;

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
    println!("明示的な型変換: x + y = {}", x as f64 + y);
    println!("CONST PI_F32 = {}", PI_F32);

    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", nums);

    let result = cal(30,64);
    println!("30 + 64 = {} , 30 * 64 = {}", result.0, result.1);
}

fn cal(x: i32, y: i32) -> (i32, i32) {
    return (x + y, x * y);
}