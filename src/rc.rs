use std::rc::Rc;

fn main() {
    // `Rc` 记录内部的值的所有者的数量，即内部的值的引用计数。
    // 复制一个 `Rc` 时，引用计数加一，丢弃一个 `Rc` 时，引用计数减一。
    // 当 `Rc` 的引用计数为零时，`Rc` 和 `Rc` 内部的值都将被丢弃。
    // 复制 `Rc`，不是深拷贝，而是创建另一个指向内部的值的指针，并且引用计数加一。

    let rc_example = "引用计数的例子".to_string();

    {
        println!("--- 创建 rc_a ---");
        let rc_a: Rc<String> = Rc::new(rc_example);
        println!("rc_a 的引用计数：{}", Rc::strong_count(&rc_a));

        {
            println!("--- 创建 rc_b ---");
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("rc_b 的引用计数：{}", Rc::strong_count(&rc_b));
            println!("rc_a 的引用计数：{}", Rc::strong_count(&rc_a));

            // 内部的值相当，则两个 `Rc` 也相等。
            println!("rc_a 等于 rc_b：{}", rc_a.eq(&rc_b));
            // `Rc` 直接使用内部的值、内部的值的方法。
            println!("rc_a 内部的值：{}", rc_a);
            println!("rc_b 内部的值的长度：{}", rc_b.len());

            println!("--- 丢弃 rc_b ---");
        }

        println!("rc_a 的引用计数：{}", Rc::strong_count(&rc_a));
        println!("--- 丢弃 rc_a ---");
    }

    // `rc_example` 已经移动到 `rc_a` 中。
    // 现在 `rc_a` 被丢弃，`rc_example` 也被丢弃，不能使用 `rc_example` 了。
}
