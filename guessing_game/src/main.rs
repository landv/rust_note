use std::{io, process};
use rand::Rng;
use std::cmp::Ordering;

// https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html
fn main() {
    println!("Guess the number!");

    // 生成秘密数字
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // 处理无效输入
        /***
        为了改善游戏性，不要在用户输入非数字时奔溃，需要忽略非数字输入，让用户可以继续猜测。
        可以通过修改 guess 将 String 转化为 u32 那部分代码来实现。

        不要问 `guess:u32`,问酒店parse()的标准写法
        https://doc.rust-lang.org/std/primitive.str.html#method.parse
        `let four: u32 = "4".parse().unwrap();`

        trim()返回去除了前导空格的字符串切片
        parse()如果 parse 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 Ok。
        这个 Ok 值与 match 第一个分支的模式相匹配，该分支对应的动作返回 Ok 值中的数字 num，最后如愿变成新创建的 guess 变量。
         */
        let guess:u32= match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        println!("You guessed:{}",guess);

        // 比较猜测的数字和秘密数字,猜测正确后退出
        /****
        cmp 方法用来比较两个值并可以在任何可比较的值上调用。它获取一个被比较值的引用：这里是把 guess 与 secret_number 做比较。
        然后它会返回一个刚才通过 use 引入作用域的 Ordering 枚举的成员。
        使用一个 match 表达式，根据对 guess 和 secret_number 调用 cmp 返回的 Ordering 成员来决定接下来做什么。
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // process::exit(1);
                break;
            }
        }
    }
}
