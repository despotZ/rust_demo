const MATH_PI: f64 = 3.1415926; //全局作用域下，只能声明常量，不能声明变量，常量一般用全大写，常量必须申明类型
fn main() {
    println!("The const PI is {}", MATH_PI);
    // mut标识使变量可以被更改
    let mut x = 5;
    println!("The value x is {}", x);
    x = 6;
    println!("The value x is changed to {}", x);

    // Shadowing
    // rust 的变量可以重复申明，下次申明的变量会把上次申明的变量隐藏
    let a = 5;
    println!("The value a is {}", a);
    let a = a + 5;
    println!("The value a is {}", a);
    let a = a * 2;
    println!("The value a is {}", a);
    // 隐藏与将变量标记为 mut 是有区别的。当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误。通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。
    // mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，但复用这个名字。
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {} spaces", spaces);
    // 元组
    let tup: (i32, f64, bool) = (500, 6.4, true);
    let (x, y, z) = tup;
    println!("x is {},y is {},z is {}", x, y, z);
    // 数组
    let a = [3; 5];
    let [x, y, z, _, _] = a;
    println!("x is {},y is {},z is {}", x, y, z);
}

/*  int
    i8,u8
    i16,u16,
    i32,u32,
    i64,u64,
    i128,u128
    isize,usize   isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的
*/

/*  float
    f64
    f32
*/

/*  bool
    true
    false
*/

/*  char  */
// fn char_demo() {
//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';
// }

/*
    tuple 元组 长度固定 含在圆括号中的逗号分隔的值列表
    let tup: (i32, f64, bool) = (500, 6.4, true);元组中的每个值的类型可以不一致
    可以解构,也可以使用点号（.）后跟值的索引来直接访问它们
    let (x, y, z) = tup;
    let five_hundred = x.0;
    let six_point_four = x.1;
    let truth = x.2;
*/

/*
    tuple 数组 长度固定 含在中括号中的逗号分隔的值列表
    let a = [1, 2, 3, 4, 5];数组中每个值的类型必须一致
    在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    如果要为每个元素创建包含相同值的数组，可以指定初始值，后跟分号，然后在方括号中指定数组的长度
    let a = [3; 5];
    let a = [3, 3, 3, 3, 3];
    可以解构,也可以使用[]的索引来直接访问它们
    let [x, y, z] = a;
    let first = a[0];
    let second = a[1];
*/
