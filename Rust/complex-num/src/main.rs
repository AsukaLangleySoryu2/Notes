use num::complex::Complex;
fn main() {
    println!("Hello, world!");
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{}+{}i", result.re, result.im);
    //Rust语言圣经
    let _x = 2.0;
    let _y: f32 = 3.0;
    //ccdhar类型
    // let _c = 'z';
    let z = '国';
    println!("字符'国'占用了{}字节的内存大小",std::mem::size_of_val(&z));
    let _t=true;
    let f:bool= false;
    if !f{
        println!("Just Test!");
    }
    //()单元类型
    let a=8_i32;
    let _b=33_i32;
    // let _b:Vec<f64>=Vec::new();
    // let (_a,_c)=("h1",false);
    // let b=(let a=8_i32);
    // (a+8_i32)
    let a:i32;
    let y={
        let b:i32=100;
        a=33+b;
        a
    };
    println!("y:{}",y);
    //表达式不返回任何值，会隐式返回一个()
    assert_eq!({},());

    //函数  
    let x=plus_or_minus(5);
    assert_eq!(x,10);
    
    forever();


}
fn dead_end()->!{
    panic!("你已经到了穷途末路,崩溃吧!");
}
fn forever(){
    loop{
        dead_end();
    };
}
fn _add(x:u32, y:u32) ->u32{
    x+y
}
fn _clear(text: &mut String) -> () {
  *text = String::from("");
}
fn plus_or_minus(x:i32)->i32{
    if x>5{
        return x-5
    }
    x+5
}
//函数就是表达式
fn _add_with_extra(x:i32,y:i32)->i32{
    let x=x+1;
    let y=y+1;
    x+y
}