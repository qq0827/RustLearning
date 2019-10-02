use std::io;

fn main() {

    base_type();

    char_type();

    compound_type();

    range_type();

    slice_type();
}

///基本类型
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
    println!("c1 as u8={}", c1 as u8);
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
        print!("{} ", item);
    }
    println!();
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


struct People {
    name: &'static str,
    gender: u32,
    id: u32,
}

impl People {
    fn new(name: &'static str, gender: u32, id: u32) -> Self {
        return  People(name,gender,id);
    }

    fn name(&self){
        println!("name={:?}",self.name);
    }
}