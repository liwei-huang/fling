use std::thread;

fn main() {
    let lumber = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("长度：{}", lumber.len());
        });
        s.spawn(|| {
            for l in &lumber {
                println!("{}", l);
            }
        });
    });

    println!("lumber: {:?}", lumber);
}
