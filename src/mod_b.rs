// 寻找一个叫 mod_a.rs 的文件，让其公共的内容在这里可用。
mod mod_a;

fn main() {
    // 调用 mod_a.rs 中的 funch 函数。
    mod_a::funch();
    mod_a::main();
}
