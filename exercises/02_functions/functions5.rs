// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num //rust 会返回最后一个表达式，不要加分号即可
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
