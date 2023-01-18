fn main() {
    // (1) 闭包，就像匿名函数。
    // 有类型注解。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    // 无类型注解。
    let closure_inferred = |i| i + 1;
    // 无参数（无输入变量）。
    let abs_arg = || '😅';
    println!(
        "{}, {}, {}",
        closure_annotated(10),
        closure_inferred(10i8),
        abs_arg()
    );

    // (2) 高阶函数。
    let upper = 1000;
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();

    println!("函数式：{}", sum_of_squared_odd_numbers);

    // (3) 离散函数，永远不返回值。
    fn foo() -> ! {
        panic!("从不返回值。");
    }

    foo();
    // println!("不会运行到这里。");
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
