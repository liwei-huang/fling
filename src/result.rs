mod mathium {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    fn op_(x: f64, y: f64) -> MathResult {
        // 如果 `div` 失败，则函数 `op_` 返回 `DivisionByZero`。
        let ratio = div(x, y)?;
        // 如果 `ln` 失败，则函数 `op_` 返回 `NonPositiveLogarithm`。
        let ln = ln(ratio)?;
        // 如果 `div` 和 `ln` 都成功，返回 `sqrt(ln)`，它的返回值类型是 `MathResult`。
        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!(
                "{}",
                match why {
                    MathError::NonPositiveLogarithm => "非正数的对数",
                    MathError::DivisionByZero => "除数为零",
                    MathError::NegativeSquareRoot => "负数的平方根",
                }
            ),
            Ok(value) => println!("{}", value),
        }
    }
}

fn main() {
    // `Result<T, E>` 是一个枚举，拥有两个变体：
    // (1) `Ok(value)`，表示成功，暴露出 `T` 类型的 `value`。
    // (2) `Err(why`，表示失败，暴露出 `E` 类型的 `why`。
    // (3) `?` 操作符用在返回 `Result` 的表达式后面，实现 `match` 的功能。

    match mathium::div(7.0, 3.5) {
        Err(why) => panic!("{:?}", why),
        Ok(resultor) => println!("{}", resultor),
    }

    match mathium::sqrt(4.0) {
        Err(why) => panic!("{:?}", why),
        Ok(resultor) => println!("{}", resultor),
    }

    match mathium::ln(6.0) {
        Err(why) => panic!("{:?}", why),
        Ok(resultor) => println!("{}", resultor),
    }

    mathium::op(1.0, 10.0);
}
