use std::collections::HashSet;

fn main() {
    // `HashSet` 只存储键，等价 `HashMap<T, ()>`。
    // `HashSet` 保证没有重复的元素，每一个元素都是独一无二的。
    // `HashSet` 主要有下面四个操作，返回值都是一个迭代器。
    // (1) `union`，获取两个集合中的所有元素，并去除相同的元素。
    // (2) `difference`，获取在第一个集合中而不是在第二个集合中的元素。
    // (3) `intersection`，获取两个集合共有的元素。
    // (4) `symmetric_difference`，获取不是两个集合共有的元素。

    let mut asset: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut esset: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    // 插入不存在的值，返回 `true`，插入存在的值，返回 `false`。
    // 插入已经存在的值，新值将替换旧值，所以仍然只有一个值。
    if asset.insert(4) {
        println!("asset.insert(4) -> true");
    } else {
        println!("asset.insert(4) -> false");
    }

    if asset.contains(&4) {
        println!("asset 含有 4 这个值。");
    } else {
        println!("asset 不含有 4 这个值。");
    }

    esset.insert(5);

    println!("asset: {:?}", asset);
    println!("esset: {:?}", esset);

    println!("Union: {:?}", asset.union(&esset).collect::<Vec<&i32>>());
    println!(
        "Difference: {:?}",
        asset.difference(&esset).collect::<Vec<&i32>>()
    );
    println!(
        "Intersection: {:?}",
        asset.intersection(&esset).collect::<Vec<&i32>>()
    );
    println!(
        "Symmetric Difference: {:?}",
        asset.symmetric_difference(&esset).collect::<Vec<&i32>>()
    );
}
