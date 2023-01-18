use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn heap_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // (1) 默认，所有值都分配在栈内存中。
    // (2) 使用 `Box<T>` 让值分配在堆内存中。
    // (3) `Box` 是智能指针。
    // (4) 使用 `*` 操作符去除引用。

    // 栈内存分配。
    let stack_point: Point = origin();
    let stack_rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // 堆内存分配。
    let heap_point: Box<Point> = Box::new(origin());
    let heap_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });
    let biheap_point: Box<Box<Point>> = Box::new(heap_origin());

    // 复制 `heap_point` 中的数据（指针指向的数据）到 `dereferenced_heap_point` 中。
    let dereferenced_heap_point: Point = *heap_point;

    println!("各个变量在栈内存中占用的字节数：");
    println!("stack_point: {}", mem::size_of_val(&stack_point));
    println!("stack_rectangle: {}", mem::size_of_val(&stack_rectangle));
    println!("heap_point: {}", mem::size_of_val(&heap_point));
    println!("heap_rectangle: {}", mem::size_of_val(&heap_rectangle));
    println!("biheap_point: {}", mem::size_of_val(&biheap_point));
    println!(
        "dereferenced_heap_point: {}",
        mem::size_of_val(&dereferenced_heap_point)
    );
}
