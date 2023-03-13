use std::thread;

fn main() {
    let thread_one = thread::spawn(f);
    let thread_two = thread::spawn(f);

    println!("[main thread]: 你好");

    thread_one.join().unwrap();
    thread_two.join().unwrap();
}

fn f() {
    let thread_id = thread::current().id();
    println!("[other<{:?}> thread]: 你好", thread_id);

    for _x in 0..100 {
        let _ = _x + 1;
    }

    println!("计算后。");
}
