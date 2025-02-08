#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a == i64::MIN && b == -1 {
        Err(DivisionError::IntegerOverflow)
    } else if a % b != 0 {
        Err(DivisionError::NotDivisible)
    } else {
        Ok(a / b)
    }
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `Ok([1, 11, 1426, 3])`
fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    /*
    numbers.into_iter(): 将 numbers 数组转换为一个迭代器。这将允许你逐一访问数组中的元素
    .map(|n| divide(n, 27)): 对于数组中的每个元素 n，调用 divide(n, 27)
    division_results 是一个迭代器，其中每个元素都是一个 Result<i64, DivisionError> 类型的值
     */
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    //.filter(|x| x.is_ok()): 过滤 division_results 迭代器中的每个元素，保留那些 Result 类型是 Ok 的项
    //.collect(): 将过滤后的结果收集成一个集合, 它会将 Ok(i64) 的值提取出来并组成一个 Vec<i64>
    division_results.filter(|x| x.is_ok()).collect()
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() -> Vec<Result<i64, DivisionError>>{
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // collect::<Vec<Result<i64, DivisionError>>>()：将所有通过 filter 后的 Result（即 Ok(i64) 或 Err(DivisionError)）收集到一个 Vec<Result<i64, DivisionError>> 中
    division_results.filter(|x| x.is_ok()).collect::<Vec<Result<i64, DivisionError>>>()

}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
