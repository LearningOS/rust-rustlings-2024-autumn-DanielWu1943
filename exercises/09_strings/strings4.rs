// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");//"blue"是字符串切片&str，"blue".to_string()后是String

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));//format!返回的是String

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());//"  hello there ".trim()相当于"hello there"，是&str

    string("Happy Monday!".replace("Mon", "Tues"));// replace 返回的是一个新的 String，应该使用 string()

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());// to_lowercase() 返回 String 类型
}
