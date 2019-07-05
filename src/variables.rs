fn main() {
    let x= 6;
    println!("the value od x is:{}",x);

    let mut x = 5; //let默认是immutable，如果要重新赋值，要加mut
    println!("the value od x is:{}",x);
    x = 6;             //数据类型不能变
    println!("the value od x is:{}",x);

//    let mut a ="   ";
//    a=a.len();               //报错
//    println!("the value of a is:{}",a)

    demo();
    demo2();
    demo3();

}

fn demo(){
    const MAX_POINTS: u32 = 100000; //这种类型逯如何都不会变，也不接受shadowing
    println!("the value of x is: {}",MAX_POINTS);
}

fn demo2(){
    let x=5;
    let x=x+1;
    let x=x*2;
    println!("the value of x is :{}",x);
}

fn demo3(){  //shadowing
    let a="   ";
    let a=a.len();    //进行了数据类型的转换
    println!("the value of a is:{}",a)
}












