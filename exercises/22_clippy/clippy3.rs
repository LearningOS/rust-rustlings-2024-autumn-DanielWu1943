// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        //println!("{:?}", my_option.unwrap());//1. unwrap 会在 Option 为 None 时 panic
        //if let Some(x) = my_option {};
        println!("Option is None, can't unwrap it");
    }
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");
    //resize 是一个改变现有 Vec 长度的方法，应该在已经创建的 Vec 上使用
    //let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    //value_a = value_b;
    //value_b = value_a;
    println!("value a: {value_a}; value b: {value_b}");
}
