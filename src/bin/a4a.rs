// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

// 主题：使用 match 进行条件判断
//
// 程序要求：
// * 根据变量的值显示 "it's true" 或 "it's false"
//
// 注意：
// * 使用一个值为 true 或 false 的变量
// * 使用 match 表达式来决定显示哪条消息
fn main() {
    let flag = false;
    match flag {
        true=>{ println!("it's true"); }
        false=>{ println!("it's false"); }
    }

}
