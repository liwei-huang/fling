use std::thread;

fn main() {
    let lumber = vec![1, 2, 3];
    thread::spawn(move || {
        for l in lumber {
            println!("{l}");
        }
    })
    .join()
    .unwrap();

    let plumb = Vec::from_iter(0..=1000);
    let atthread = thread::spawn(move || {
        let length = plumb.len();
        let sum = plumb.into_iter().sum::<usize>();
        sum / length
    });
    let average = atthread.join().unwrap();
    println!("平均值：{}", average);
}
