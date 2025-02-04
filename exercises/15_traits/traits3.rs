trait Licensed {//定义一个名为 Licensed 的 trait（类似其他语言中的接口），包含一个方法 licensing_info
    // TODO: Add a default implementation for `licensing_info` so that
    // implementors like the two structs below can share that default behavior
    // without repeating the function.
    // The default license information should be the string "Default license".
    //定义一个带有默认实现的 trait 方法
    fn licensing_info(&self) -> String{
        String::from("Default license")// 默认返回固定字符串
    }
}

//定义两个不同的软件结构体，一个用整数一个用字符串表示版本号
struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}
//{}表示空实现，继承默认方法
impl Licensed for SomeSoftware {} // Don't edit this line.
impl Licensed for OtherSoftware {} // Don't edit this line.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);//输入什么都返回"Default license"
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
