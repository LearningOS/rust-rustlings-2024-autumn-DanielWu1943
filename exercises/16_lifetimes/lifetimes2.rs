// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    //result 变量的赋值过程中引用了 string2，然后 string2 在赋值之后就超出了它的作用域了
    //在 longest 函数返回的引用 result 是指向 string2 的，而 string2 已经被销毁了，因此会导致悬挂引用的问题
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
