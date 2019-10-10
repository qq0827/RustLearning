use std::io;
use std::collections::HashMap;
use std::iter::Map;
use std::panic::resume_unwind;

fn main() {

    base_type();

    char_type();

    compound_type();

    range_type();

    slice_type();

    struct_test();

    enum_test();

    expression_test();
}

///# 基本类型
///
///# Examples
/// ```
/// bool u8 u16 u32 u64 u128
/// ```
fn base_type() {
    //基本类型
    let flag: bool = true;      //bool只有两个值true和false

    //无符号整型
    let x1: u8 = 56;            //u8类型,占用1个字节
    let x2: u16 = 110;          //u16类型,占用2个字节
    let x3: u32 = 1000;         //u32类型,占用4个字节
    let x4: u64 = 10000000;     //u64类型,占用8个字节
    let x5: u128 = 100000000;   //u128类型,占用16个字节

    //符号整数
    let x6: i8 = -56;             //i8类型,占用1个字节
    let x7: i16 = 110;            //i16类型,占用2个字节
    let x8: i32 = 1000;           //i32类型,占用4个字节
    let x9: i64 = 10000000;       //i64类型,占用8个字节
    let x10: i128 = 100000000;    //i128类型,占用16个字节

    //浮点数类型分f32和f64
    let f1: f32 = 3223.22;   //单精度位浮点数
    let f2: f64 = 10.0;     //双精度浮点数
    let f3 = 10.0;    //f3变量,类型推导为f64双精度浮点数
}

//字符类型
fn char_type() {
    let c1 = 'a';     //定义字符类型
    let c2 = '\'';    //字符转义其他语言一样,用\,两个\\输出一个\, 单引号出输出\'
    println!("{}", c1);
    println!("{}", c2);
    println!("c1 as u8={}", c1 as u8);  //通过as进行类型转换，将字符转为 u8类型
}

fn compound_type() {
    //数组
    let arr1: [i32; 3] = [0, 1, 2];      //声明一个长度为3的i32类型数组,这个用法稍微有点怪
    let arr2 = [0, 1, 2];        //一般都使用简写

    let last_val = arr1[2];        //访问数组下标为2的值
    println!("array arr[2]={}", last_val);

    //元组,在rust中是原生的类型,和csharp(c#)不一样
    let xx = (12, "hello rust", 110.1);  //声明一个类型有i32类型,字符串类型,f64类型的元组
    //使用下标获取元组的元素
    println!("index=0 val={}", xx.0);
    println!("index=1 val={}", xx.1);
    println!("index=2 val={}", xx.2);

    //使用多个变量按照顺序接受元组的元素值
    let (a, b, c) = xx;
    println!("a={}", a);
    println!("b={}", b);
    println!("c={}", c);
}

fn range_type() {
    let x = std::ops::Range { start: 1, end: 10 };  //声明一个Range左闭右开区间范围类型
    let y = (1..10);                    //左闭右开区间简写的方式
    assert_eq!(x, y);


    let x2 = std::ops::RangeInclusive::new(1, 10);  //全闭区间范围类型
    let y2 = (1..=10);                                        //全闭区间简写方式
    assert_eq!(x2, y2);
}

fn slice_type() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    //切片类型 通过引用符&对数组引用,产生切片,进行范围操作
    let lists = &arr[0..5];       //切片类型通过数组地址获取数组下标为0开始,5个元素,中间不产生新的数组

    assert_eq!(lists.len(), 5);           //验证lists的长度是否为5

    //遍历lists的元素,和arr前五个元素进行对比
    for item in lists {
        print!("{:?} ", item);
    }
    println!();
}

