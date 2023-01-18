fn main() {
    // `Option<T>` 是一个枚举，拥有两个变体：
    // (1) `None`，表示失败、没有值；
    // (2) `Some(value)`，表示成功，是一个元组结构体，`value` 的类型是 `T`。

    try_division(16, 4);
    try_division(8, 0);

    // 绑定 `None` 给变量，必须要有类型注解。
    let a_none: Option<i32> = None;
    let b_none = None::<i32>;
    let _ = (a_none, b_none);

    // 解开 `Some` 的包裹，但不能解开 `None` 的包裹。
    let c_some = Some(23.35f32);
    println!("{}", c_some.unwrap());
}

fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        // 结果包裹在 `Some` 中。
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("{} / {} 失败！", dividend, divisor),
        // `quotient` 是 `Some` 暴露出来的，`quotient` 就是 `dividend / divisor`。
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        }
    }
}
