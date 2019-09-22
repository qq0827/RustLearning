fn main() {
  println!("hello rust");

  
  //条件分支
  let x = 6;
  if x == 5{
      println!("x==5 is ture");
  }
  //match rust没有switch关键字
  match x {
      1 =>{println!("one")},
      2 => println!("two"),
      5 => println!("five"),
      _=> println!("default")
  }

  //for循环
  for i in 0..10 {
      println!("{}",i);
  }

  //while loop 

  //函数调用
  let xm="hi---";
  let xy="hello---";
  say_what(xm,hi   );
  say_what(xy,hello);
  println!("add resutl={}.",add (10,20));

  let (p2,p3) = pow_2_3(10);
  println!("pow 2 of 10 is {}.", p2);
  println!("pow 3 of 10 is {}.", p3);
}

fn hi(name:&str){
    println!("hi,{}.",name);
}

fn hello(name:&str){
    println!("hello,{}",name);
}

//回调函数
fn say_what(name:&str,func:fn(&str)){
    func(name);
}

///带返回值的函数
fn add(x:i32,y:i32)->i32{
    x+y
}
///多返回值
fn pow_2_3(n:i32)->(i32,i32){
    (n*n,n*n*n)
}