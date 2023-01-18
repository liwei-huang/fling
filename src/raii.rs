fn create_box() {
    let _box1 = Box::new(1i32);
    // `_box1` 在这里销毁，内存释放。
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("[自定义的解构器]: 正在丢弃 `ToDrop`。");
    }
}

fn main() {
    // `RAII(Resource Acquisition Is Initialization)`。
    // 任何对象离开作用域，将调用它的解构器，释放内存。
    // 解构器是通过 `Drop` 特征实现的。

    let _box2 = Box::new(2i32);

    {
        let _box3 = Box::new(3i32);
        // `_box3` 在这里销毁，内存释放。
    }

    // 创建大量 `Box`，不用手动释放内存。
    for _ in 0u32..1_000u32 {
        create_box();
    }

    let _drop = ToDrop;

    // `_box2` 在这里销毁，内存释放。
}
