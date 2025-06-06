// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

//主题：基本算术运算
// +, - , *, /, %
// +=, -= , *=, /=, %=
//
//项目要求：
// *显示两个数字的和的结果
//
// 注:
// *使用函数将两个数字相加
// *使用函数来显示结果
// *使用"{:?}"标记在println宏中显示结果

fn main() {
    let a = 1+2;
    let b = 9-8;
    let sab = sum(a, b);
    println!("Sum of a and b = {:?}", sab);
    let c = 7*3;
    let d = 7/2;
    let e = 5%2;
    let mut aa = 2;
    aa += a; // aa = aa + a;
    // x+= y ==> x = x+y;
    // x-= y ==> x = x-y;
    // x *= y ==> x = x*y;
    // x /= y ==> x = x/y;
    // x /= y ==> x = x/y;
    // x %= y ==> x = x%y;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", aa);
}

fn sum(a:i32, b:i32)->i32{
   a+b
}
