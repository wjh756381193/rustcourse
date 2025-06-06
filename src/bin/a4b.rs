// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

// 主题：使用 match 进行决策
//
// 程序需求：
// * 根据变量的值是否为 1、2、3 或其他数字，分别显示 "one"、"two"、"three" 或 "other"
//
// 注意事项：
// * 使用一个设置为任意整数的变量
// * 使用 match 表达式来决定显示哪条消息
// * 使用下划线 (_) 来匹配任何值
fn main() {

    let sint = 200;
    match sint {
        1=>{ println!("One"); }
        2=>{ println!("Two"); }
        3=>{ println!("Three"); }
        _=>{ println!("Others"); }
    }

}
