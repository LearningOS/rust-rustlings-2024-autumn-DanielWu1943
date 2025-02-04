#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership，不修改data，所以只需要借用即可
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);//data被移动到函数中，函数调用后 data 就不再可用。所以只需要借用

    string_uppercase(data);//因为修改data所以不借用，直接移动到函数中。
}
