// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// 主题：使用枚举
//
// 程序需求：
// * 输出当前日期是否是工作日
//
// 注意事项：
// * 使用包含日期枚举，枚举值具体是那一天
// * 使用一个函数是否是工作日
// * 该函数必须将枚举作为参数
// * 该函数必须返回一个bool值
// * 使用 match 表达式来确定要返回的值

enum Week {
    Mon, Tue, Wed, Thu, Fri, Sat, Sun
}

fn is_weekday(wk: Week)->bool{
    match wk {
        Week::Sun | Week::Sat => false,
        _ => true,
    }
}

fn main() {
    let today = Week::Sat;

    if is_weekday(today) {
        println!("On Duty");
    }else{
        println!("Off Duty");
    }
}
