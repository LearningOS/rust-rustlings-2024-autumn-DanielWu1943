// TODO: Fix the compiler error without taking the macro definition out of this
// module.
//宏 my_macro! 被定义在 macros 模块内部，因此它只能在该模块的范围内访问
//通过 导入宏: #[macro_use]
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
