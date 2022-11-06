fn main() {
    println!("Hello, world!");
    //所有权，简单来说就是半自动管理堆上的数据
    //一个值只能拥有一个所有者
    //所有者离开作用域范围，丢弃值
    let mut _s = "Hello";
    let mut s = String::from("Hello");
    s.push_str(",World");
    println!("{}", s);
    //基本类型自动拷贝u
    // let _s2=s;//为了避免二次释放，通过移交转移权
    let _x: &str = "hello world";
    // let y=x;
    let s1 = s.clone(); //深拷贝
    println!("{}\n{}", s, s1);
    //浅拷贝；拷贝基本类型，因为大小可知，所以拷贝到栈中
    //函数传值也是适用的，基本类型传副本，堆类型采用使用权转移
    //引用与借用
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s1 = String::from("hello");
    //这里是将s1的地址传给了函数
    let len = calculate_length(&s1);
    //传递引用不会改变所有权,相当于指向变量的指针
    println!("The length of '{}' is {}.", s1, len);
    let mut s = String::from("hello");
    change(&mut s);
    let _reference_to_nothing=dangle();




}
//这个地方创建了一个内存，返回该地方的引用，但是该对象在作用域结束时销毁
fn dangle()->String{
    let s=String::from("hello");
    // &s;解决办法--不返回引用，将控制权移交
    s
}
fn change(some_string: &mut String) {
    some_string.push_str(",world");
}
//可变引用只能存在一个同时不与不可变兼容
//这个函数负责返回变量大小
//引用的作用域有效为最后使用的位置
fn calculate_length(s: &String) -> usize {
    s.len()
}
