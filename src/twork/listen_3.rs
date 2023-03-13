use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn main() {
    let nump = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..10 {
                thread::sleep(Duration::from_secs(i / 2));
                nump.store((i + 1).try_into().unwrap(), Relaxed);
                main_thread.unpark();
            }
        });

        loop {
            let n = nump.load(Relaxed);
            if n == 10 {
                break;
            }
            println!("工作已完成 {}/10。", n);
            thread::park_timeout(Duration::from_secs(2));
        }
    });

    println!("全部完成！");
}
