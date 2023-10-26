// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)
//
// Execute `rustlings hint primitive_types2` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // Characters (`char`)

    // String: 以 UTF8 编码存储
    // str: 实质是 u8 数组，存储 utf8
    // &str: “胖指针”
    // char: 
    // Vec<u8>: 可以存储任意类型
    // [u8]: 存储在堆上，“胖指针”
    // &[u8]: 
    // CString:  FFI接口使用，跨语言
    // CStr:
    // &CStr:

    // String: {ptr, len, cap}， 正常情况下，ptr 指向堆
    // &str: {ptr, len}


    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = 'X'; // Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
