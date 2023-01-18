mod outer_module {
    // (1) 模块中的物品默认是私有的。
    #[allow(dead_code)]
    fn func_a() {
        println!("outer_module::func_a()");
    }

    // (2) 使用 pub 变为公共的。
    pub fn func_b() {
        println!("outer_module::func_b()");
    }

    // (3) 模块嵌套。
    pub mod inner_module {
        // (4) 指定路径下可见。
        #[allow(dead_code)]
        pub(in crate::outer_module) fn func_c() {
            println!("outer_module::inner_module::func_c()");
        }

        // (5) 只在当前模块可见。
        #[allow(dead_code)]
        pub(self) fn func_d() {
            println!("outer_module::inner_module::func_d()");
        }

        // (6) 只在父模块可见。
        #[allow(dead_code)]
        pub(super) fn func_e() {
            println!("outer_module::inner_module::func_e()");
        }
    }

    // (7) 在当前 crate 可见。
    pub(crate) fn func_f() {
        println!("outer_module::func_f()");
    }

    // 结构体公共，字段公共。
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 结构体公共，字段私有。
    pub struct ClosedBox<T> {
        #[allow(dead_code)]
        contents: T,
    }

    // 构造器方法公共。
    impl<T> ClosedBox<T> {
        pub fn new(accontent: T) -> ClosedBox<T> {
            ClosedBox {
                contents: accontent,
            }
        }
    }
}

fn main() {
    outer_module::func_b();
    outer_module::func_f();

    // 公共字段，允许字段构造。
    let open_box = outer_module::OpenBox { contents: '🙋' };
    println!("{}", open_box.contents);

    // 私有字段，不允许字段构造。
    // let closed_box = outer_module::ClosedBox { contents: '🙋' };

    // 私有字段，但有公共构造器。
    let _closed_box = outer_module::ClosedBox::new('🙋');

    {
        // (8) 使用 use 让一个长路径中的物品可以直接使用。
        use outer_module::{func_b, func_f};
        func_b();
        func_f();

        // (9) 使用 use 重命名。
        use outer_module::func_b as funch;
        funch();
    }
}
