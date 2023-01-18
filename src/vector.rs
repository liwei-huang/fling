fn main() {
    // (1) 矢量是可重新分配大小的数组。
    // (2) 编译时，不知道矢量的大小，矢量的大小随时可能变化。
    // (3) 矢量使用 3 个参数表示：指向数据的指针、长度、容量。
    // (4) 长度比容量小，当容量不够时，会重新分配矢量，得到更大的容量。
    // (5) 超出边界的索引会引发编译器的恐慌。

    // `vec!` 宏初始化矢量。
    let mut avec = vec!['😅', '😂'];
    avec.push('🤣');
    println!("avec: {:?}", avec);

    // 把迭代器收集为矢量。
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("collected_iterator: {:?}", collected_iterator);

    // 当前矢量存储的元素个数。
    println!("avec 矢量的元素个数：{}", avec.len());

    // 索引。
    println!("collected_iterator[3]: {}", collected_iterator[3]);

    // 迭代。
    for c in avec.iter() {
        println!("> {}", c);
    }

    for (index, elm) in collected_iterator.iter().enumerate() {
        print!("位置[{index}]: {elm}, ");
    }

    // 可变迭代。
    let mut invec = vec![2u16, 4, 6];
    for elm in invec.iter_mut() {
        *elm *= 3;
    }
    println!("更新后的 invec: {:?}", invec);
}
