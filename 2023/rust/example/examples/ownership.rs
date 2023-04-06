fn main() {
    ownership()
}

/// 记住两条规则即可：
/// 一个变量只能有一个所有者
/// 当所有者（变量）离开作用域，这个变量将被丢弃
/// 这是rust的核心之一，所有权+生命周期 保证了内存的安全
/// 所有权避免了悬垂指针的问题，即不会有野指针
fn ownership() {
    // ownership
    let s1 = String::from("hello");
    let _s2 = s1; // s1的所有权被转移到s2
    // println!("{}", s1); // s1已经被转移，不能再使用,这行代码会报错

    // variable shadow
    let x = String::from("hello");
    let y = x;
    // 这个x和上面的x已经没有关系了，所有不会发生编译错误
    let x = String::from("hello");
    println!("x: {},y: {}", x, y);
}
