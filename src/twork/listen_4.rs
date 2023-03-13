use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let a = AtomicI32::new(100);
    let b = a.fetch_add(23, Relaxed);
    let c = a.load(Relaxed);

    assert_eq!(b, 100);
    assert_eq!(c, 123);
}

#[allow(dead_code)]
fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = {
            let mut r = 0;
            for i in 0..10000 {
                r = i + 1;
            }
            r
        };
        X.store(x, Relaxed);
    }
    x
}
