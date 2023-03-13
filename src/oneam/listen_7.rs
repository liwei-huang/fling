use std::cell::Cell;
use std::marker::PhantomData;

fn main() {}

#[allow(dead_code)]
struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>,
}

#[allow(dead_code)]
struct Y {
    p: *mut i32,
}

unsafe impl Send for Y {}
unsafe impl Sync for Y {}
