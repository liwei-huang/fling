#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// (1) `origin` 和 `new` 都是关联函数，类似静态方法。
impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// (2) `area` 和 `translate` 是方法，由实例来调用。
impl Rectangle {
    // `&self` 是 `self: &Self` 的简写。
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    // `&mut self` 是 `self: &mut Self` 的简写。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

fn main() {
    // (1) 函数有返回值。
    let ra = func_a(4, true);
    // (2) 函数默认的返回值是单位类型，即 `()`。
    let rb = func_b();

    println!("{}, {:?}", ra, rb);

    // (3) 关联函数。
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // (4) 方法。
    println!("{}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 改变自身的方法。
    square.translate(10.0, -10.0);
    println!("{:?}", square);
}

fn func_a(a: i32, b: bool) -> i32 {
    let c = if b { a * a } else { a ^ a };
    // 这是一个表达式，不是语句，表达式的值成为函数的返回值。
    c / 2
}

fn func_b() {
    println!("不返回任何值。");
}
