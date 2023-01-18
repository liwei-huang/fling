use std::fmt;

#[derive(Debug)]
struct Struck(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "名字：{}, 年龄：{}", self.name, self.age)
    }
}

struct Lick(Vec<i32>);

impl fmt::Display for Lick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        // 尝试 write!()，如果报错，返回错误，否则继续。
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    // 一、占位符。
    // 1. 只有实现了 fmt::Display 的类型，才能使用占位符输出。用户自定义的类型默认是没有实现 fmt::Display 的。
    // 2. std::fmt 中的两个 traits：
    // (1) fmt::Debug，使用 {:?} 标记。
    // (2) fmt::Display，使用 {} 标记。
    // (3) 所有类型都能得到 fmt::Debug 实现，而 fmt::Display 需要手动实现。
    println!("{} 个 😅", 3);
    println!("{0}, {2}, {0}, {1}", "A", "B", "C");
    println!(
        "{cuneus}, {burst}, {ablaze}",
        ablaze = -1,
        burst = -2,
        cuneus = -3
    );

    // 二、格式化输出。
    println!(
        "Base 10: {0}; Base 2: {0:b}; Base 8: {0:o}; Base 16: {0:x}",
        69420
    );
    println!("右对齐：{nimble:>4}", nimble = 6);
    println!("左对齐：{nimble:0<width$}", nimble = 6, width = 4);

    // 三、实现 fmt::Debug。
    println!("{:?}, {:?}", 14, Struck(15));

    let name = "保罗";
    let age = 17;
    let paul = Person { name, age };

    println!("[debug] {}, {}", paul.name, paul.age);
    println!("简陋的输出：{:?}", paul);
    println!("漂亮的输出：{:#?}", paul);

    // 四、实现 fmt::Display。
    println!("自定义输出：{}", paul);

    let cev = Lick(vec![100, 200, 300]);
    println!("{}", cev);
}
