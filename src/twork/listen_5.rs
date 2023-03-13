use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let nump = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let peak_time = &AtomicU64::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    let _ = t * 25 + i;
                    thread::sleep(Duration::from_secs(1));
                    let time_taken = start.elapsed().as_micros() as u64;

                    nump.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    peak_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let peak_time = Duration::from_micros(peak_time.load(Relaxed));

            let n = nump.load(Relaxed);
            if n == 100 {
                break;
            }
            if n == 0 {
                println!("还没开始工作。");
            } else {
                println!(
                    "工作已完成 {}/100，平均 {:?}，峰值 {:?}",
                    n,
                    total_time / n as u32,
                    peak_time
                );
            }

            thread::sleep(Duration::from_secs(2));
        }
    });

    println!("全部完成！");
}