///# 输出结构体内容
fn struct_test() {
    let p = People {
        id: 1,
        name: "tom",
        sex: "男",
    };

    //如果在结构体定义的时候不加上#[derive(Debug)]注解，无法通过println!宏直接打印结构体内容
    //error[E0277]: `People` doesn't implement `std::fmt::Debug`
    //add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
    //u8 bool等类型可以通过println!打印，是因为这些类型默认实现Display，这个后面在说
    //下边没有{}而是改为{:?}输出结构体内容，{:#?}带缩进的输出
    println!("people = {:?}", p);
}

fn enum_test() {
    let status = WindowState::Max;
    match status {
        WindowState::Normal => println!("normal"),
        WindowState::Min => println!("min"),
        WindowState::Max => println!("max"),
    }
    //通过将枚举status转为u8类型,发现这点和其他语言一样
    println!("enum min val={}", WindowState::Min as u8);
    println!("enum normal val={}", WindowState::Normal as u8);
    println!("enum max val={}", WindowState::Max as u8);
}

fn map_test() {
    let mut map = HashMap::new();
    map.insert(1, "rust");
    map.insert(2, "go");
    map.insert(3, "csharp");

    println!("{:?}", map);

    for item in map.iter() {
        println!("{} {}", item.0, item.1);
    }
}

fn expression_test() {
//if语句和其他语言基本一样，只是没有条件没有小括号
let x = 100;
if x > 0 {
    println!("x >0");
} else {
    println!("x < 0")
}

//多重if语句
let y = 200;
if y > 10 {
    println!("level 1");
} else if y > 20 {
    println!("level 2");
} else if y > 30 {
    println!("level 3");
} else {
    println!("level 5");
}

//let和if组合使用
let num = if y > 100 {
    1
} else {
    0
};
println!("num = {}", num);

//loop 死循环,相当于其他语言的 while(true){ //执行代码} 或者 for(;;){ //执行代码 }
//下面代码进行注释，不然下面的代码无法执行
////    loop {
////        println!("loop out ....");
////    }

//loop和let组合使用
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
println!("result = {}", result);

//while
let mut a = 0;
while a < 10 {
    a += 1;
    println!("a = {}", a);
}

//for
for num in (1..100) {
    println!("for inc num = {}", num);
}

//match 在rust语言中，没有switch关键字,可以通过match实现相同的功能
//_ 相当于其他语言中default
let level = 2;
match level {
    1 => { println!("level 1") }
    2 => println!("level 2"),
    3 => println!("level 3"),
    4 => println!("level 4"),
    _ => println!("level 5")
}
}

fn main1() {


    //条件分支
    let x = 6;
    if x == 5 {
        println!("x==5 is ture");
    }
    //match rust没有switch关键字，这里相当于switch，不过更简洁
    match x {
        1 => { println!("one") }
        2 => println!("two"),
        5 => println!("five"),
        _ => println!("default")
    }

    //for循环
    for i in 0..10 {
        println!("{}", i);
    }

    //while loop

    //函数调用
    let xm = "hi---";
    let xy = "hello---";
    say_what(xm, hi);
    say_what(xy, hello);
    println!("add resutl={}.", add(10, 20));

    let (p2, p3) = pow_2_3(10);
    println!("pow 2 of 10 is {}.", p2);
    println!("pow 3 of 10 is {}.", p3);
}

fn hi(name: &str) {
    println!("hi,{}.", name);
}

fn hello(name: &str) {
    println!("hello,{}", name);
}

//回调函数
fn say_what(name: &str, func: fn(&str)) {
    func(name);
}

///带返回值的函数
fn add(x: i32, y: i32) -> i32 {
    x + y
}

///多返回值
fn pow_2_3(n: i32) -> (i32, i32) {
    (n * n, n * n * n)
}

///# 结构体
///# 定义结构体，通过#[derive(Debug)]注解，才能让println!打印结构体内容
#[derive(Debug)]
struct People {
    id: u32,
    //id
    name: &'static str,
    //姓名 字符串
    sex: &'static str,  //性别
}

#[derive(Debug)]
enum WindowState {
    Min,
    //最小化
    Normal,
    //正常
    Max,       //最大化
}
