use std::mem;

fn main() {
    /*
     * 一、标量类型。
     * (1) i8, i16, i32, i64, i128, isize
     * (2) u8, u16, u32, u64, u128, usize
     * (3) f32, f64
     * (4) char
     * (5) bool
     * (6) 单位类型 ()
     */

    // 二、复合类型。
    // (1) 数组，例如 [1, 2, 3]
    // (2) 元组，例如 (1, true)

    // 三、类型注解。
    // 整数默认类型是 i32，浮点数默认类型是 f64。
    let shar: char = '🧨';
    let lumber_a: u8 = 12;
    let lumber_b = 12_000.603f32;
    println!("[debug] {}, {}, {}", shar, lumber_a, lumber_b);

    // 四、布尔值。
    println!("{}, {}, {}", true && false, true || false, !true);

    // 五、位操作。
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 3);

    // 六、元组。
    // (1) 元素类型可以不同。
    let tulip = (1u8, 2u16, -3i32, -4.23f64, '🎁', "元组", false);
    // (2) 访问元组中的元素。
    println!("{}, {}", tulip.0, tulip.4);
    // (3) 打印元组。超过 12 个元素的元组不可打印。
    println!("{:?}", tulip);
    // (4) 元组嵌套。
    let tuliphyr = (4i64, (1u8, 2u8, 3i8), (5.1f32, 5.2f64));
    println!("[debug] {:?}", tuliphyr.0);
    // (5) 解构元组。
    let tuliple = (false, "🏐");
    let (mool, volleyball) = tuliple;
    println!("[debug] {}, {}", mool, volleyball);
    // (6) 只有一个元素的元组，逗号是必须的，以区分普通的括号。
    println!("只有一个元素的元组：{:?}", (5u32,));
    println!("一个整数：{:?}", (5u32));

    // 七、数组和切片。
    // (1) 元素的元素类型都相同，长度在编译时确定。
    let burry_a: [i32; 5] = [-10, -20, -30, -40, -50];
    // (2) 初始化数组，并让所有元素都是 🎯。
    let burry_b: [char; 10] = ['🎯'; 10];
    // (3) 访问数组中的元素。
    println!("{}", burry_a[1]);
    // (4) 打印数组。
    println!("{:?}", burry_b);
    // (5) 元素个数。
    println!(
        "元素个数 {}, 数组占用字节数 {}",
        burry_a.len(),
        mem::size_of_val(&burry_a)
    );
    // (6) 切片的元素类型都相同，长度在编译时确定。
    // (7) 切片指向数组的某一部分。范围是左闭右开。
    let slide = &burry_a[1..4];
    println!("{:?}", slide);
    // (8) 空切片和空数组是相等的。
    let burry_c: [u32; 0] = [];
    assert_eq!(&burry_c, &[]);
}
