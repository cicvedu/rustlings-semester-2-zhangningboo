// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.



#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];  // 栈上
    // 切片指向了原变量的相应位置处；具有指针和长度的结构，是一个胖指针
    let nice_slice = &a[1..4];  // 切片：左闭右开；&a[1..=4]; 左闭右闭

    assert_eq!([2, 3, 4], nice_slice)
}
