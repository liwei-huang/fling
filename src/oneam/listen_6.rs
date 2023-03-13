use std::cell::Cell;
use std::cell::RefCell;

fn main() {}

#[allow(dead_code)]
fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("可能发生。");
    }
}

#[allow(dead_code)]
fn g(c: &Cell<Vec<i32>>) {
    let mut d = c.take();
    d.push(1);
    c.set(d);
}

#[allow(dead_code)]
fn h(e: &RefCell<Vec<i32>>) {
    e.borrow_mut().push(1);
}
