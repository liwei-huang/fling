use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());

    let c = Arc::new([1, 2, 3]);
    let d = c.clone();

    thread::spawn(move || dbg!(c));
    thread::spawn(move || dbg!(d));

    let e = Arc::new([1, 2, 3]);

    thread::spawn({
        let e = e.clone();
        move || {
            dbg!(e);
        }
    });

    thread::spawn(move || dbg!(e));
}
