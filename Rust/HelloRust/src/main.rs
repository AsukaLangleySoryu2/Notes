// use ferris_says::say;
// use std::io::{stdout,BufWriter};

// fn greet_world(){
//     let sourthern_germany="DWD";
//     let chinese="世界，你好";
//     let english="Hello,World";
//     let regions=[sourthern_germany,chinese,english];
//     for region in regions.iter() {
//         println!("{}",&region);
//     }
// }
fn main() {
    // let stdout=stdout();
    // let message=String::from("Hello Eorld");
    // let width=message.chars().count();

    // let mut writer =BufWriter::new(stdout.lock());
    // say(message.as_bytes(),width,&mut writer).unwrap();
    // greet_world();
    //这一步就是字符串赋值
    //     let penguin_data = "\
    //    common name,length (cm)
    //    Little penguin,33
    //    Yellow-eyed penguin,65
    //    Fiordland penguin,60
    //    Invalid,data
    //    ";
    //这一步就是调用方法
    /* let records = penguin_data.lines();
     //这一步循环
    for (i, record) in records.enumerate() {
      if i == 0 || record.trim().len() == 0 {
        continue;
         }
         //声明一个fields类型的变量，该集合可伸缩
         let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();
      if cfg!(debug_assertions) {
          // 输出到标准错误输出
        eprintln!("debug: {:?} -> {:?}",
               record, fields);
      }

      let name = fields[0];
       if let Ok(length) = fields[1].parse::<f32>() {
          // 输出到标准输出
          println!("{}, {}cm", name, length);
      }
     } */
    /*  //所有权的概念
    let a = 10;
    let _e="test";
    let _g="我是大哥大";
    let b: i32 = 20;
    let mut c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d)); */

    //let变量解构
    // let (a, mut b): (bool, bool) = (true, true);
    // println!("a={:?}, b={:?}", a, b);
    // b = true;
    // assert_eq!(a, b);

    //解构式赋值
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    let z=5;
    println!("The value of z is:{}",z);
    //常量声明
    const MAX_POINTS:u32=100_000;
    println!("The const is {:?}",MAX_POINTS);

    //变量遮蔽,可以用来遮蔽全局不需要变量
    let y=5;
    let y=y+1;
    //内存对象的新分配
    {
      let y=y*2;
      println!("The value of y in the inner scope is:{}",y);
    }
    println!("The value of y in the inner scope is:{}",y);
    let spaces=" ";
    //变量遮蔽可以改变变量类型
    let spaces=spaces.len();
    //不过Java有很多值得学习的思想

  }
//这里没有分号
/* fn add(i: i32, j: i32) -> i32 {
    i + j
} */
struct Struct {
    e: i32,
}
