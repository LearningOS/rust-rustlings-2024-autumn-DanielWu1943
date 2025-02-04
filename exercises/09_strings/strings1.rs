// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string()//"blue"是&str不是String，要转换成String
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
