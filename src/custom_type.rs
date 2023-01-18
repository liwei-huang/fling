// 一、结构体。
// (1) 普通的结构体；(2) 单位结构体，没有字段；(3) 元组结构体。

struct StruckA {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct StruckB;

struct StruckC(i32, i32);

// 二、枚举。

enum WebEvent {
    PageLoad,
    PageUnload,
    // (1) 类似元组结构体。
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("页面已加载"),
        WebEvent::PageUnload => println!("页面未加载"),
        // (2) 从枚举中解构出 a_char。
        WebEvent::KeyPress(a_char) => println!("按下 {}", a_char),
        WebEvent::Paste(a_string) => println!("粘贴 {}", a_string),
        WebEvent::Click { x: rex, y } => {
            println!("在 x={}, y={} 处点击", rex, y);
        }
    }
}

// 三、类型别名。

#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Substract,
}

type Laconic = VeryVerboseEnumOfThingsToDoWithNumbers;

// impl 块里面的 Self 类型别名。
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
}

// 四、枚举的 use。

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

// 五、枚举的编号和转换。
// (1) 隐式编号，从零开始。
enum Number {
    Zero,
    One,
}

// (2) 显式编号。
enum Color {
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// 六、常量。
// (1) const: 不可改变的常量。
// (2) static: 带 'static 声明周期，有可能可变的变量。
// 他们都必须显式写上类型注解。
const THRESHOLD: i32 = 10;
static LANGUAGE: &str = "Rust";

fn main() {
    // (4) 实例化结构体。
    let strudel = StruckA { x: 10.3, y: 0.4 };
    let strud = StruckB;
    let stru = StruckC(24, 44);
    // (5) 访问结构体的字段。
    println!("{}, {}", strudel.x, strudel.y);
    println!("{}, {}", stru.0, stru.1);
    // (6) 更新字段。
    let strung = StruckA { y: 2.4, ..strudel };
    println!("{}, {}", strung.x, strung.y);
    // (7) 解构。
    let StruckA { x: rex, y } = strudel;
    let StruckC(j, k) = stru;
    println!("[debug] {:?}, {}, {}, {}, {}", strud, rex, y, j, k);

    // (3) 使用枚举。
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("My text.".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };

    let consume: [WebEvent; 5] = [load, unload, pressed, pasted, click];
    for i in consume {
        inspect(i);
    }

    // [debug] 类型别名。
    let ena = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let enb = Laconic::Substract;
    let enc = VeryVerboseEnumOfThingsToDoWithNumbers::run(
        &VeryVerboseEnumOfThingsToDoWithNumbers::Add,
        4,
        7,
    );
    println!("[debug] {:?}, {:?}, {}", ena, enb, enc);

    // [debug] 枚举的 use。
    use crate::Status::{Poor, Rich};
    // 直接使用，而不是 Status::Rich。
    let status = Rich;
    println!("{:?}, {:?}", status, Poor);

    // (3) 枚举转换为整数。
    println!("{}", Number::One as i32);
    println!("{:06x}", Color::Blue as i32);
    println!(
        "[debug] {}, {:06x}",
        Number::Zero as i32,
        Color::Green as i32
    );

    // [debug] 常量。
    println!("{}, {}", THRESHOLD, LANGUAGE);
}
