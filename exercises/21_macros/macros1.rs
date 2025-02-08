macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!();//在宏调用时必须加上 !，这是 Rust 中调用宏的语法。
}
