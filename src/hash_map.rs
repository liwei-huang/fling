use std::collections::HashMap;

// 实现 `Eq` 和 `Hash` 特征。
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type AccountMap<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(account_map: &AccountMap<'a>, username: &'a str, password: &'a str) {
    println!("用户名：{}", username);
    println!("密码：{}", password);
    println!("尝试登录......");

    let logon = Account { username, password };

    match account_map.get(&logon) {
        Some(account_info) => {
            println!("登录成功！");
            println!("名字：{}", account_info.name);
            println!("邮箱：{}", account_info.email);
        }
        _ => println!("登陆失败。"),
    }
}

fn main() {
    // `HashMap` 存储键值对。
    // (1) 键可以是布尔值、整数、字符串等实现了 `Eq` 和 `Hash` 特征的数据类型。
    // (2) `HashMap` 的大小是可变的。
    // (3) 创建 `HashMap` 的两种方式。
    // (3.1) `HashMap::with_capacity(uint)`，创建容量为 `uint` 的 `HashMap`；
    // (3.2) `HashMap::new()`，创建初始容量的 `HashMap`。

    let mut contacts = HashMap::new();

    contacts.insert("张三", "798-1364");
    contacts.insert("李四", "645-7689");
    contacts.insert("王五", "435-8291");
    contacts.insert("赵六", "956-1745");

    // 取得引用，返回 `Option<&V>`。
    match contacts.get(&"王五") {
        Some(&tele) => println!("王五 {}", tele),
        _ => println!("没有王五的电话号码。"),
    }

    // `HashMap.insert()` 的结果是 `Option<T>`。
    // (1) 插入已经存在的键，更新值，结果是 `Some(value)`，`value` 是旧值。
    match contacts.insert("赵六", "164-6743") {
        Some(value) => println!("赵六的旧电话号码 {}", value),
        None => println!("已插入赵六的电话号码。"),
    }

    // (2) 插入新的键，结果是 `None`。
    match contacts.insert("孙七", "178-4435") {
        Some(value) => println!("孙七的旧电话号码 {}", value),
        None => println!("已插入孙七的电话号码。"),
    }

    println!("{:?}", contacts);

    contacts.remove(&"张三");
    contacts.remove(&"李四");

    // 迭代 `HashMap`，随机顺序，以 `(&'a key, &'a value)` 的形式。
    for (contact, &tele) in contacts.iter() {
        println!("{}, {}", contact, tele);
    }

    // 键可以是任何实现了 `Eq` 和 `Hash`特征的类型：
    // `bool`
    // `int`、`uint` 及变体
    // `String` 和 `&str`
    // 集合，集合中的内容实现了 `Eq` 和 `Hash` 特征
    // 等。

    let mut accounts: AccountMap = HashMap::new();
    let account = Account {
        username: "Liuii",
        password: "password123",
    };
    let account_info = AccountInfo {
        name: "刘一",
        email: "liuii@gmail.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "Liuii", "password123");
    try_logon(&accounts, "Liuii", "password123456");
}
