fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_param(5, 10);

    let r = add(10, 20);
    println!("The result is {}", r);
    let r = add2(20, 20);
    println!("The result is {}", r);
    // 表达式
    let x = { 5 };
    let y = {
        let x = 3;
        x
    };
    let r = add(x, y);
    println!("The result is {}", r);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

//表达式不带分号，在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。使用 return 关键字和指定值，可从函数中提前返回
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn add2(x: i32, y: i32) -> i32 {
    return x + y;
}
