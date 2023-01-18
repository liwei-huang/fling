fn main() {
    // (1) let 绑定。
    let spring = String::from("变量绑定");
    println!("{}", spring);
    // (2) 未使用的变量，静默警告。
    let _unused_value = 23;
    // (3) 变量默认不可变，mut 变量可变。
    let mut wool = false;
    println!("{}", wool);
    wool = true;
    // (4) 这是一个块，是一个新的作用域。
    {
        // bloct 在块外不可用。
        let bloct = wool;
        println!("{}", bloct);
    }
    // (5) 允许影子变量。
    let spring = String::from("影子变量");
    let _spring = spring;
    // (6) 冻结变量，影子变量是不可变时，当前作用域的影子变量被冻结，即不可变，即使原始变量是可变的。
    let mut _attr = 32u16;
    {
        let _attr = 32i16;
    }
    // OK!
    _attr = 64u16;
}
