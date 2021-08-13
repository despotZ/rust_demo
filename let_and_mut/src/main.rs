const MATH_PI: f64 = 3.1415926; //全局作用域下，只能声明常量，不能声明变量，常量一般用全大写，常量必须申明类型
fn main() {
    println!("The const PI is {}", MATH_PI);
    // mut标识使变量可以被更改
    let mut x = 5;
    println!("The value x is {}", x);
    x = 6;
    println!("The value x is changed to {}", x);
}
