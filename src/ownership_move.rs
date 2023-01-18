// (1) 资源只能拥有一个所有者，避免一个资源被多次释放。
// (2) 不是所有变量拥有资源，比如引用。
// (3) 赋值（`let x = y`）或者函数通过值传递参数（`foo(x)`），发生了资源的所有权转让，这叫做移动 `move`。
// (4) 移动资源后，不能再使用之前的所有者，避免创建悬荡指针。

#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}

fn main() {
    // 栈分配的整数。
    let stach = 5u32;
    // 复制 `stach` 到 `stact`，没有资源移动。
    let stact = stach;
    // 两个值都能独立使用。
    println!("stach: {}, stact: {}", stach, stact);

    // 堆分配的整数，`hean` 是指向它的指针。
    let hean = Box::new(5i32);
    // 移动 `hean` 到 `heam`。
    // 复制 `hean` 的指针地址（不是数据）到 `heam`。
    // 两个指针都指向同一个数据，但是现在 `heam` 是所有者，而 `hean` 不是。
    let heam = hean;

    // 不能，因为 `hean` 不是所有者了，访问不到数据。
    // println!("hean: {}", hean);

    // 函数拿走 `heam` 的所有权。
    destroy_box(heam);

    // 不能，因为 `heam` 不是所有者了，访问不到数据。
    // println!("heam: {}", heam);

    let immutable_box = Box::new(7u32);

    // 不能，因为 `immutable_box` 不可变。
    // *immutable_box = 8u32;

    // 移动所有权，并且变为可变。
    let mut mutable_box = immutable_box;
    *mutable_box = 8u32;
    println!("mutable_box: {}", mutable_box);

    let alice = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` 移动到变量 `name`，而 `age` 是引用。
    let Person { name, ref age } = alice;
    println!("name: {}, age: {}, alice.age: {}", name, age, alice.age);
    // 不能，因为 `alice` 处于部分移动。
    // println!("alice: {:?}", alice);
}

fn destroy_box(tar: Box<i32>) {
    println!("销毁一个 `Box`，`Box` 内容是：{}", tar);
    // 销毁 `tar`，释放内存。
}
