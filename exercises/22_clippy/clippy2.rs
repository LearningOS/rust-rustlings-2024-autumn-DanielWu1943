fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    //使用 if let 来解构 Option，如果 option 是 Some，它会提取出内部的值并绑定到 x 上，然后执行相应的代码。这样就避免了不必要的 for 循环
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
