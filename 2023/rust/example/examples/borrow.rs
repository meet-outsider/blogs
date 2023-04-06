fn main() {
    borrow();
    mut_borrow();
}

fn borrow() {
    let x = 5;
    let y = &x;
    println!("x = {}, y = {}", x, y);
}

fn mut_borrow() {
    let mut x = 5;//---------x声明为可变
    let z = &mut x;//----z为x的一个可变借用
    *z += 2;
    // println!("x = {}", x);//----如果这里x声明为不可变，那么之后打印z就会报错，因为z是可变借用，不符合借用的规则
    println!("z = {}", z);
    println!("x = {}", x);//-------这里x声明为不可变
    let y = &x;//--------------不可变借用
    println!("y = {}", y);
}