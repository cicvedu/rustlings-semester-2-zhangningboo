// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let x = 10;  // 变量设置初始值
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }

    let a;  // 现在不用设置初值，因为a的值在下面肯定被初始化
    let b = 1;
    if b == 1 {
        a = 10;  // let a时，没有mut，因为本行代码是初次初始化a
    } else {
        a = 11;
    }
}
