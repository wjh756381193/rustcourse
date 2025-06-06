// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

// 主题: 使用 if..else 控制程序流
//
// 要求:
// * 根据给定的变量值展示对应的信息
// * 变量为true时, 展示 "hello"
// * 变量为false时, 展示  "goodbye"
//
// 注:
// * 使用一个变量要么为 true 要么为 false
// * 使用 if..else 来决定要展示什么信息
// * 使用 println 展示信息

// 额外的训练
// 1. 用户分数=100分的时候，输出 “Well Done”
// 2. 用户分数大于60分的时候，输出 “Passed”,否则输出“Not Passed”
// 3. 用户分数大于90分的时候，输出 “Well Done”, 用户分数小于60 输出“Not Passed”
// 4. 用户分数大于90分的时候，输出 “A”, 分数大于80分输出“B”，大于60分输出"C"，否则“D”

fn main() {
    let score = 0;

    // if score == 100{
    //     println!("Well Done");
    // }

    // if score>=60 {
    //     println!("passed");
    // }else{
    //     println!("Not Passed");
    // }

    // if score>=90 {
    //     println!("Well Done");
    // }else if score<60 {
    //     println!("Not Passed");
    // }

    if score>=90 {
        println!("A");
    } else if score>=80 {
        println!("B");
    } else if score>=60 {
        println!("C");
    } else {
        println!("D");
    }

}
