use std::convert;
use std::mem;

#[derive(Debug)]
struct Lumber {
    value: i32,
}

impl convert::From<i32> for Lumber {
    fn from(item: i32) -> Self {
        Lumber { value: item }
    }
}

fn main() {
    // 一、类型转换。
    // 没有隐式类型转换（coercion），而是使用 as 实现显式类型转换（casting）。
    let decimal = 65.4321f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("{}f32 -> {}u8 -> {}(char)", decimal, integer, character);

    // 二、数字字面量后缀。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    // 默认推断为 i32 和 f64。
    let i = 1;
    let f = 1.0;
    println!(
        "[size] {}, {}, {}, {}, {}",
        mem::size_of_val(&x),
        mem::size_of_val(&y),
        mem::size_of_val(&z),
        mem::size_of_val(&i),
        mem::size_of_val(&f),
    );

    // 三、From 和 Into。
    let lube = Lumber::from(30);
    println!("{:?}", lube.value);

    // 四、块的返回值。
    let resume = {
        let x = 5u32;
        let square = x * x;
        let cube = square * x;
        square + cube
    };
    let resune = {
        let x = 5u32;
        let square = x * x;
        let _cube = square * x;
        // 返回值是空元组 ()。
    };
    println!("{}, {:?}", resume, resune);
}
