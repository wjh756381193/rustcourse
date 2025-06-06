// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

// 使用while语句进行循环
//
// 程序要求：
// * 从5倒数到1，在终端显示倒计时
//   完成后打印 "done!"
//
// 注意：
// * 使用可变整数变量
// * 使用while语句
// * 在while循环内打印变量
// * 不要使用break退出循环

fn main() {
    let mut row = 1;
    while row<10 {
        let mut col = 1;
        while col<=row {
            print!("{}*{}={}\t",  col, row, row*col);
            col += 1;
        }
        row += 1;
        print!("\n");
        // println!();
    }
}
