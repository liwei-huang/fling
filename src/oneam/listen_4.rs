use std::thread;

fn main() {
    static X: [u32; 3] = [10, 20, 30];
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));

    let y: &'static [u16; 3] = Box::leak(Box::new([1, 2, 3]));
    thread::spawn(move || dbg!(y));
    thread::spawn(move || dbg!(y));
}
