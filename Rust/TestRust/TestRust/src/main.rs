
fn main() {
    println!("Hello, world!你好，世界");
    //Rust基本类型
    //数值类型：有符号无符号、浮点、有理数
    //字符串
    //布尔
    //字符
    //单元
    /* let _guess:i32="42".parse().expect("Not a number!");
    //整数类型
    let _x=2.0;
    let _y:f32=3.0;
    // assert!(0.1+0.2==0.3);
    assert!((0.1_f64+0.2-0.3).abs()<0.00001);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2); */
    let x=(-42.0_f32).sqrt();
    // assert_eq!(x,x);
    if x.is_nan() {
        println!("未定义的数学行为");
    }
    // let x=32_u32;
    // let y=42_u8;
    // let _z=x+y;
    // 编译器会进行自动推导，给予twenty i32的类型
  let twenty = 20;
  // 类型标注
  let twenty_one: i32 = 21;
  // 通过类型后缀的方式进行类型标注：22是i32类型
  let twenty_two = 22_i32;

  // 只有同样类型，才能运算
  let addition = twenty + twenty_one + twenty_two;
  println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

  // 对于较长的数字，可以用_进行分割，提升可读性
  let one_million: i64 = 1_000_000;
  println!("{}", one_million.pow(2));

  // 定义一个f32数组，其中42.0会自动被推导为f32类型
  let forty_twos = [
    42.0,
    42f32,
    42.0_f32,
  ];

  // 打印数组中第一个值，并控制小数位为2位
  //这里:.的意思有待
  println!("{:.2}", forty_twos[0]);

  // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);

    //序列
    for i in 1..=5 {
        println!("{}",i);
    }
    for i in 'a'..='z'{
        println!("{}",i);
    }
}    