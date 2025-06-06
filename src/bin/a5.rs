// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

// 主题：使用 loop 语句进行循环
//
// 程序要求：
// * 在终端中显示数字 "1" 到 "4"
//
// 注意事项：
// * 使用一个可变的整数变量
// * 使用 loop 循环语句
// * 在循环体内打印该变量
// * 使用 break 退出循环
fn main() {
    let mut row = 1;
    loop {
        if row==10 { break; }

        let mut col = 1;
        loop {
            if col>row { break; }
            print!("{}*{}={}\t",  col, row, row*col);
            col += 1;
        }
        println!();
        row += 1;
    }
}
