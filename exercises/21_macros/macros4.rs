// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]
//定义多个宏分支时，缺少分隔符
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
