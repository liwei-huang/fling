use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            let _some_work = ();
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("帮助"),
            "stop" => break,
            cmd => println!("未知命令：{cmd:?}"),
        }
    }

    STOP.store(true, Relaxed);
    background_thread.join().unwrap();
}
