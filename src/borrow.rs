// 传递对象，值传递（`T`）、引用传递（`&T`）或者可变引用传递（`&mut T`）。
// 引用总是指向合法的对象，对象存在引用时，不会被销毁。

// (1) 同一时间，可以有多个不可变借用。
// (2) 同一时间，只能有一个可变借用。
// (3) 不能同时存在可变借用和不可变借用，即它们不能交叉使用。

fn take_ownership(boxed: Box<i32>) {
    println!("取走 {} 的所有权，并销毁。", boxed);
}

fn borrow(borrowed: &i32) {
    println!("借用 {}", borrowed);
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_immut(book: &Book) {
    println!("不可变借用：{} {}", book.title, book.year);
}

fn borrow_mut(book: &mut Book) {
    book.year = 2023;
    println!("可变借用：{} {}", book.title, book.year);
}

fn main() {
    let heap_i32 = Box::new(5i32);
    let stack_i32 = 6i32;

    // 借用，没有拿走所有权，之后仍然可以再次借用。
    borrow(&heap_i32);
    borrow(&stack_i32);
    borrow(&heap_i32);
    borrow(&stack_i32);

    {
        // 指针，指向 `heap_i32` 的内容。
        let _ref_heap_i32: &i32 = &heap_i32;

        // 不能，因为后面的代码还要借用 `heap_i32`。
        // take_ownership(heap_i32);

        borrow(_ref_heap_i32);
    }

    take_ownership(heap_i32);

    let immut_book = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // `Book` 实现了 `Clone` 和 `Copy` 特征，下面是复制一份 `immut_book`，不是移动。
    let mut mut_book = immut_book;

    borrow_immut(&immut_book);
    borrow_immut(&mut_book);
    borrow_mut(&mut mut_book);

    // `ref` 用在左边，`&` 用在右边。
    let arch = '😄';
    let ref_arch1 = &arch;
    let ref ref_arch2 = arch;
    // 两种写法相等。
    println!("*ref_arch1 == *ref_arch2: {}", *ref_arch1 == *ref_arch2);

    // `ref` 和 `mut` 在解构中。
    let mut tulip = (Box::new(5u32), 3u32);

    {
        let (_, ref mut last) = tulip;
        *last = 2u32;
    }

    println!("tulip: {:?}", tulip);
}
